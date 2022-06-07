use wasm_bindgen::prelude::*;
use crate::dom_tree::css_style::CssStyle;
use crate::VirturalNode;

#[wasm_bindgen]
pub struct Img {
    src: String,
    alt: String,
    classes: Vec<String>,
    height: f64,
    depth: f64,
    maxFontSize: f64,
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
            maxFontSize: 0.0,
            style: style,
        }
    }

    pub fn hasClass(className: String) -> bool {
        //   TODO interface HtmlDomNode
        return false; //return utils.contains(this.classes, className);
    }
}

impl VirturalNode for Img {
    fn toNode(&self) -> web_sys::Node {
        let document = web_sys::window().expect("").document().expect("");
        let node = document.create_element("img").expect("");
        web_sys::Element::set_attribute(&node, "src", self.src.as_str());
        web_sys::Element::set_attribute(&node, "alt", self.alt.as_str());
        web_sys::Element::set_attribute(&node, "className", "mord");
        web_sys::Element::set_attribute(&node, "style", self.style.to_string().as_str());

        return web_sys::Node::from(node);
    }

    fn toMarkup(&self) -> String {
        let mut markup = format!("<img  src='{} 'alt='${}' ", self.src, self.alt);

        // Add the styles, after hyphenation
        // let styles = "";
        // for style in self.style {
        //     if (self.style.hasOwnProperty(style)) {
        //         styles += format!("${utils.hyphenate(style)}:${this.style[style]};");
        //     }
        // }
        // if styles !="" {
        //     markup += format!("style=\"${utils.escape(styles)}");
        // }

        markup += "'/>";
        return markup;
    }
}
