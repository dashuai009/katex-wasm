mod settings_types;

use wasm_bindgen::prelude::*;

use crate::token::Token;
/**
 * This is a module for storing settings passed into KaTeX. It correctly handles
 * default settings.
 */
use crate::utils;

// Custom KaTeX behaviors.

// use crate::{error::Result, js_engine::JsScope};
// use derive_builder::Builder;
// use itertools::process_results;
use crate::settings::settings_types::{OutputType, StrictType};
use crate::utils::{console_log, log};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// Options to be passed to KaTeX.
///
/// Read <https://katex.org/docs/options.html> for more information.
///
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    macros: HashMap<String, String>,
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

#[wasm_bindgen]
impl Settings {
    #[wasm_bindgen(getter = displayMode)]
    pub fn display_mode(&self) -> bool {
        self.display_mode
    }

    #[wasm_bindgen(setter = displayMode)]
    pub fn set_display_mode(&mut self, display_mode: bool) {
        self.display_mode = display_mode
    }

    #[wasm_bindgen(getter)]
    pub fn output(&self) -> String {
        self.output.as_str().to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_output(&mut self, output: String) {
        self.output = OutputType::from_str(output.as_str()).unwrap();
    }

    #[wasm_bindgen(getter = throwOnError)]
    pub fn throw_on_error(&self) -> bool {
        self.throw_on_error
    }

    #[wasm_bindgen(setter = throwOnError)]
    pub fn set_throw_on_error(&mut self, throw_on_error: bool) {
        self.throw_on_error = throw_on_error;
    }

    #[wasm_bindgen(getter = errorColor)]
    pub fn error_color(&self) -> String {
        self.error_color.clone()
    }

    #[wasm_bindgen(setter = errorColor)]
    pub fn set_error_color(&mut self, error_color: String) {
        self.error_color = error_color;
    }

    #[wasm_bindgen(getter = minRuleThickness)]
    pub fn min_rule_thickness(&self) -> f64 {
        self.min_rule_thickness
    }

    #[wasm_bindgen(setter = minRuleThickness)]
    pub fn set_min_rule_thickness(&mut self, min_rule_thickness: f64) {
        self.min_rule_thickness = min_rule_thickness;
    }

    #[wasm_bindgen(getter = colorIsTextColor)]
    pub fn color_is_text_color(&self) -> bool {
        self.color_is_text_color
    }

    #[wasm_bindgen(setter = colorIsTextColor)]
    pub fn set_color_is_text_color(&mut self, color_is_text_color: bool) {
        self.color_is_text_color = color_is_text_color;
    }

    #[wasm_bindgen(getter = strict)]
    pub fn strict(&self) -> String {
        self.strict.as_str().to_string()
    }

    #[wasm_bindgen(setter = strict)]
    pub fn set_strict(&mut self, strict: String) {
        //TODO 为了兼容katex，这里得传入 boolean|"ignore" | warn | "error" | StrictFunction 类型
        self.strict = StrictType::from_str(strict.as_str()).unwrap();
    }

    #[wasm_bindgen(getter = maxSize)]
    pub fn max_size(&self) -> Option<f64> {
        self.max_size
    }

    #[wasm_bindgen(setter = maxSize)]
    pub fn set_max_size(&mut self, max_size: Option<f64>) {
        self.max_size = max_size;
    }

    #[wasm_bindgen(getter = maxExpand)]
    pub fn max_expand(&self) -> Option<i32> {
        self.max_expand
    }

    #[wasm_bindgen(setter = maxExpand)]
    pub fn set_max_expand(&mut self, max_expand: Option<i32>) {
        self.max_expand = max_expand;
    }

    #[wasm_bindgen(getter = globalGroup)]
    pub fn global_group(&self) -> bool {
        self.color_is_text_color
    }

    #[wasm_bindgen(setter = globalGroup)]
    pub fn set_global_group(&mut self, global_group: bool) {
        self.global_group = global_group;
    }
}

#[wasm_bindgen]
impl Settings {
    pub(crate) fn to_js_value(&self) -> JsValue {
        return JsValue::from_serde(&self).unwrap();
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
    pub fn new(res: &JsValue) -> Settings {
        // console_log!("input setting = {}",res);
        use js_sys::{Boolean, JsString, Reflect};
        let opt_display_mode = Reflect::get(&res, &JsString::from("displayMode")).unwrap();
        let opt_output = Reflect::get(&res, &JsString::from("output")).unwrap();

        let opt_leqno = Reflect::get(&res, &JsString::from("leqno")).unwrap();
        let opt_fleqn = Reflect::get(&res, &JsString::from("fleqn")).unwrap();
        let opt_throw_on_error = Reflect::get(&res, &JsString::from("throwOnError")).unwrap();
        let opt_error_color = Reflect::get(&res, &JsString::from("errorColor")).unwrap();

        let opt_macros = Reflect::get(&res, &JsString::from("macros")).unwrap();
        let opt_min_rule_thickness =
            Reflect::get(&res, &JsString::from("minRuleThickness")).unwrap();
        let opt_color_is_text_color =
            Reflect::get(&res, &JsString::from("colorIsTextColor")).unwrap();
        let opt_strict = Reflect::get(&res, &JsString::from("strict")).unwrap();
        let opt_trust = Reflect::get(&res, &JsString::from("trust")).unwrap();
        let opt_max_size = Reflect::get(&res, &JsString::from("maxSize")).unwrap();
        let opt_max_expand = Reflect::get(&res, &JsString::from("maxExpand")).unwrap();
        let opt_global_group = Reflect::get(&res, &JsString::from("globalGroup")).unwrap();

        let res = Settings {
            display_mode: match opt_display_mode.as_bool() {
                Some(d) => d,
                None => false,
            },
            output: match opt_output.as_string() {
                Some(d) => {
                    let o = OutputType::from_str(d.as_str()).unwrap();
                    // console_log!("output type = {}",o.as_str());
                    o
                }
                None => OutputType::Html,
            },
            leqno: match opt_leqno.as_bool() {
                Some(d) => d,
                None => false,
            },
            fleqn: match opt_fleqn.as_bool() {
                Some(d) => d,
                None => false,
            },
            throw_on_error: match opt_throw_on_error.as_bool() {
                Some(d) => d,
                None => false,
            },
            error_color: match opt_error_color.as_string() {
                Some(d) => d,
                None => String::from("#cc0000"),
            },
            macros: HashMap::new(),
            min_rule_thickness: match opt_min_rule_thickness.as_f64() {
                Some(d) => d,
                None => 0.0,
            },
            color_is_text_color: match opt_color_is_text_color.as_bool() {
                Some(d) => d,
                None => false,
            },
            strict: match opt_strict.as_string() {
                Some(d) => StrictType::from_str(d.as_str()).unwrap(),
                None => StrictType::Warn,
            },
            trust: match opt_trust.as_bool() {
                Some(d) => d,
                None => false,
            },
            max_size: opt_max_size.as_f64(),
            max_expand: match opt_max_expand.as_f64() {
                Some(d) => Some(d as i32),
                None => None,
            },
            global_group: match opt_global_group.as_bool() {
                Some(d) => d,
                None => false,
            },
        };
        //return tmp.into_serde().unwrap();
        return res;
    }

    /**
     * Report nonstrict (non-LaTeX-compatible) input.
     * Can safely not be called if `this.strict` is false in JavaScript.
     */
    pub fn reportNonstrict(&self, errorCode: String, errorMsg: String, token: Option<Token>) {
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
    pub fn isTrusted(context: JsValue) -> bool {
        // if (context.url && !context.protocol) {
        //     context.protocol = utils.protocolFromUrl(context.url);
        // }
        // const trust = typeof this.trust === "function"
        //     ? this.trust(context)
        //     : this.trust;
        // return Boolean(trust);
        return false;
    }
}
