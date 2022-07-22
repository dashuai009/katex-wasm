use crate::dom_tree::css_style::CssStyle;
use crate::utils::{escape};
use crate::units::make_em;
use crate::{path_get, scriptFromCodepoint, VirtualNode};
use js_sys::Array;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct PathNode /* implements VirtualNode*/ {
    pub pathName: String,
    pub alternate: Option<String>,
}

impl VirtualNode for PathNode {
    fn to_node(&self) -> web_sys::Node {
        let svgNS = "http://www.w3.org/2000/svg";

        let document = web_sys::window().expect("").document().expect("");
        let node = document.create_element_ns(Some(svgNS), "path").expect("");

        if let Some(alt) = self.alternate.clone() {
            web_sys::Element::set_attribute(&node, "d", alt.as_str());
        } else {
            web_sys::Element::set_attribute(&node, "d", path_get(self.pathName.clone()).as_str());
        }
        return web_sys::Node::from(node);
    }

    fn to_markup(&self) -> String {
        if let Some(alt) = self.alternate.clone() {
            return format!("<path d='{alt}'/>");
        } else {
            return format!("<path d='{}'/>", path_get(self.pathName.clone()));
        }
    }
}
#[wasm_bindgen]
impl PathNode {
    #[wasm_bindgen(constructor)]
    pub fn new(pathName: String, alternate: Option<String>) -> PathNode {
        PathNode {
            pathName,
            alternate, // Used only for \sqrt, \phase, & tall delims
        }
    }

    pub fn toNode(&self) -> web_sys::Node {
        return self.to_node();
    }

    pub fn toMarkup(&self) -> String {
        return self.to_markup();
    }
}
