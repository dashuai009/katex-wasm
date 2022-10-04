use std::any::Any;
use crate::utils::escape;
use crate::{
    dom_tree::css_style::CssStyle,
    tree::{HtmlDomNode, VirtualNode},
    Options::Options,
};
use std::collections::HashMap;
use struct_format::html_dom_node;

use super::utils::{this_init_node, this_to_markup, this_to_node};

/**
 * This node represents an anchor (<a>) element with a hyperlink.  See `span`
 * for further details.
 */
#[derive(html_dom_node, Clone)]
pub struct Anchor {
    children: Vec<Box<dyn HtmlDomNode>>,
    attributes: HashMap<String, String>,
    classes: Vec<String>,
    height: f64,
    depth: f64,
    max_font_size: f64,
    style: CssStyle,
}
impl Anchor {
    pub fn new(
        href: String,
        classes: Vec<String>,
        children: Vec<Box<dyn HtmlDomNode>>,
        options: Options,
    ) -> Anchor {
        let mut res = Anchor {
            children: vec![],
            attributes: HashMap::new(),
            classes: vec![],
            height: 0.0,
            depth: 0.0,
            max_font_size: 0.0,
            style: CssStyle::new(),
        };
        let _opt = Some(options);
        let _style = CssStyle::new();
        this_init_node!(res, classes, _opt, _style);
        res.children = children;
        res.attributes.insert("href".to_string(), href);
        return res;
    }

    pub fn set_attribute(&mut self, attribute: String, value: String) {
        self.attributes.insert(attribute, value);
    }
}
impl VirtualNode for Anchor {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn to_node(&self) -> web_sys::Node {
        this_to_node!(self, "a")
    }

    fn to_markup(&self) -> String {
        this_to_markup!(self, "a")
    }
}
impl Anchor {
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
