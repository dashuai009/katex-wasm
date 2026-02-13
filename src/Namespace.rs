use std::cell::RefCell;
use crate::parse_error::ParseError;
use crate::utils::{console_log, log};
use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use crate::parse_node::types::vcenter;

/**
 * A `Namespace` refers to a space of nameable things like macros or lengths,
 * which can be `set` either globally or local to a nested group, using an
 * undo stack similar to how TeX implements this functionality.
 * Performance-wise, `get` and local `set` take constant time, while global
 * `set` takes time proportional to the depth of group nesting.
 */

pub type Mapping<T> = std::collections::HashMap<String, T>;
pub type MapRef<T> = Arc<RefCell<Mapping<T>>>;
#[derive(Debug)]
pub struct Namespace<Value> {
    current: MapRef<Value>,
    builtins: MapRef<Value>,
    undef_stack: Vec<MapRef<Option<Value>>>,
}

impl<Value: Clone> Namespace<Value> {
    /**
     * Both arguments are optional.  The first argument is an object of
     * built-in mappings which never change.  The second argument is an object
     * of initial (global-level) mappings, which will constantly change
     * according to any global/top-level `set`s done.
     */
    pub fn new<'a>(
        builtins: MapRef<Value>,
        global_macros: MapRef<Value>,
    ) -> Namespace<Value> {
        Namespace {
            current: global_macros,
            builtins,
            undef_stack: Vec::new(),
        }
    }

    /**
     * Start a new nested group, affecting future local `set`s.
     */
    pub fn begin_group(&mut self) {
        self.undef_stack.push(RefCell::new(HashMap::new()).into());
    }

    /**
     * End current nested group, restoring values before the group began.
     */
    pub fn end_group(&mut self) {
        if self.undef_stack.len() == 0 {
            console_log!("Unbalanced namespace destruction: attempt to pop global namespace; please report this as a bug");
            return;
        }
        let stack = self
            .undef_stack
            .pop() /*.expect("").lock()*/
            .unwrap();
        for (k, v) in stack.borrow().iter() {
            //let mut cur = Arc::get_mut(&mut self.current).unwrap(); //.lock().unwrap();
            if v.is_none() {
                self.current.borrow_mut().remove(k);
            } else {
                self.current.borrow_mut().insert(k.to_string(), v.clone().unwrap());
            }
        }
    }

    /**
     * Ends all currently nested groups (if any), restoring values before the
     * groups began.  Useful in case of an error in the middle of parsing.
     */
    pub fn end_all_groups(&mut self) {
        while self.undef_stack.len() > 0 {
            self.end_group();
        }
    }

    /**
     * Detect whether `name` has a definition.  Equivalent to
     * `get(name) != null`.
     */
    pub fn has(&self, name: &str) -> bool {
        // let cur = self.current.lock().unwrap();
        // let buil = self.builtins.lock().unwrap();
        // return cur.contains_key(name) ||
        //     buil.contains_key(name);
        return self.current.borrow().contains_key(name) || self.builtins.borrow().contains_key(name);
    }

    /**
     * Get the current value of a name, or `undefined` if there is no value.
     *
     * Note: Do not use `if (namespace.get(...))` to detect whether a macro
     * is defined, as the definition may be the empty string which evaluates
     * to `false` in JavaScript.  Use `if (namespace.get(...) != null)` or
     * `if (namespace.has(...))`.
     */
    pub fn get(&self, name: &str) -> Option<Value> {
        let cur = self.current.borrow();
        match cur.get(name) {
            None => {
                match self.builtins.borrow().get(name) {
                    Some(vv)=>{
                        Some(vv.clone())
                    },
                    None =>{
                        None
                    }
                }
            }
            Some(v) => {Some(v.clone())}
        }
    }

    /**
     * Set the current value of a name, and optionally set it globally too.
     * Local set() sets the current value and (when appropriate) adds an undo
     * operation to the undo stack.  Global set() may change the undo
     * operation at every level, so takes time linear in their number.
     * A value of undefined means to delete existing definitions.
     */
    pub fn set(&mut self, name: &String, value: Option<Value>, global: bool) {
        if global {
            // Global set is equivalent to setting in all groups.  Simulate this
            // by destroying any undos currently scheduled for this name,
            // and adding an undo with the *new* value (in case it later gets
            // locally reset within this environment).
            for stack in self.undef_stack.iter_mut() {
                stack.borrow_mut().remove(name);
            }
            if self.undef_stack.len() > 0 {
                self.undef_stack
                    .last_mut()
                    .unwrap()
                    .borrow_mut()
                    .insert(name.clone(), value.clone());
            }
        } else {
            // Undo this set at end of this group (possibly to `undefined`),
            // unless an undo is already in place, in which case that older
            // value is the correct one.
            if let Some(top) = self.undef_stack.last_mut() {
                let mut top_mut = top.borrow_mut();
                if top_mut.contains_key(name) {
                    top_mut.insert(
                        name.clone(),
                        Some((*self.current.borrow().get(name).unwrap()).clone()),
                    );
                }
            }
        }

        let  mut c = self.current.borrow_mut();
        if value.is_none() {
            c.remove(name);
        } else {
            c.insert(name.clone(), value.unwrap());
        }
    }
}
