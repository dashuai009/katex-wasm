use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::line_node::LineNode;
use crate::dom_tree::utils::{this_init_node, this_to_markup, this_to_node};
use crate::units::make_em;
use crate::utils::escape;
use crate::Options::Options;
use crate::{path_get, scriptFromCodepoint, HtmlDomNode, VirtualNode};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;
use struct_format::html_dom_node;

#[derive(html_dom_node, Clone)]
pub struct Span {
    children: Vec<Box<dyn HtmlDomNode>>,
    attributes: HashMap<String, String>,
    classes: Vec<String>,
    height: f64,
    depth: f64,
    width: f64,
    style: CssStyle,
    max_font_size: f64,
}

impl Span {
    pub fn new(
        classes: Vec<String>,
        children: Vec<Box<dyn HtmlDomNode>>,
        options: Option<Options>,
        style: CssStyle,
    ) -> Span {
        let mut res = Span {
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

    // pub fn get_mut_children(&mut self) -> &Vec<Box<dyn HtmlDomNode>> {
    //     &self.children
    // }
}

impl VirtualNode for Span {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn to_node(&self) -> web_sys::Node {
        this_to_node!(self, "span")
    }

    fn to_markup(&self) -> String {
        this_to_markup!(self, "span")
    }
}
impl Span {
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
