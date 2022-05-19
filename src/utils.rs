use wasm_bindgen::prelude::*;

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
}

use regex::Regex;
use std::collections::HashMap;
lazy_static! {
    static ref uppercase: Regex = Regex::new("([A-Z])").unwrap();
    static ref ESCAPE_REGEX: Regex = Regex::new("[&><\"']").unwrap();
}


// hyphenate and escape adapted from Facebook's React under Apache 2 license
pub fn hyphenate(s: String)->String {
    return uppercase.replace_all(s.as_str(), "-$1").to_string().to_lowercase();
}

pub fn ESCAPE_LOOKUP(c:char)-> String {
    return match c{
        '&' => "&amp;".to_string(),
        '>' => "&gt;".to_string(),
        '<' => "&lt;".to_string(),
        '\"'=>"&quot;".to_string(),
        '\'' =>"&#x27;".to_string(),
        _=> c.to_string()
    }
}
/**
 * Escapes text to prevent scripting attacks.
 */
pub fn escape(text: String)->String {
    return text.chars().map(ESCAPE_LOOKUP).collect();
}
