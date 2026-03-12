use wasm_bindgen::prelude::*;
mod mathS;
pub(crate) mod public;
mod textS;
use crate::{
    symbols::{
        mathS::define_all_math_symbols,
        public::{Font, Group, Symbol},
        textS::define_all_text_symbols,
    },
    types::Mode,
};
use std::{collections::HashMap, str::FromStr, sync::LazyLock};

use std::sync::Mutex;

static MATH_SYMBOLS: LazyLock<HashMap<String, Symbol>> =
    LazyLock::new(define_all_math_symbols);

static TEXT_SYMBOLS: LazyLock<HashMap<String, Symbol>> =
    LazyLock::new(define_all_text_symbols);

pub fn get_symbol(mode: Mode, name: &str) -> Option<Symbol> {
    match mode {
        Mode::math => MATH_SYMBOLS.get(name).cloned(),
        Mode::text => TEXT_SYMBOLS.get(name).cloned(),
    }
}
#[wasm_bindgen]
pub fn _get_symbol(mode: String, name: String) -> Option<js_sys::Object> {
    let res = get_symbol(Mode::from_str(mode.as_str()).unwrap(), &name);
    if let Some(sy) = res {
        Some(sy.to_js_object())
    } else {
        None
    }
}

// #[wasm_bindgen]
// pub fn wasm_define_symbol(
//     mode: String,
//     font: String,
//     group: String,
//     replace: Option<String>,
//     name: String,
//     acceptUnicodeChar: bool,
// ) {
//     match Mode::from_str(mode.as_str()).unwrap() {
//         Mode::math => {
//             let tmp = Symbol {
//                 font: Font::from_str(font.as_str()).unwrap(),
//                 group: Group::from_str(group.as_str()).unwrap(),
//                 replace: replace.clone(),
//             };
//             mathSymbols.lock().unwrap().insert(name, tmp.clone());

//             if acceptUnicodeChar && replace.is_some() {
//                 mathSymbols.lock().unwrap().insert(replace.unwrap(), tmp);
//             }
//         }
//         Mode::text => {
//             let tmp = Symbol {
//                 font: Font::from_str(font.as_str()).unwrap(),
//                 group: Group::from_str(group.as_str()).unwrap(),
//                 replace: replace.clone(),
//             };
//             textSymbols.lock().unwrap().insert(name, tmp.clone());

//             if acceptUnicodeChar && replace.is_some() {
//                 textSymbols.lock().unwrap().insert(replace.unwrap(), tmp);
//             }
//         }
//     }
// }

pub const LIGATURES: [&str; 4] = ["--", "---", "``", "''"];
