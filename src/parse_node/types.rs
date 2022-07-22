use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct raw{
    mode: Mode,
    loc: Option<SourceLocation>,
    string: String,
}