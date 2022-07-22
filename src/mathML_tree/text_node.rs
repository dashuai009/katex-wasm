use crate::mathML_tree::public::{MathDomNode, MathNodeType, ToText};
use crate::utils::{escape};
use crate::units::make_em;
use crate::Options::Options;
use crate::{path_get, scriptFromCodepoint, HtmlDomNode, VirtualNode};
use js_sys::Array;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TextNode {
    text: String,
}
#[wasm_bindgen]
impl TextNode {
    #[wasm_bindgen(constructor)]
    pub fn new(text: String) -> TextNode {
        TextNode { text }
    }
    pub fn toNode(&self) -> web_sys::Node {
        return self.to_node();
    }

    pub fn toMarkup(&self) -> String {
        return self.to_markup();
    }
    pub fn toText(&self) -> String {
        return self.text.clone();
    }
}
impl VirtualNode for TextNode {
    /**
     * Converts the math node into a MathML-namespaced DOM element.
     */
    fn to_node(&self) -> web_sys::Node {
        let document = web_sys::window().expect("").document().expect("");
        let node = document.create_text_node(self.text.as_str());
        return web_sys::Node::from(node);
    }

    /**
     * Converts the math node into an HTML markup string.
     */
    fn to_markup(&self) -> String {
        return escape(&self.to_text());
    }
}
impl ToText for TextNode {
    /**
     * Converts the math node into a string, similar to innerText, but escaped.
     */
    fn to_text(&self) -> String {
        return self.text.clone();
    }
}
impl MathDomNode for TextNode {}
