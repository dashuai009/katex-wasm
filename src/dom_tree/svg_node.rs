use crate::dom_tree::css_style::CssStyle;
use crate::utils::{escape, make_em};
use crate::{path_get, scriptFromCodepoint, VirtualNode};
use js_sys::Array;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
/**
 * SVG nodes are used to render stretchy wide elements.
 */
#[wasm_bindgen]
pub struct SvgNode {
    children: Vec<Box<dyn Any>>,
    attributes: HashMap<String, String>,
}

impl VirtualNode for SvgNode {
    fn to_node(&self) -> web_sys::Node {
        let svgNS = "http://www.w3.org/2000/svg";

        let document = web_sys::window().expect("").document().expect("");
        let node = document.create_element_ns(Some(svgNS), "svg").expect("");

        // Apply attributes
        for (k, v) in self.attributes.iter() {
            web_sys::Element::set_attribute(&node, k.as_str(), v.as_str());
        }
        // for child in self.children {}

        return web_sys::Node::from(node);
    }

    fn to_markup(&self) -> String {
        let mut markup = "<svg xmlns=\"http://www.w3.org/2000/svg\"".to_string();

        for (k, v) in self.attributes.iter() {
            markup.push_str(format!(" {}:{}", k, v).as_str());
        }

        markup.push_str(">");

        // for child in self.children {}
        markup.push_str("</svg>");

        return markup;
    }
}
#[wasm_bindgen]
impl SvgNode {
    #[wasm_bindgen(constructor)]
    pub fn new(attributes: js_sys::Object) -> SvgNode {
        let mut res = HashMap::new();
        for (k, v) in js_sys::Object::keys(&attributes)
            .iter()
            .zip(js_sys::Object::values(&attributes).iter())
        {
            res.insert(k.as_string().unwrap(), v.as_string().unwrap());
        }
        SvgNode {
            children: vec![],
            attributes: res,
        }
    }

    pub fn toNode(&self) -> web_sys::Node {
        return self.to_node();
    }

    pub fn toMarkup(&self) -> String {
        return self.to_markup();
    }
}
