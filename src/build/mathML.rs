use crate::mathML_tree::text_node::TextNode;
use crate::types::Mode;
use crate::Options::Options;
use crate::{get_symbol, LIGATURES};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/**
 * Takes a symbol and converts it into a MathML text node after performing
 * optional replacement from symbols.js.
 */
pub fn make_text(text: String, mode: Mode, options: Option<&Options>) -> TextNode {
    let mut flag = false;
    if LIGATURES.contains(&text.as_str()) && options.is_some() {
        if let Some(opt) = options {
            if opt.fontFamily != ""
                && &opt.fontFamily[4..6] != "tt"
                && opt.font != ""
                && &opt.font[4..6] != "tt"
            {
                flag = true;
            }
        }
    }
    if let Some(c) = text.chars().next() {
        if c as u32 != 0xD835 && !flag {
            if let Some(sym) = get_symbol(mode, &text) {
                if let Some(r) = sym.replace {
                    return TextNode::new(r);
                }
            }
        }
    }

    return TextNode::new(text);
}

// symbols[mode][text] &&
// symbols[mode][text].replace &&
// text.charCodeAt(0) !== 0xD835 &&
// !(
//     ligatures.hasOwnProperty(text) &&
//     options &&
//     (
//         (options.fontFamily && options.fontFamily.substr(4, 2) === "tt") ||
//         (options.font && options.font.substr(4, 2) === "tt")
//     )
// )
#[wasm_bindgen]
pub fn makeText(text: String, mode: String, options: &Options) -> TextNode {
    let m = Mode::from_str(&mode).unwrap();
    return make_text(text, m, Some(options));
}

#[wasm_bindgen]
pub fn makeTextNoneOpt(text: String, mode: String) -> TextNode {
    let m = Mode::from_str(&mode).unwrap();
    return make_text(text, m, None);
}
