use std::any::Any;
use std::fmt::Debug;
use struct_format::html_dom_node;
use wasm_bindgen::prelude::*;
use web_sys::Node;

use crate::{
    dom_tree::css_style::CssStyle
};

///////////////////////////////////////////////////////////////////////////////////////////////////
pub trait VirtualNode: VirtualNodeClone + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self)->&mut dyn Any;
    fn to_node(&self) -> web_sys::Node;
    fn to_markup(&self) -> String;
}

pub trait VirtualNodeClone {
    fn clone_virtual_node(&self) -> Box<dyn VirtualNode>;
}

impl<T> VirtualNodeClone for T
where
    T: VirtualNode + Clone + 'static,
{
    fn clone_virtual_node(&self) -> Box<dyn VirtualNode> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn VirtualNode> {
    fn clone(&self) -> Box<dyn VirtualNode> {
        self.clone_virtual_node()
    }
}
///////////////////////////////////////////////////////////////////////////////////////////////////
pub trait HtmlDomNodeClone {
    //for clone
    fn clone_html_dom_node(&self) -> Box<dyn HtmlDomNode>;
}

impl<T> HtmlDomNodeClone for T
where
    T: HtmlDomNode + Clone + 'static,
{
    fn clone_html_dom_node(&self) -> Box<dyn HtmlDomNode> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn HtmlDomNode> {
    fn clone(&self) -> Box<dyn HtmlDomNode> {
        self.clone_html_dom_node()
    }
}


pub trait HtmlDomNode: VirtualNode + HtmlDomNodeClone {
    fn get_classes(&self) -> &Vec<String>;
    fn get_mut_classes(&mut self) -> &mut Vec<String>;
    fn set_classes(&mut self, _classes: Vec<String>);

    fn get_height(&self) -> f64;
    fn set_height(&mut self, _height: f64);

    fn get_depth(&self) -> f64;
    fn set_depth(&mut self, _depth: f64);

    fn get_max_font_size(&self) -> f64;
    fn set_max_font_size(&mut self, _max_font_size: f64);

    fn get_style(&self) -> &CssStyle;
    fn get_mut_style(&mut self) -> &mut CssStyle;
    fn set_style(&mut self, _style: CssStyle);

    fn has_class(&self, class_name: &String) -> bool;

    fn get_children(&self) -> Option<&Vec<Box<dyn HtmlDomNode>>>;
    fn get_mut_children(&mut self) -> Option<&mut Vec<Box<dyn HtmlDomNode>>>;
}
