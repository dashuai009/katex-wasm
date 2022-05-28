use super::tree::VirturalNode;
use wasm_bindgen::prelude::*;
/**
 * This node represents an image embed (<img>) element.
 */
#[derive(Debug,Clone)]
#[wasm_bindgen]
pub struct CssStyle {
    backgroundColor: String,
    borderBottomWidth: String,
    borderColor: String,
    borderRightStyle: String,
    borderRightWidth: String,
    borderTopWidth: String,
    borderStyle: String,
    borderWidth: String,
    bottom: String,
    color: String,
    height: String,
    left: String,
    margin: String,
    marginLeft: String,
    marginRight: String,
    marginTop: String,
    minWidth: String,
    paddingLeft: String,
    position: String,
    top: String,
    width: String,
    verticalAlign: String,
}

#[wasm_bindgen]
impl CssStyle{
    pub fn js_clone(&self)->CssStyle{
        self.clone()
    }
}

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
        web_sys::Element::set_attribute(&node,"src",self.src.as_str());
        web_sys::Element::set_attribute(&node,"alt",self.alt.as_str());
        web_sys::Element::set_attribute(&node,"className","mord");

        // Apply inline styles
        // for style in self.style.iter() {
        //     if (self.style.hasOwnProperty(style)) {
        //         // $FlowFixMe
        //         node.style[style] = self.style[style];
        //     }
        // }

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
