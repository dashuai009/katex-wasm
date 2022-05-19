use wasm_bindgen::prelude::*;

pub trait VirturalNode{
    fn toNode(&self)->web_sys::Node;
    fn toMarkup(&self)->String;
}