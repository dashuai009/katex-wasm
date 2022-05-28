use wasm_bindgen::prelude::*;

use crate::token::Token;
/**
 * This is a module for storing settings passed into KaTeX. It correctly handles
 * default settings.
 */
use crate::utils;
use crate::ParseError::ParseError;

// Custom KaTeX behaviors.

// use crate::{error::Result, js_engine::JsScope};
// use derive_builder::Builder;
// use itertools::process_results;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::default;
use std::str::FromStr;

/// Output type from KaTeX.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OutputType {
    /// Outputs KaTeX in HTML only.
    Html,
    /// Outputs KaTeX in MathML only.
    Mathml,
    /// Outputs HTML for visual rendering and includes MathML for accessibility.
    HtmlAndMathml,
}
impl FromStr for OutputType {
    type Err = ();

    fn from_str(input: &str) -> Result<OutputType, Self::Err> {
        match input {
            "html" => Ok(OutputType::Html),
            "mathml" => Ok(OutputType::Mathml),
            "htmlAndMathml" => Ok(OutputType::HtmlAndMathml),
            _ => Err(()),
        }
    }
}
/// Options to be passed to KaTeX.
///
/// Read <https://katex.org/docs/options.html> for more information.
///     
#[non_exhaustive]
#[derive(Clone, Debug, Serialize,Deserialize)]
// #[wasm_bindgen]
pub struct Settings {
    /// Whether to render the math in the display mode.
    display_mode: bool,
    /// KaTeX output type.
    output_type: OutputType,
    /// Whether to have `\tags` rendered on the left instead of the right.
    leqno: bool,
    /// Whether to make display math flush left.
    fleqn: bool,
    /// Whether to let KaTeX throw a ParseError for invalid LaTeX.
    throw_on_error: bool,
    /// Color used for invalid LaTeX.
    error_color: String,
    /// Collection of custom macros.
    /// Read <https://katex.org/docs/options.html> for more information.
    macros: HashMap<String,String>,
    /// Specifies a minimum thickness, in ems.
    /// Read <https://katex.org/docs/options.html> for more information.
    min_rule_thickness: f64,
    /// Max size for user-specified sizes.
    /// If set to `None`, users can make elements and spaces arbitrarily large.
    /// Read <https://katex.org/docs/options.html> for more information.
    #[allow(clippy::option_option)]
    max_size: Option<f64>,
    /// Limit the number of macro expansions to the specified number.
    /// If set to `None`, the macro expander will try to fully expand as in LaTeX.
    /// Read <https://katex.org/docs/options.html> for more information.
    #[allow(clippy::option_option)]
    max_expand: Option<i32>,
    /// Whether to trust users' input.
    /// Read <https://katex.org/docs/options.html> for more information.
    trust: bool,
}

impl Settings {
    /// Return [`OptsBuilder`].
    // pub fn builder() -> OptsBuilder {
    //     OptsBuilder::default()
    // }

    pub fn set_display_mode(&mut self, flag: bool) {
        self.display_mode = flag;
    }

    pub fn set_output_type(&mut self, output_type: OutputType) {
        self.output_type = output_type;
    }

    pub fn set_leqno(&mut self, flag: bool) {
        self.leqno = flag;
    }

    pub fn set_fleqn(&mut self, flag: bool) {
        self.fleqn = flag;
    }

    pub fn set_throw_on_error(&mut self, flag: bool) {
        self.throw_on_error = flag;
    }

    pub fn set_error_color(&mut self, color: String) {
        self.error_color = color;
    }

    pub fn add_macro(&mut self, entry_name: String, entry_data: String) {
        self.macros.insert(entry_name, entry_data);
    }
    
    pub fn set_min_rule_thickness(&mut self, value: f64) {
        self.min_rule_thickness = value;
    }

    pub fn set_max_size(&mut self, value: f64) {
        self.max_size = Some(value);
    }

    pub fn set_max_expand(&mut self, value: i32) {
        self.max_expand = Some(value);
    }

    pub fn set_trust(&mut self, flag: bool) {
        self.trust = flag;
    }
}

#[wasm_bindgen]
impl Settings {
    pub(crate) fn to_js_value(&self) -> JsValue {
        return  JsValue::from_serde(&self).unwrap();
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

impl Settings {
    pub fn new(res: JsValue) -> Settings {
        return res.into_serde().unwrap();
    }

    /**
     * Report nonstrict (non-LaTeX-compatible) input.
     * Can safely not be called if `this.strict` is false in JavaScript.
     */
    pub fn reportNonstrict(errorCode: String, errorMsg: String, token: &JsValue) {
        // let strict = this.strict;
        // if (typeof strict === "function") {
        //     // Allow return value of strict function to be boolean or string
        //     // (or null/undefined, meaning no further processing).
        //     strict = strict(errorCode, errorMsg, token);
        // }
        // if (!strict || strict === "ignore") {
        //     return;
        // } else if (strict === true || strict === "error") {
        //     throw new ParseError(
        //         "LaTeX-incompatible input and strict mode is set to 'error': " +
        //         `${errorMsg} [${errorCode}]`, token);
        // } else if (strict === "warn") {
        //     typeof console !== "undefined" && console.warn(
        //         "LaTeX-incompatible input and strict mode is set to 'warn': " +
        //         `${errorMsg} [${errorCode}]`);
        // } else {  // won't happen in type-safe code
        //     typeof console !== "undefined" && console.warn(
        //         "LaTeX-incompatible input and strict mode is set to " +
        //         `unrecognized '${strict}': ${errorMsg} [${errorCode}]`);
        // }
    }

    /**
     * Check whether to apply strict (LaTeX-adhering) behavior for unusual
     * input (like `\\`).  Unlike `nonstrict`, will not throw an error;
     * instead, "error" translates to a return value of `true`, while "ignore"
     * translates to a return value of `false`.  May still print a warning:
     * "warn" prints a warning and returns `false`.
     * This is for the second category of `errorCode`s listed in the README.
     */
    pub fn useStrictBehavior(errorCode: String, errorMsg: String, token: JsValue) -> bool {
        // let strict = this.strict;
        // if (typeof strict === "function") {
        //     // Allow return value of strict function to be boolean or string
        //     // (or null/undefined, meaning no further processing).
        //     // But catch any exceptions thrown by function, treating them
        //     // like "error".
        //     try {
        //         strict = strict(errorCode, errorMsg, token);
        //     } catch (error) {
        //         strict = "error";
        //     }
        // }
        // if (!strict || strict === "ignore") {
        //     return false;
        // } else if (strict === true || strict === "error") {
        //     return true;
        // } else if (strict === "warn") {
        //     typeof console !== "undefined" && console.warn(
        //         "LaTeX-incompatible input and strict mode is set to 'warn': " +
        //         `${errorMsg} [${errorCode}]`);
        //     return false;
        // } else {  // won't happen in type-safe code
        //     typeof console !== "undefined" && console.warn(
        //         "LaTeX-incompatible input and strict mode is set to " +
        //         `unrecognized '${strict}': ${errorMsg} [${errorCode}]`);
        //     return false;
        // }
        return false;
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
