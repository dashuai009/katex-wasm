use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
}

macro_rules!  console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub(crate) use console_log;

#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    console_log!("console_error_panic_hook has set!");
}

use regex::Regex;
use std::collections::HashMap;
lazy_static! {
    static ref uppercase: Regex = Regex::new("([A-Z])").unwrap();
    static ref ESCAPE_REGEX: Regex = Regex::new("[&><\"']").unwrap();
}

// hyphenate and escape adapted from Facebook's React under Apache 2 license
#[wasm_bindgen]
pub fn hyphenate(s: String) -> String {
    return uppercase
        .replace_all(s.as_str(), "-$1")
        .to_string()
        .to_lowercase();
}

pub fn ESCAPE_LOOKUP(c: char) -> String {
    return match c {
        '&' => "&amp;".to_string(),
        '>' => "&gt;".to_string(),
        '<' => "&lt;".to_string(),
        '\"' => "&quot;".to_string(),
        '\'' => "&#x27;".to_string(),
        _ => c.to_string(),
    };
}
/**
 * Escapes text to prevent scripting attacks.
 */
pub fn escape(text: &String) -> String {
    return text.chars().map(ESCAPE_LOOKUP).collect();
}

/**
 * Round `n` to 4 decimal places, or to the nearest 1/10,000th em. See
 * https://github.com/KaTeX/KaTeX/pull/2460.
 */
pub fn make_em(n: f64) -> String {
    format!("{:.4}em", n)
}
