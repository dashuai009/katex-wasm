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

use crate::parse_node::types::ParseNodeToAny;
use crate::{parse_node, AnyParseNode};
use regex::Regex;
use std::collections::HashMap;
lazy_static! {
    static ref UPPERCASE: Regex = Regex::new("([A-Z])").unwrap();
    static ref ESCAPE_REGEX: Regex = Regex::new("[&><\"']").unwrap();
}

// hyphenate and escape adapted from Facebook's React under Apache 2 license
#[wasm_bindgen]
pub fn hyphenate(s: String) -> String {
    return UPPERCASE
        .replace_all(s.as_str(), "-$1")
        .to_string()
        .to_lowercase();
}



pub fn escape_to(out: &mut String, text: &str) {
    let mut start = 0;
    for (i, byte) in text.bytes().enumerate() {
        let replacement = match byte {
            b'&' => "&amp;",
            b'>' => "&gt;",
            b'<' => "&lt;",
            b'"' => "&quot;",
            b'\'' => "&#x27;",
            _ => continue,
        };
        // ASCII delimiter offsets are always valid UTF-8 boundaries.
        out.push_str(&text[start..i]);
        out.push_str(replacement);
        start = i + 1;
    }
    out.push_str(&text[start..]);
}

/**
 * Sometimes we want to pull out the innermost element of a group. In most
 * cases, this will just be the group itself, but when ordgroups and colors have
 * a single element, we want to pull that out.
 */
pub fn get_base_elem(_group: &Box<dyn AnyParseNode>) -> &Box<dyn AnyParseNode> {
    return if let Some(group) = _group
        .as_any()
        .downcast_ref::<parse_node::types::ordgroup>()
    {
        if group.body.len() == 1 {
            get_base_elem(&group.body[0])
        } else {
            _group
        }
    } else if let Some(color) = _group.as_any().downcast_ref::<parse_node::types::color>() {
        if color.body.len() == 1 {
            get_base_elem(&color.body[0])
        } else {
            _group
        }
    } else if let Some(font) = _group.as_any().downcast_ref::<parse_node::types::font>() {
        get_base_elem(&font.body)
    } else {
        _group
    };
}
/**
 * TeXbook algorithms often reference "character boxes", which are simply groups
 * with a single character in them. To decide if something is a character box,
 * we find its innermost group, and see if it is a single character.
 */
pub fn is_character_box(group: &Box<dyn AnyParseNode>) -> bool {
    let base_elem = get_base_elem(group);
    // These are all they types of groups which hold single characters
    return base_elem.get_type() == "mathord"
        || base_elem.get_type() == "textord"
        || base_elem.get_type() == "atom";
}

#[cfg(test)]
mod escape_regression_tests {
    use super::escape_to;

    #[test]
    fn preserves_unicode_and_escapes_all_html_delimiters() {
        let mut output = String::from("prefix:");
        escape_to(&mut output, "α中😀&<>\"'e\u{0301}");
        assert_eq!(output, "prefix:α中😀&amp;&lt;&gt;&quot;&#x27;e\u{0301}");
        escape_to(&mut output, "");
        assert!(output.ends_with("e\u{0301}"));
    }
}
