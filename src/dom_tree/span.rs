use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::line_node::LineNode;
use crate::dom_tree::utils::{this_init_node, this_to_markup, this_to_node};
use crate::utils::{escape};
use crate::units::make_em;
use crate::Options::Options;
use crate::{path_get, scriptFromCodepoint, HasClassNode, HtmlDomNode, VirtualNode};
use js_sys::Array;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

pub struct Span<ChildType: VirtualNode> {
    children: Vec<ChildType>,
    attributes: HashMap<String, String>,
    classes: Vec<String>,
    height: f64,
    depth: f64,
    width: f64,
    style: CssStyle,
    max_font_size: f64,
}

impl<ChildType: VirtualNode> Span<ChildType> {
    pub fn new(
        classes: Vec<String>,
        children: Vec<ChildType>,
        options: Option<Options>,
        style: CssStyle,
    ) -> Span<ChildType> {
        let mut res = Span::<ChildType> {
            children: vec![],
            attributes: HashMap::new(),
            classes: vec![],
            height: 0.0,
            depth: 0.0,
            width: 0.0,
            style: CssStyle::default(),
            max_font_size: 0.0,
        };
        this_init_node!(res, classes, options, style);
        res.children = children;
        return res;
    }
    pub fn set_attribute(&mut self, attribute: String, value: String) {
        self.attributes.insert(attribute, value);
    }
}

impl<ChildType: VirtualNode> HasClassNode for Span<ChildType> {
    fn has_class(&self, class_name: &String) -> bool {
        return self.classes.contains(class_name);
    }
}

impl<ChildType: VirtualNode> HtmlDomNode for Span<ChildType> {}

impl<ChildType: VirtualNode> VirtualNode for Span<ChildType> {
    fn to_node(&self) -> web_sys::Node {
        this_to_node!(self, "span")
    }

    fn to_markup(&self) -> String {
        this_to_markup!(self, "span")
    }
}

// /**
//  * Sets an arbitrary attribute on the span. Warning: use this wisely. Not
//  * all browsers support attributes the same, and having too many custom
//  * attributes is probably bad.
//  */
// setAttribute(attribute: string, value: string) {
// this.attributes[attribute] = value;
// }
//
//
// toNode(): HTMLElement {
// return toNode.call(this, "span");
// }
//
// toMarkup(): string {
// return toMarkup.call(this, "span");
// }
// }
