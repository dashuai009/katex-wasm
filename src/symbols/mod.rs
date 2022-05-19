use wasm_bindgen::prelude::*;
mod public;
mod mathS;
mod textS;
use std::{str::FromStr, collections::HashMap};
use crate::{types::Mode, symbols::{public::Symbol, mathS::define_all_math_symbols,textS::define_all_text_symbols}};


use std::sync::Mutex;
lazy_static! {
    pub static ref mathSymbols: Mutex<HashMap<String, Symbol>> = Mutex::new(define_all_math_symbols());
    pub static ref textSymbols: Mutex<HashMap<String, Symbol>> = Mutex::new(define_all_text_symbols());
}

#[wasm_bindgen]
pub fn get_symbol(mode: String, name: String) -> Option<js_sys::Object> {
    match Mode::from_str(mode.as_str()).unwrap() {
        Mode::math => {
            let ms = mathSymbols.lock().unwrap();
            let sy = ms.get(&name);
            if sy.is_some() {
                Some(sy.unwrap().to_js_object())
            } else {
                None
            }
        }
        Mode::text => {
            let tm = textSymbols.lock().unwrap();
            let sy = tm.get(&name);
            if sy.is_some() {
                Some(sy.unwrap().to_js_object())
            } else {
                None
            }
        }
        _ => None,
    }
}