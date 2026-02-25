/**
 * This is a module for storing settings passed into KaTeX. It correctly handles
 * default settings.
 */

mod settings_types;

use crate::define::macros::public::MacroDefinition;
use crate::token::Token;
use crate::utils;
use wasm_bindgen::prelude::*;

use settings_types::{OutputType, StrictType};
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
pub use settings_types::TrustContext;
use crate::Namespace::Namespace;

/// Options to be passed to KaTeX.
///
/// Read <https://katex.org/docs/options.html> for more information.
///
#[non_exhaustive]
#[derive(Clone, Debug)]
#[wasm_bindgen(getter_with_clone)]
// #[wasm_bindgen]
pub struct Settings {
    /// Whether to render the math in the display mode.
    display_mode: bool,
    /// KaTeX output type.
    output: OutputType,
    /// Whether to have `\tags` rendered on the left instead of the right.
    leqno: bool,
    /// Whether to make display math flush left.
    fleqn: bool,
    /// Whether to let KaTeX throw a ParseError for invalid LaTeX.
    throw_on_error: bool,
    /// Color used for invalid LaTeX.
    error_color: String,
    /// Collection of custom macros.
    macros: crate::Namespace::MapRef<MacroDefinition>,
    /// Specifies a minimum thickness, in ems.
    min_rule_thickness: f64,
    color_is_text_color: bool,
    strict: StrictType,
    /// Whether to trust users' input.
    trust: bool,
    /// Max size for user-specified sizes.
    /// If set to `None`, users can make elements and spaces arbitrarily large.
    #[allow(clippy::option_option)]
    max_size: Option<f64>,
    /// Limit the number of macro expansions to the specified number.
    /// If set to `None`, the macro expander will try to fully expand as in LaTeX.
    #[allow(clippy::option_option)]
    max_expand: Option<i32>,

    global_group: bool,
}

impl Settings {
    pub fn get_ref_macros(&self) -> crate::Namespace::MapRef<MacroDefinition> {
        return self.macros.clone();
    }
}

#[wasm_bindgen]
impl Settings {
    #[wasm_bindgen(getter = displayMode)]
    pub fn get_display_mode(&self) -> bool {
        self.display_mode
    }

    #[wasm_bindgen(setter = displayMode)]
    pub fn set_display_mode(&mut self, display_mode: bool) {
        self.display_mode = display_mode
    }

    #[wasm_bindgen(getter = output)]
    pub fn get_output(&self) -> String {
        self.output.as_str().to_string()
    }

    #[wasm_bindgen(setter = output)]
    pub fn set_output(&mut self, output: String) {
        self.output = OutputType::from_str(output.as_str()).unwrap();
    }

    #[wasm_bindgen(getter = leqno)]
    pub fn get_leqno(&self)->bool{
        self.leqno
    }

    #[wasm_bindgen(setter = leqno)]
    pub fn set_leqno(&mut self, leqno:bool){
        self.leqno = leqno;
    }

    #[wasm_bindgen(getter = fleqn)]
    pub fn get_fleqn(&self)->bool{
        self.fleqn
    }

    #[wasm_bindgen(setter = fleqn)]
    pub fn set_fleqn(&mut self, fleqn:bool){
        self.fleqn = fleqn;
    }

    #[wasm_bindgen(getter = throwOnError)]
    pub fn get_throw_on_error(&self) -> bool {
        self.throw_on_error
    }

    #[wasm_bindgen(setter = throwOnError)]
    pub fn set_throw_on_error(&mut self, throw_on_error: bool) {
        self.throw_on_error = throw_on_error;
    }

    #[wasm_bindgen(getter = errorColor)]
    pub fn get_error_color(&self) -> String {
        self.error_color.clone()
    }

    #[wasm_bindgen(setter = errorColor)]
    pub fn set_error_color(&mut self, error_color: String) {
        self.error_color = error_color;
    }

    #[wasm_bindgen(getter = minRuleThickness)]
    pub fn get_min_rule_thickness(&self) -> f64 {
        self.min_rule_thickness
    }

    #[wasm_bindgen(setter = minRuleThickness)]
    pub fn set_min_rule_thickness(&mut self, min_rule_thickness: f64) {
        self.min_rule_thickness = min_rule_thickness;
    }

    #[wasm_bindgen(getter = colorIsTextColor)]
    pub fn get_color_is_text_color(&self) -> bool {
        self.color_is_text_color
    }

    #[wasm_bindgen(setter = colorIsTextColor)]
    pub fn set_color_is_text_color(&mut self, color_is_text_color: bool) {
        self.color_is_text_color = color_is_text_color;
    }

    #[wasm_bindgen(getter = strict)]
    pub fn get_strict(&self) -> String {
        self.strict.as_str().to_string()
    }

