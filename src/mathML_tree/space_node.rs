use crate::mathML_tree::public::{MathDomNode, MathNodeType, ToText};
use crate::utils::{escape, make_em};
use crate::Options::Options;
use crate::{path_get, scriptFromCodepoint, HtmlDomNode, VirtualNode};
use js_sys::Array;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

/**
 * This node represents a space, but may render as <mspace.../> or as text,
 * depending on the width.
 */
#[wasm_bindgen]
pub struct SpaceNode {
    width: f64,
    character: Option<String>,
}

#[wasm_bindgen]
impl SpaceNode {
    /**
     * Create a Space node with width given in CSS ems.
     */
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64) -> SpaceNode {
        return SpaceNode {
            width,
            // See https://www.w3.org/TR/2000/WD-MathML2-20000328/chapter6.html
            // for a table of space-like characters.  We use Unicode
            // representations instead of &LongNames; as it's not clear how to
            // make the latter via document.createTextNode.
            character: if width >= 0.05555 && width <= 0.05556 {
                Some("\u{200a}".parse().unwrap()) // &VeryThinSpace;
            } else if width >= 0.1666 && width <= 0.1667 {
                Some("\u{2009}".parse().unwrap()) // &ThinSpace;
            } else if width >= 0.2222 && width <= 0.2223 {
                Some("\u{2005}".to_string()) // &MediumSpace;
            } else if width >= 0.2777 && width <= 0.2778 {
                Some("\u{2005}\u{200a}".to_string()) // &ThickSpace;
            } else if width >= -0.05556 && width <= -0.05555 {
                Some("\u{200a}\u{2063}".to_string()) // &NegativeVeryThinSpace;
            } else if width >= -0.1667 && width <= -0.1666 {
                Some("\u{2009}\u{2063}".to_string()) // &NegativeThinSpace;
            } else if width >= -0.2223 && width <= -0.2222 {
                Some("\u{205f}\u{2063}".to_string()) // &NegativeMediumSpace;
            } else if width >= -0.2778 && width <= -0.2777 {
                Some("\u{2005}\u{2063}".to_string()) // &NegativeThickSpace;
            } else {
                None
            },
        };
    }
}
impl VirtualNode for SpaceNode {
    fn to_node(&self) -> web_sys::Node {
        web_sys::console::log_1(&"asdfadfasdfasdf".into());
        let document = web_sys::window().expect("").document().expect("");
        return match self.character.clone() {
            Some(c) => web_sys::Node::from(document.create_text_node(c.as_str())),
            None => web_sys::Node::from(
                document
                    .create_element_ns(Some("http://www.w3.org/1998/Math/MathML"), "mspace")
                    .expect(""),
            ),
        };
    }
    fn to_markup(&self) -> String {
        return match self.character.clone() {
            Some(c) => format!(" < mtext > {c} < /mtext >"),
            None => format!(" <mspace width = \"{}\" / >", make_em(self.width)),
        };
    }
}
impl ToText for SpaceNode {
    fn to_text(&self) -> String {
        match self.character.clone() {
            Some(c) => c,
            None => " ".to_string(),
        }
    }
}
impl MathDomNode for SpaceNode {}

#[wasm_bindgen]
impl SpaceNode {
    pub fn toNode(&self) -> web_sys::Node {
        return self.to_node();
    }
    pub fn toMarkup(&self) -> String {
        return self.to_markup();
    }
    pub fn toText(&self) -> String {
        return self.to_text();
    }
}
