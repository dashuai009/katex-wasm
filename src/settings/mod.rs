mod settings_types;

use crate::define::macros::public::MacroDefinition;
use crate::token::Token;
/**
 * This is a module for storing settings passed into KaTeX. It correctly handles
 * default settings.
 */
use crate::utils;
use wasm_bindgen::prelude::*;

// Custom KaTeX behaviors.

// use crate::{error::Result, js_engine::JsScope};
// use derive_builder::Builder;
// use itertools::process_results;
use crate::settings::settings_types::{OutputType, StrictType};
use crate::utils::{console_log, log};
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;

/// Options to be passed to KaTeX.
///
/// Read <https://katex.org/docs/options.html> for more information.
///
#[non_exhaustive]
#[derive(Clone)]
#[wasm_bindgen(getter_with_clone)]
// #[wasm_bindgen]
pub struct Settings {
    /// Whether to render the math in the display mode.
    display_mode: bool,
    /// KaTeX output type.
    output: OutputType,
    /// Whether to have `\tags` rendered on the left instead of the right.
    pub leqno: bool,
    /// Whether to make display math flush left.
    pub fleqn: bool,
    /// Whether to let KaTeX throw a ParseError for invalid LaTeX.
    throw_on_error: bool,
    /// Color used for invalid LaTeX.
    error_color: String,
    /// Collection of custom macros.
    macros: Arc<HashMap<String, MacroDefinition>>,
    /// Specifies a minimum thickness, in ems.
    min_rule_thickness: f64,
    color_is_text_color: bool,
    strict: StrictType,
    /// Whether to trust users' input.
    pub trust: bool,
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
    pub fn get_ref_macros(&self) -> Arc<HashMap<String, MacroDefinition>> {
        return Arc::clone(&self.macros);
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

    #[wasm_bindgen(getter)]
    pub fn get_output(&self) -> String {
        self.output.as_str().to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_output(&mut self, output: String) {
        self.output = OutputType::from_str(output.as_str()).unwrap();
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

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "display_mode:{} output:{} leqno:{}",
            self.display_mode,
            self.output.as_str(),
            self.leqno
        )
    }
}

impl AsRef<Settings> for Settings {
    fn as_ref(&self) -> &Settings {
        self
    }
}

// impl OptsBuilder {
//     /// Add an entry to [`macros`](OptsBuilder::macros).
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// let opts = katex::Opts::builder()
//     ///     .add_macro(r#"\RR"#.to_owned(), r#"\mathbb{R}"#.to_owned())
//     ///     .build()
//     ///     .unwrap();
//     /// let html = katex::render_with_opts(r#"\RR"#, &opts).unwrap();
//     /// ```
//     pub fn add_macro(mut self, entry_name: String, entry_data: String) -> Self {
//         match self.macros.as_mut() {
//             Some(macros) => {
//                 macros.insert(entry_name, entry_data);
//             }
//             None => {
//                 let mut macros = HashMap::new();
//                 macros.insert(entry_name, entry_data);
//                 self.macros = Some(macros);
//             }
//         }
//         self
//     }
// }

#[wasm_bindgen]
impl Settings {
    #[wasm_bindgen(constructor)]
    pub fn new_from_js(js_v: &JsValue) -> Settings {
        // console_log!("input setting = {}",res);
        let mut res = Settings {
            display_mode: false,
            output: OutputType::Html,
            leqno: false,
            /// Whether to make display math flush left.
            fleqn: false,
            /// Whether to let KaTeX throw a ParseError for invalid LaTeX.
            throw_on_error: false,
            /// Color used for invalid LaTeX.
            error_color: String::new(),
            /// Collection of custom macros.
            macros: Arc::new(HashMap::<String, MacroDefinition>::new()),
            min_rule_thickness: 0.0,
            color_is_text_color: false,
            strict: StrictType::Warn,
            trust: false,
            max_size: None,
            max_expand: None,
            global_group: false,
        };
        use js_sys::{Boolean, JsString, Reflect};
        if let Ok(opt_display_mode) = Reflect::get(&js_v, &JsString::from("displayMode")) {
            res.display_mode = opt_display_mode.as_bool().unwrap();
        }
        console_log!("{}", res.display_mode);

        if let Ok(opt_output) = Reflect::get(&js_v, &JsString::from("output")) {
            if let Some(s) = opt_output.as_string() {
                console_log!("ouput raw str = {}", s);
                res.output = OutputType::from_str(s.as_str()).unwrap();
            }
        }
        console_log!("{}", res.output.as_str());

        if let Ok(opt_leqno) = Reflect::get(&js_v, &JsString::from("leqno")) {
            res.leqno = opt_leqno.as_bool().unwrap_or(false);
        }
        console_log!("{}", res.leqno);
        if let Ok(opt_fleqn) = Reflect::get(&js_v, &JsString::from("fleqn")) {
            res.fleqn = opt_fleqn.as_bool().unwrap_or_default();
        }
        console_log!("{}", res.fleqn);
        if let Ok(opt_throw_on_error) = Reflect::get(&js_v, &JsString::from("throwOnError")) {
            res.throw_on_error = opt_throw_on_error.as_bool().unwrap_or_default();
        }
        console_log!("{}", res.throw_on_error);
        if let Ok(opt_error_color) = Reflect::get(&js_v, &JsString::from("errorColor")) {
            if let Some(c) = opt_error_color.as_string() {
                res.error_color = c;
            } else {
                res.error_color = String::from("#ff000");
            }
        } else {
            res.error_color = String::from("#ff000");
        }
        console_log!("{}", res.error_color);

        // if let Ok(opt_macros) = Reflect::get(&js_v, &JsString::from("macros")){}

        if let Ok(opt_min_rule_thickness) = Reflect::get(&js_v, &JsString::from("minRuleThickness"))
        {
            res.min_rule_thickness = opt_min_rule_thickness.as_f64().unwrap_or_default();
        }
        console_log!("{}", res.min_rule_thickness);
        if let Ok(opt_color_is_text_color) =
            Reflect::get(&js_v, &JsString::from("colorIsTextColor"))
        {
            res.color_is_text_color = opt_color_is_text_color.as_bool().unwrap_or_default();
        }
        console_log!("{}", res.color_is_text_color);
        if let Ok(opt_strict) = Reflect::get(&js_v, &JsString::from("strict")) {
            res.strict = StrictType::from_str(opt_strict.as_string().unwrap_or_default().as_str())
                .unwrap_or_default();

            // console_log!("{}", res.strict);
        }
        if let Ok(opt_trust) = Reflect::get(&js_v, &JsString::from("trust")) {
            res.trust = opt_trust.as_bool().unwrap_or_default();
        }
        console_log!("{}", res.trust);
        if let Ok(opt_max_size) = Reflect::get(&js_v, &JsString::from("maxSize")) {
            res.max_size = opt_max_size.as_f64();
            // console_log!("{}", res.max_size.u);
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
        console_log!("{}", res.global_group);
        return res;
    }

    pub fn new() -> Settings {
        let mut res = Settings {
            display_mode: false,
            output: OutputType::Html,
            leqno: false,
            /// Whether to make display math flush left.
            fleqn: false,
            /// Whether to let KaTeX throw a ParseError for invalid LaTeX.
            throw_on_error: false,
            /// Color used for invalid LaTeX.
            error_color: String::new(),
            /// Collection of custom macros.
            macros: Arc::new(HashMap::<String, MacroDefinition>::new()),
            min_rule_thickness: 0.0,
            color_is_text_color: false,
            strict: StrictType::Warn,
            trust: false,
            max_size: None,
            max_expand: None,
            global_group: false,
        };
        res
    }
    /**
     * Report nonstrict (non-LaTeX-compatible) input.
     * Can safely not be called if `this.strict` is false in JavaScript.
     */
    pub fn report_nonstrict(&self, errorCode: String, errorMsg: String, token: Option<Token>) {
        match self.strict {
            StrictType::Ignore => {}
            StrictType::Warn => {
                console_log!(
                    "LaTeX-incompatible input and strict mode is set to 'warn': {} [{}]",
                    errorCode,
                    errorMsg
                );
            }
            StrictType::Error => {
                console_log!("error lllll");
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
    pub fn useStrictBehavior(&self, errorCode: String, errorMsg: String, token: &JsValue) -> bool {
        match self.strict {
            StrictType::Ignore => {
                return false;
            }
            StrictType::Warn => {
                console_log!(
                    "LaTeX-incompatible input and strict mode is set to 'warn': {} [{}]",
                    errorCode,
                    errorMsg
                );
                return false;
            }
            StrictType::Error => {
                console_log!("error lllll");
                return true;
            }
        }
    }

    /**
     * Check whether to test potentially dangerous input, and return
     * `true` (trusted) or `false` (untrusted).  The sole argument `context`
     * should be an object with `command` field specifying the relevant LaTeX
     * command (as a string starting with `\`), and any other arguments, etc.
     * If `context` has a `url` field, a `protocol` field will automatically
     * get added by this function (changing the specified object).
     */
    #[wasm_bindgen(js_name = isTrusted)]
    pub fn is_trusted(&self, context: &JsValue) -> bool {
        web_sys::console::log_1(context);
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
