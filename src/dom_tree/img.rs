use crate::dom_tree::css_style::CssStyle;
use crate::utils::escape;
use crate::{HasClassNode, HtmlDomNode, VirtualNode};
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Img {
    src: String,
    alt: String,
    classes: Vec<String>,
    height: f64,
    depth: f64,
    max_font_size: f64,
    style: CssStyle,
}

#[wasm_bindgen]
impl Img {
    #[wasm_bindgen(constructor)]
    pub fn new(src: String, alt: String, style: CssStyle) -> Img {
        Img {
            src: src,
            alt: alt,
            classes: vec![String::from("mord")],
            height: 0.0,
            depth: 0.0,
            max_font_size: 0.0,
            style: style,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn classes(&self) -> Array {
        let arr = Array::new_with_length(self.classes.len() as u32);
        for (i, s) in self.classes.iter().enumerate() {
            arr.set(i as u32, JsValue::from_str(s));
        }
        arr
    }
}

impl VirtualNode for Img {
    fn to_node(&self) -> web_sys::Node {
        let document = web_sys::window().expect("").document().expect("");
        let node = document.create_element("img").expect("");
        web_sys::Element::set_attribute(&node, "src", self.src.as_str());
        web_sys::Element::set_attribute(&node, "alt", self.alt.as_str());
        web_sys::Element::set_attribute(&node, "className", "mord");
        let style_str = self.style.to_css_str();
        web_sys::Element::set_attribute(&node, "style", style_str.as_str());

        return web_sys::Node::from(node);
    }

    fn to_markup(&self) -> String {
        let mut markup = format!("<img  src='{} 'alt='${}' ", self.src, self.alt);
        let style_str = escape(&format!("style={}", self.style.to_css_str()));
        markup.push_str(style_str.as_str());
        markup += "'/>";
        return markup;
    }
}

impl HasClassNode for Img {
    fn has_class(&self, class_name: &String) -> bool {
        return self.classes.contains(class_name);
    }
}

impl HtmlDomNode for Img {}
#[wasm_bindgen]
impl Img {
    pub fn toNode(&self) -> web_sys::Node {
        return self.to_node();
    }

    pub fn toMarkup(&self) -> String {
        return self.to_markup();
    }

    pub fn hasClass(&self, class_name: String) -> bool {
        return self.has_class(&class_name);
    }
}
