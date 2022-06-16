use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
pub trait VirturalNode {
    fn toNode(&self) -> web_sys::Node;
    fn toMarkup(&self) -> String;
}
