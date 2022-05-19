// This is an internal module, not part of the KaTeX distribution,
// whose purpose is to generate `unicodeSymbols` in Parser.js
// In this way, only this module, and not the distribution/browser,
// needs String's normalize function. As this file is not transpiled,
// Flow comment types syntax is used.
use wasm_bindgen::prelude::*;
use super::unicodeAccents::unicodeAccents;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

const letters: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\
    αβγδεϵζηθϑικλμνξοπϖρϱςστυφϕχψωΓΔΘΛΞΠΣΥΦΨΩ";
lazy_static! {
    pub static ref unicodeSysmbols: HashMap<String, String> = {
        let mut m = HashMap::new();
        for letter in letters.chars() {
            for accent in unicodeAccents.iter() {
                let combined = String::from(letter) + accent.0;
                let normalized = combined.nfc().collect::<String>();
                if normalized.chars().count() == 1 {
                    m.insert(normalized.clone(), combined.clone());
                }
                for accent2 in unicodeAccents.iter() {
                    if accent.0 == accent2.0 {
                        continue;
                    }
                    let combined2 = String::from(&combined) + accent2.0;
                    let normalized2 = combined2.nfc().collect::<String>();
                    if normalized2.chars().count() == 1 {
                        m.insert(normalized2.clone(), combined2.clone());
                    }
                }
            }
        }
        m
    };
}



#[wasm_bindgen]
pub fn __result() -> js_sys::Object {
    let mut res = js_sys::Object::new();
    for o in unicodeSysmbols.iter() {
        js_sys::Reflect::set(&res, &JsValue::from_str(o.0), &JsValue::from_str(o.1));
    }
    return res;
}