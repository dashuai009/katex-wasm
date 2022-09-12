// This is an internal module, not part of the KaTeX distribution,
// whose purpose is to generate `unicodeSymbols` in Parser.js
// In this way, only this module, and not the distribution/browser,
// needs String's normalize function. As this file is not transpiled,
// Flow comment types syntax is used.
use super::unicodeAccents::unicodeAccents;
use std::collections::HashMap;
use std::sync::Mutex;
use unicode_normalization::UnicodeNormalization;
use wasm_bindgen::prelude::*;

const letters: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\
    αβγδεϵζηθϑικλμνξοπϖρϱςστυφϕχψωΓΔΘΛΞΠΣΥΦΨΩ";
lazy_static! {
    pub static ref unicodeSysmbols: Mutex<HashMap<String, String>> = Mutex::new({
        let mut m = HashMap::new();
        for letter in letters.chars() {
            for accent in unicodeAccents.iter() {
                let combined = format!("{}{}", letter, accent.0);
                let normalized = combined.nfc().collect::<String>();
                if normalized.chars().count() == 1 {
                    m.insert(normalized.clone(), combined.clone());
                }
                for accent2 in unicodeAccents.iter() {
                    if accent.0 == accent2.0 {
                        continue;
                    }
                    let combined2 = format!("{}{}", combined, accent2.0);
                    let normalized2 = combined2.nfc().collect::<String>();
                    if normalized2.chars().count() == 1 {
                        m.insert(normalized2.clone(), combined2.clone());
                    }
                }
            }
        }
        m
    });
}

#[wasm_bindgen]
pub fn unicode_sysmbols_result_get(key: String) -> Option<String> {
    match unicodeSysmbols.lock().unwrap().get(&key) {
        Some(s) => Some(s.clone()),
        None => None,
    }
}
