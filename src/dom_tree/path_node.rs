use std::any::Any;
use crate::dom_tree::css_style::CssStyle;
use crate::utils::{escape};
use crate::units::make_em;
use crate::{path_get, scriptFromCodepoint, VirtualNode};
use js_sys::Array;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct PathNode /* implements VirtualNode*/ {
    pub pathName: String,
    pub alternate: Option<String>,
}

impl VirtualNode for PathNode {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn to_node(&self) -> web_sys::Node {
        let svgNS = "http://www.w3.org/2000/svg";

        let document = web_sys::window().expect("").document().expect("");
        let node = document.create_element_ns(Some(svgNS), "path").expect("");

        let data = self.path_data();
        web_sys::Element::set_attribute(&node, "d", data.as_str());
        return web_sys::Node::from(node);
    }

    fn to_markup(&self) -> String {
        return format!("<path d=\"{}\"/>", self.path_data());
    }
}
#[wasm_bindgen]
impl PathNode {
    fn path_data(&self) -> String {
        let data = if let Some(alt) = &self.alternate {
            alt.clone()
        } else {
            path_get(self.pathName.clone())
        };
        data.lines()
            .map(|line| line.strip_prefix("    ").unwrap_or(line))
            .collect::<Vec<_>>()
            .join("\n")
            .trim_end()
            .to_string()
    }

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
