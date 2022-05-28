/**
 * Describes spaces between different classes of atoms.
 */
use super::units::Measurement;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// Making the type below exact with all optional fields doesn't work due to
// - https://github.com/facebook/flow/issues/4582
// - https://github.com/facebook/flow/issues/5688
// However, since *all* fields are optional, $Shape<> works as suggested in 5688
// above.

pub type __spacings = HashMap<String, HashMap<String, Measurement>>;

lazy_static! {
    pub static  ref thinspace: Mutex<Measurement> = Mutex::new(Measurement{
        number: 3.0,
        unit: String::from("mu")
    });
    static ref mediumspace: Mutex<Measurement> = Mutex::new(Measurement{
        number: 4.0,
        unit:  String::from("mu")
    });
    static ref thickspace: Mutex<Measurement> = Mutex::new(Measurement{
        number: 5.0,
        unit:  String::from("mu")
    });

// Spacing relationships for display and text styles

    static ref spacings: __spacings= {
        let mut m = HashMap::new();
        let mut mord = HashMap::new();
        let thinspace_c = thinspace.lock().unwrap();
        let mediumspace_c = mediumspace.lock().unwrap();
        let thickspace_c = thickspace.lock().unwrap();
        mord.insert(String::from("mop"), thinspace_c.clone());
        mord.insert(String::from("mbin"),mediumspace_c.clone());
        mord.insert(String::from("mrel"),thickspace_c.clone());
        mord.insert(String::from("minner"),thinspace_c.clone());
        m.insert(String::from("mord"),mord);

        let mut mop = HashMap::new();
        mop.insert(String::from("mord"), thinspace_c.clone());
        mop.insert(String::from("mop"), thinspace_c.clone());
        mop.insert(String::from("mrel"), thickspace_c.clone());
        mop.insert(String::from("minner"), thinspace_c.clone());
        m.insert(String::from("mop"),mop);

        let mut mbin = HashMap::new();
        mbin.insert(String::from("mord"), mediumspace_c.clone());
        mbin.insert(String::from("mop"), mediumspace_c.clone());
        mbin.insert(String::from("mopen"), mediumspace_c.clone());
        mbin.insert(String::from("minner"), mediumspace_c.clone());
        m.insert(String::from("mbin"),mbin);


        let mut mrel = HashMap::new();
        mrel.insert(String::from("mord"), thickspace_c.clone());
        mrel.insert(String::from("mop"), thickspace_c.clone());
        mrel.insert(String::from("mopen"), thickspace_c.clone());
        mrel.insert(String::from("minner"), thickspace_c.clone());
        m.insert(String::from("mrel"),mrel);

        let mut mclose = HashMap::new();
        mclose.insert(String::from("mop"), thinspace_c.clone());
        mclose.insert(String::from("mbin"), mediumspace_c.clone());
        mclose.insert(String::from("mrel"), thickspace_c.clone());
        mclose.insert(String::from("minner"), thinspace_c.clone());
        m.insert(String::from("mclose"),mclose);

        let mut mpunct  = HashMap::new();
        mpunct.insert(String::from("mord"), thinspace_c.clone());
        mpunct.insert(String::from("mop"), thinspace_c.clone());
        mpunct.insert(String::from("mrel"), thickspace_c.clone());
        mpunct.insert(String::from("mopen"), thinspace_c.clone());
        mpunct.insert(String::from("mclose"), thinspace_c.clone());
        mpunct.insert(String::from("mpunct"), thinspace_c.clone());
        mpunct.insert(String::from("minner"), thinspace_c.clone());
        m.insert(String::from("mpunct"),mpunct);

        let mut minner= HashMap::new();
        minner.insert(String::from("mord"), thinspace_c.clone());
        minner.insert(String::from("mop"), thinspace_c.clone());
        minner.insert(String::from("mbin"), mediumspace_c.clone());
        minner.insert(String::from("mrel"), thickspace_c.clone());
        minner.insert(String::from("mopen"), thinspace_c.clone());
        minner.insert(String::from("mpunct"), thinspace_c.clone());
        minner.insert(String::from("minner"), thinspace_c.clone());
        m.insert(String::from("minner"),minner);
        return m;
    };
    static ref tightSpacings: __spacings = {
        let thinspace_c = thinspace.lock().unwrap();
        let mediumspace_c = mediumspace.lock().unwrap();
        let thickspace_c = thickspace.lock().unwrap();
        let mut m = HashMap::new();
        let   mut mord = HashMap::new();
        mord.insert(String::from("mop"), thinspace_c.clone());
        m.insert(String::from("mord"),mord);

        let mut mop = HashMap::new();
        mop.insert(String::from("mord"), thinspace_c.clone());
        mop.insert(String::from("mop"),thinspace_c.clone());
        m.insert(String::from("mop"),mop);

        let mut mclose= HashMap::new();
        mclose.insert(String::from("mop"),thinspace_c.clone());
        m.insert(String::from("mclose"), mclose);

        let mut minner = HashMap::new();
        minner.insert(String::from("mop"),thinspace_c.clone());
        m.insert(String::from("minner"),minner);
        m
    };
// Spacing relationships for script and scriptscript styles

}

#[wasm_bindgen]
pub fn get_spacings(k1: String, k2: String) -> JsValue {
    let res = spacings.get(&k1);

    match res {
        Some(p) => match p.get(&k2) {
            Some(q) => {
                JsValue::from_serde(q).unwrap()
            }
            None => JsValue::NULL,
        },
        None => JsValue::NULL,
    }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules!  console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

use std::panic;

#[wasm_bindgen]
pub fn get_tightSpacings(k1: String, k2: String) -> JsValue {
    let res = tightSpacings.get(&k1);
    let res = spacings.get(&k1);

    match res {
        Some(p) => match p.get(&k2) {
            Some(q) => {
                JsValue::from_serde(q).unwrap()
            }
            None => JsValue::NULL,
        },
        None => JsValue::NULL,
    }
}