    #[wasm_bindgen(setter = strict)]
    pub fn set_strict(&mut self, strict: String) {
        //TODO 为了兼容katex，这里得传入 boolean|"ignore" | warn | "error" | StrictFunction 类型
        self.strict = StrictType::from_str(strict.as_str()).unwrap();
    }

    #[wasm_bindgen(getter = trust)]
    pub fn get_trust(&self) -> bool {
        self.trust
    }

    #[wasm_bindgen(setter = trust)]
    pub fn set_trust(&mut self, trust: bool) {
        self.trust = trust;
    }

    #[wasm_bindgen(getter = maxSize)]
    pub fn get_max_size(&self) -> Option<f64> {
        self.max_size
    }

    #[wasm_bindgen(setter = maxSize)]
    pub fn set_max_size(&mut self, max_size: Option<f64>) {
        self.max_size = max_size;
    }

    #[wasm_bindgen(getter = maxExpand)]
    pub fn get_max_expand(&self) -> Option<i32> {
        self.max_expand
    }

    #[wasm_bindgen(setter = maxExpand)]
    pub fn set_max_expand(&mut self, max_expand: Option<i32>) {
        self.max_expand = max_expand;
    }

    #[wasm_bindgen(getter = globalGroup)]
    pub fn get_global_group(&self) -> bool {
        self.color_is_text_color
    }

    #[wasm_bindgen(setter = globalGroup)]
    pub fn set_global_group(&mut self, global_group: bool) {
        self.global_group = global_group;
    }
}

impl AsRef<Settings> for Settings {
    fn as_ref(&self) -> &Settings {
        self
    }
}

#[wasm_bindgen]
impl Settings {
    #[wasm_bindgen(constructor)]
    pub fn new_from_js(js_v: &JsValue) -> Settings {
        let mut res = Settings::new();
        use js_sys::{Boolean, JsString, Reflect};
        if let Ok(opt_display_mode) = Reflect::get(&js_v, &JsString::from("displayMode")) {
            res.display_mode = opt_display_mode.as_bool().unwrap();
        }

        if let Ok(opt_output) = Reflect::get(&js_v, &JsString::from("output")) {
            if let Some(s) = opt_output.as_string() {
                res.output = OutputType::from_str(s.as_str()).unwrap();
            }
        }

        if let Ok(opt_leqno) = Reflect::get(&js_v, &JsString::from("leqno")) {
            res.leqno = opt_leqno.as_bool().unwrap_or(false);
        }
        if let Ok(opt_fleqn) = Reflect::get(&js_v, &JsString::from("fleqn")) {
            res.fleqn = opt_fleqn.as_bool().unwrap_or_default();
        }
        if let Ok(opt_throw_on_error) = Reflect::get(&js_v, &JsString::from("throwOnError")) {
            res.throw_on_error = opt_throw_on_error.as_bool().unwrap_or_default();
        }
        if let Ok(opt_error_color) = Reflect::get(&js_v, &JsString::from("errorColor")) {
            if let Some(c) = opt_error_color.as_string() {
                res.error_color = c;
            } else {
                res.error_color = String::from("#ff000");
            }
        } else {
            res.error_color = String::from("#ff000");
        }

        // if let Ok(opt_macros) = Reflect::get(&js_v, &JsString::from("macros")){}

        if let Ok(opt_min_rule_thickness) = Reflect::get(&js_v, &JsString::from("minRuleThickness"))
        {
            res.min_rule_thickness = opt_min_rule_thickness.as_f64().unwrap_or_default();
        }
        if let Ok(opt_color_is_text_color) =
        Reflect::get(&js_v, &JsString::from("colorIsTextColor"))
        {
            res.color_is_text_color = opt_color_is_text_color.as_bool().unwrap_or_default();
        }
        if let Ok(opt_strict) = Reflect::get(&js_v, &JsString::from("strict")) {
            res.strict = StrictType::from_str(opt_strict.as_string().unwrap_or_default().as_str())
                .unwrap_or_default();
        }
        if let Ok(opt_trust) = Reflect::get(&js_v, &JsString::from("trust")) {
            res.trust = opt_trust.as_bool().unwrap_or_default();
        }
        if let Ok(opt_max_size) = Reflect::get(&js_v, &JsString::from("maxSize")) {
            res.max_size = opt_max_size.as_f64();
        }
        if let Ok(opt_max_expand) = Reflect::get(&js_v, &JsString::from("maxExpand")) {
            res.max_expand = match opt_max_expand.as_f64() {
                Some(d) => Some(d as i32),
                None => None,
            }
        }
        if let Ok(opt_global_group) = Reflect::get(&js_v, &JsString::from("globalGroup")) {
            res.global_group = opt_global_group.as_bool().unwrap_or_default();
        }
        return res;
    }

