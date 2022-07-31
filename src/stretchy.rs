use std::collections::HashMap;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::mathML_tree::{math_node::MathNode, public::MathNodeType, text_node::TextNode};

lazy_static! {
    static ref stretchy_codepoint: HashMap<&'static str, &'static str> = {
        HashMap::from([
       ("\\widechar", "^"),
       ("\\widecheck", "ˇ"),
       ("\\widetilde", "~"),
       ("\\utilde", "~"),
       ("\\overleftarrow", "\u{2190}"),
       ("\\underleftarrow", "\u{2190}"),
       ("\\xleftarrow", "\u{2190}"),
       ("\\overrightarrow", "\u{2192}"),
       ("\\underrightarrow", "\u{2192}"),
       ("\\xrightarrow", "\u{2192}"),
       ("\\underbrace", "\u{23df}"),
       ("\\overbrace", "\u{23de}"),
       ("\\overgroup", "\u{23e0}"),
       ("\\undergroup", "\u{23e1}"),
       ("\\overleftrightarrow", "\u{2194}"),
       ("\\underleftrightarrow", "\u{2194}"),
       ("\\xleftrightarrow", "\u{2194}"),
       ("\\Overrightarrow", "\u{21d2}"),
       ("\\xRightarrow", "\u{21d2}"),
       ("\\overleftharpoon", "\u{21bc}"),
       ("\\xleftharpoonup", "\u{21bc}"),
       ("\\overrightharpoon", "\u{21c0}"),
       ("\\xrightharpoonup", "\u{21c0}"),
       ("\\xLeftarrow", "\u{21d0}"),
       ("\\xLeftrightarrow", "\u{21d4}"),
       ("\\xhookleftarrow", "\u{21a9}"),
       ("\\xhookrightarrow", "\u{21aa}"),
       ("\\xmapsto", "\u{21a6}"),
       ("\\xrightharpoondown", "\u{21c1}"),
       ("\\xleftharpoondown", "\u{21bd}"),
       ("\\xrightleftharpoons", "\u{21cc}"),
       ("\\xleftrightharpoons", "\u{21cb}"),
       ("\\xtwoheadleftarrow", "\u{219e}"),
       ("\\xtwoheadrightarrow", "\u{21a0}"),
       ("\\xlongequal", "="),
       ("\\xtofrom", "\u{21c4}"),
       ("\\xrightleftarrows", "\u{21c4}"),
       ("\\xrightequilibrium", "\u{21cc}"),
       ("\\xleftequilibrium", "\u{21cb}"),
       ("\\\\cdrightarrow", "\u{2192}"),
       ("\\\\cdleftarrow", "\u{2190}"),
        ("\\\\cdlongequal", "=")
            ])
    };
}


#[wasm_bindgen]
pub fn get_stretchy_codepoint(key: String) -> String {
    stretchy_codepoint.get(&key.as_str()).unwrap().to_string()
}

fn MathML_node(label: String)-> MathNode {
    let mut node = MathNode::new(
       MathNodeType::Mo,
        vec![
            Box::new(TextNode::new(
                get_stretchy_codepoint(label)
            ))
        ],
        vec![]
    );
    node.set_attribute("stretchy".to_string(), "true".to_string());
    return node;
}
