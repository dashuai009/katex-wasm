use crate::dom_tree::css_style::CssStyle;
use crate::utils::{escape};
use crate::units::make_em;
use crate::{path_get, scriptFromCodepoint, VirtualNode};
use js_sys::Array;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LineNode {
    attributes: HashMap<String, String>,
}
impl VirtualNode for LineNode {
    fn to_node(&self) -> web_sys::Node {
        let svgNS = "http://www.w3.org/2000/svg";

        let document = web_sys::window().expect("").document().expect("");
        let node = document.create_element_ns(Some(svgNS), "line").expect("");

        // Apply attributes
        for (k, v) in self.attributes.iter() {
            web_sys::Element::set_attribute(&node, k.as_str(), v.as_str());
        }

        return web_sys::Node::from(node);
    }

    fn to_markup(&self) -> String {
        let mut markup: String = "<line".to_string();

        for (k, v) in self.attributes.iter() {
            markup.push_str(format!(" {}:{}", k, v).as_str());
        }

        markup.push_str("/>");

        return markup;
    }
}
#[wasm_bindgen]
impl LineNode {
    #[wasm_bindgen(constructor)]
    pub fn new(attributes: js_sys::Object) -> LineNode {
        let mut res = HashMap::new();
        for (k, v) in js_sys::Object::keys(&attributes)
            .iter()
            .zip(js_sys::Object::values(&attributes).iter())
        {
            res.insert(k.as_string().unwrap(), v.as_string().unwrap());
        }
        // this.attributes = attributes || {};
        LineNode { attributes: res }
    }

    pub fn toNode(self) -> web_sys::Node {
        self.to_node()
    }

    pub fn toMarkup(&self) -> String {
        self.to_markup()
    }
}
