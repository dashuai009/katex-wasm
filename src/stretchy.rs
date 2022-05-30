use std::collections::HashMap;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref stretchy_codepoint: HashMap<&'static str, &'static str> = {
        let mut res = HashMap::new();
        res.insert("widechar", "^");
        res.insert("widecheck", "ˇ");
        res.insert("widetilde", "~");
        res.insert("utilde", "~");
        res.insert("overleftarrow", "\u{2190}");
        res.insert("underleftarrow", "\u{2190}");
        res.insert("xleftarrow", "\u{2190}");
        res.insert("overrightarrow", "\u{2192}");
        res.insert("underrightarrow", "\u{2192}");
        res.insert("xrightarrow", "\u{2192}");
        res.insert("underbrace", "\u{23df}");
        res.insert("overbrace", "\u{23de}");
        res.insert("overgroup", "\u{23e0}");
        res.insert("undergroup", "\u{23e1}");
        res.insert("overleftrightarrow", "\u{2194}");
        res.insert("underleftrightarrow", "\u{2194}");
        res.insert("xleftrightarrow", "\u{2194}");
        res.insert("Overrightarrow", "\u{21d2}");
        res.insert("xRightarrow", "\u{21d2}");
        res.insert("overleftharpoon", "\u{21bc}");
        res.insert("xleftharpoonup", "\u{21bc}");
        res.insert("overrightharpoon", "\u{21c0}");
        res.insert("xrightharpoonup", "\u{21c0}");
        res.insert("xLeftarrow", "\u{21d0}");
        res.insert("xLeftrightarrow", "\u{21d4}");
        res.insert("xhookleftarrow", "\u{21a9}");
        res.insert("xhookrightarrow", "\u{21aa}");
        res.insert("xmapsto", "\u{21a6}");
        res.insert("xrightharpoondown", "\u{21c1}");
        res.insert("xleftharpoondown", "\u{21bd}");
        res.insert("xrightleftharpoons", "\u{21cc}");
        res.insert("xleftrightharpoons", "\u{21cb}");
        res.insert("xtwoheadleftarrow", "\u{219e}");
        res.insert("xtwoheadrightarrow", "\u{21a0}");
        res.insert("xlongequal", "=");
        res.insert("xtofrom", "\u{21c4}");
        res.insert("xrightleftarrows", "\u{21c4}");
        res.insert("xrightequilibrium", "\u{21cc}");
        res.insert("xleftequilibrium", "\u{21cb}");
        res.insert("\\cdrightarrow", "\u{2192}");
        res.insert("\\cdleftarrow", "\u{2190}");
        res.insert("\\cdlongequal", "=");
        res
    };
}


#[wasm_bindgen]
pub fn get_stretchy_codepoint(key:String)->String{
    stretchy_codepoint.get(&key.as_str()).unwrap().to_string()
}