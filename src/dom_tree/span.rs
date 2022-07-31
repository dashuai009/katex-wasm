use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::line_node::LineNode;
use crate::dom_tree::utils::{this_init_node, this_to_markup, this_to_node};
use crate::units::make_em;
use crate::utils::escape;
use crate::Options::Options;
use crate::{path_get, scriptFromCodepoint, HtmlDomNode, VirtualNode};
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

impl<ChildType: VirtualNode> HtmlDomNode for Span<ChildType> {
        fn get_classes(&self) -> &Vec<String> {
            return &self.classes;
        }
        fn get_mut_classes(&mut self) -> &mut Vec<String> {
            return &mut self.classes;
        }
        fn set_classes(&mut self, _classes: Vec<String>) {
            self.classes = _classes;
        }
    
        fn get_height(&self) -> f64 {
            return self.height;
        }
        fn set_height(&mut self, _height: f64) {
            self.height = _height;
        }
    
        fn get_depth(&self) -> f64 {
            return self.depth;
        }
    
        fn set_depth(&mut self, _depth: f64) {
            self.depth = _depth;
        }
    
        fn get_max_font_size(&self) -> f64 {
            return self.max_font_size;
        }
        fn set_max_font_size(&mut self, _max_font_size: f64) {
            self.max_font_size = _max_font_size;
        }
    
        fn get_style(&self) -> &CssStyle {
            return &self.style;
        }
        fn get_mut_style(&mut self) -> &mut CssStyle {
            return &mut self.style;
        }
        fn set_style(&mut self, _style: CssStyle) {
            self.style = _style;
        }
    
        fn has_class(&self, class_name: &String) -> bool {
            return self.classes.contains(class_name);
        }
    }
impl<ChildType: VirtualNode> VirtualNode for Span<ChildType> {
    fn to_node(&self) -> web_sys::Node {
        this_to_node!(self, "span")
    }

    fn to_markup(&self) -> String {
        this_to_markup!(self, "span")
    }
}
impl<ChildType: HtmlDomNode> Span<ChildType> {
    /**
     * Calculate the height, depth, and maxFontSize of an element based on its
     * children.
     */
    pub fn size_element_from_children(&mut self) {
        for child in self.children.iter() {
            self.height = self.height.max(child.get_height());
            self.depth = self.depth.max(child.get_depth());
            self.max_font_size = f64::max(self.max_font_size, child.get_max_font_size());
        }
    }
}