    pub fn new() -> Settings {
        let mut res = Settings {
            display_mode: false,
            output: OutputType::Html,
            leqno: false,
            // Whether to make display math flush left.
            fleqn: false,
            // Whether to let KaTeX throw a ParseError for invalid LaTeX.
            throw_on_error: false,
            // Color used for invalid LaTeX.
            error_color: String::new(),
            // Collection of custom macros.
            macros: Arc::new(HashMap::<String, MacroDefinition>::new().into()),
            min_rule_thickness: 0.0,
            color_is_text_color: false,
            strict: StrictType::Warn,
            trust: false,
            max_size: None,
            max_expand: Some(1000),
            global_group: false,
        };
        res
    }
    /**
     * Report nonstrict (non-LaTeX-compatible) input.
     * Can safely not be called if `this.strict` is false in JavaScript.
     */
    pub fn report_nonstrict(&self, error_code: &str, error_msg: &str, token: Option<Token>) {
        match self.strict {
            StrictType::Ignore => {}
            StrictType::Warn => {
                println!(
                    "LaTeX-incompatible input and strict mode is set to 'warn': {} [{}]",
                    error_code,
                    error_msg
                );
            }
            StrictType::Error => {
                panic!("error lllll");
            }
        }
    }

    /**
     * Check whether to apply strict (LaTeX-adhering) behavior for unusual
     * input (like `\\`).  Unlike `nonstrict`, will not throw an error;
     * instead, "error" translates to a return value of `true`, while "ignore"
     * translates to a return value of `false`.  May still print a warning:
     * "warn" prints a warning and returns `false`.
     * This is for the second category of `errorCode`s listed in the README.
     */
    //TODO
    pub fn use_strict_behavior(&self, error_code: String, error_msg: String) -> bool {
        match self.strict {
            StrictType::Ignore => {
                return false;
            }
            StrictType::Warn => {
                println!(
                    "LaTeX-incompatible input and strict mode is set to 'warn': {} [{}]",
                    error_code,
                    error_msg
                );
                return false;
            }
            StrictType::Error => {
                panic!("error lllll");
                return true;
            }
        }
    }

    #[wasm_bindgen(js_name = "toJsValue")]
    pub fn to_js_value(&self) -> JsValue {
        use js_sys::{Object, Reflect, JsString};

        let obj = Object::new();

        Reflect::set(&obj, &JsString::from("displayMode"), &JsValue::from_bool(self.display_mode)).unwrap();
        Reflect::set(&obj, &JsString::from("output"), &JsValue::from_str(self.output.as_str())).unwrap();
        Reflect::set(&obj, &JsString::from("leqno"), &JsValue::from_bool(self.leqno)).unwrap();
        Reflect::set(&obj, &JsString::from("fleqn"), &JsValue::from_bool(self.fleqn)).unwrap();
        Reflect::set(&obj, &JsString::from("throwOnError"), &JsValue::from_bool(self.throw_on_error)).unwrap();
        Reflect::set(&obj, &JsString::from("errorColor"), &JsValue::from_str(&self.error_color)).unwrap();
        Reflect::set(&obj, &JsString::from("minRuleThickness"), &JsValue::from_f64(self.min_rule_thickness)).unwrap();
        Reflect::set(&obj, &JsString::from("colorIsTextColor"), &JsValue::from_bool(self.color_is_text_color)).unwrap();
        Reflect::set(&obj, &JsString::from("strict"), &JsValue::from_str(self.strict.as_str())).unwrap();
        Reflect::set(&obj, &JsString::from("trust"), &JsValue::from_bool(self.trust)).unwrap();

        match self.max_size {
            Some(v) => { Reflect::set(&obj, &JsString::from("maxSize"), &JsValue::from_f64(v)).unwrap(); }
            None => { Reflect::set(&obj, &JsString::from("maxSize"), &JsValue::NULL).unwrap(); }
        }

        match self.max_expand {
            Some(v) => { Reflect::set(&obj, &JsString::from("maxExpand"), &JsValue::from_f64(v as f64)).unwrap(); }
            None => { Reflect::set(&obj, &JsString::from("maxExpand"), &JsValue::NULL).unwrap(); }
        }

        Reflect::set(&obj, &JsString::from("globalGroup"), &JsValue::from_bool(self.global_group)).unwrap();

        obj.into()
    }
}
impl Settings{
    /**
     * Check whether to test potentially dangerous input, and return
     * `true` (trusted) or `false` (untrusted).  The sole argument `context`
     * should be an object with `command` field specifying the relevant LaTeX
     * command (as a string starting with `\`), and any other arguments, etc.
     * If `context` has a `url` field, a `protocol` field will automatically
     * get added by this function (changing the specified object).
     */
    // #[wasm_bindgen(js_name = isTrusted)]
    pub fn is_trusted(&self, context: &TrustContext) -> bool {
        // if (context.url && !context.protocol) {
        //     context.protocol = utils.protocolFromUrl(context.url);
        // }
        // const trust = typeof this.trust === "function"
        //     ? this.trust(context)
        //     : this.trust;
        // return Boolean(trust);
        return self.trust;
    }
}
