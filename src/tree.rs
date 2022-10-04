use std::any::Any;
use struct_format::html_dom_node;
use wasm_bindgen::prelude::*;
use web_sys::Node;

use crate::{
    dom_tree::css_style::CssStyle,
    mathML_tree::public::{MathDomNode, MathNodeType},
};

///////////////////////////////////////////////////////////////////////////////////////////////////
pub trait VirtualNode: VirtualNodeClone {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self)->&mut dyn Any;
    fn to_node(&self) -> web_sys::Node;
    fn to_markup(&self) -> String;
}

pub trait VirtualNodeClone {
    fn clone_virtual_node(&self) -> Box<dyn VirtualNode>;
    // fn box_to_node(&self) -> web_sys::Node;
    // fn box_to_markup(&self) -> String;
}

impl<T> VirtualNodeClone for T
where
    T: VirtualNode + Clone + 'static,
{
    fn clone_virtual_node(&self) -> Box<dyn VirtualNode> {
        Box::new(self.clone())
    }
    // fn box_to_node(&self) -> web_sys::Node {
    //     self.to_node()
    // }
    // fn box_to_markup(&self) -> String {
    //     self.to_markup()
    // }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn VirtualNode> {
    fn clone(&self) -> Box<dyn VirtualNode> {
        self.clone_virtual_node()
    }
}
// impl VirtualNode for Box<dyn VirtualNode> {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//
//     fn as_mut_any(&mut self) -> &mut dyn Any {
//         self
//     }
//
//     fn to_node(&self) -> web_sys::Node {
//         self.box_to_node()
//     }
//
//     fn to_markup(&self) -> String {
//         self.box_to_markup()
//     }
// }
///////////////////////////////////////////////////////////////////////////////////////////////////
pub trait HtmlDomNodeClone {
    //for clone
    fn clone_html_dom_node(&self) -> Box<dyn HtmlDomNode>;
    //for box<dyn HtmlDomNodeClone)
//     fn box_get_classes(&self) -> &Vec<String>;
//     fn box_get_mut_classes(&mut self) -> &mut Vec<String>;
//     fn box_set_classes(&mut self, _classes: Vec<String>);
//
//     fn box_get_height(&self) -> f64;
//     fn box_set_height(&mut self, _height: f64);
//
//     fn box_get_depth(&self) -> f64;
//     fn box_set_depth(&mut self, _depth: f64);
//
//     fn box_get_max_font_size(&self) -> f64;
//     fn box_set_max_font_size(&mut self, _max_font_size: f64);
//
//     fn box_get_style(&self) -> &CssStyle;
//     fn box_get_mut_style(&mut self) -> &mut CssStyle;
//     fn box_set_style(&mut self, _style: CssStyle);
//
//     fn box_has_class(&self, class_name: &String) -> bool;
//
//     fn box_get_mut_children(&mut self) -> Option<&mut Vec<Box<dyn HtmlDomNode>>>;
}

impl<T> HtmlDomNodeClone for T
where
    T: HtmlDomNode + Clone + 'static,
{
    fn clone_html_dom_node(&self) -> Box<dyn HtmlDomNode> {
        Box::new(self.clone())
    }
    // fn box_get_classes(&self) -> &Vec<String> {
    //     self.get_classes()
    // }
    // fn box_get_mut_classes(&mut self) -> &mut Vec<String> {
    //     self.get_mut_classes()
    // }
    // fn box_set_classes(&mut self, _classes: Vec<String>) {
    //     self.set_classes(_classes)
    // }
    //
    // fn box_get_height(&self) -> f64 {
    //     self.get_height()
    // }
    // fn box_set_height(&mut self, _height: f64) {
    //     self.set_height(_height)
    // }
    //
    // fn box_get_depth(&self) -> f64 {
    //     self.get_depth()
    // }
    // fn box_set_depth(&mut self, _depth: f64) {
    //     self.set_depth(_depth)
    // }
    //
    // fn box_get_max_font_size(&self) -> f64 {
    //     self.get_max_font_size()
    // }
    // fn box_set_max_font_size(&mut self, _max_font_size: f64) {
    //     self.set_max_font_size(_max_font_size)
    // }
    //
    // fn box_get_style(&self) -> &CssStyle {
    //     self.get_style()
    // }
    // fn box_get_mut_style(&mut self) -> &mut CssStyle {
    //     self.get_mut_style()
    // }
    // fn box_set_style(&mut self, _style: CssStyle) {
    //     self.set_style(_style)
    // }
    //
    // fn box_has_class(&self, class_name: &String) -> bool {
    //     self.has_class(class_name)
    // }
    // fn box_get_mut_children(&mut self) -> Option<&mut Vec<Box<dyn HtmlDomNode>>> {
    //     self.get_mut_children()
    // }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn HtmlDomNode> {
    fn clone(&self) -> Box<dyn HtmlDomNode> {
        self.clone_html_dom_node()
    }
}

// impl VirtualNode for Box<dyn HtmlDomNode> {
//     fn to_node(&self) -> Node {
//         self.box_to_node()
//     }
//
//     fn to_markup(&self) -> String {
//         self.box_to_markup()
//     }
// }
//
// impl HtmlDomNode for Box<dyn HtmlDomNode> {
//     fn get_classes(&self) -> &Vec<String> {
//         return self.box_get_classes();
//     }
//     fn get_mut_classes(&mut self) -> &mut Vec<String> {
//         return self.box_get_mut_classes();
//     }
//     fn set_classes(&mut self, _classes: Vec<String>) {
//         self.box_set_classes(_classes)
//     }
//
//     fn get_height(&self) -> f64 {
//         return self.box_get_height();
//     }
//     fn set_height(&mut self, _height: f64) {
//         self.box_set_height(_height);
//     }
//
//     fn get_depth(&self) -> f64 {
//         return self.box_get_depth();
//     }
//
//     fn set_depth(&mut self, _depth: f64) {
//         self.box_set_depth(_depth);
//     }
//
//     fn get_max_font_size(&self) -> f64 {
//         return self.box_get_max_font_size();
//     }
//     fn set_max_font_size(&mut self, _max_font_size: f64) {
//         self.box_set_max_font_size(_max_font_size);
//     }
//
//     fn get_style(&self) -> &CssStyle {
//         return self.box_get_style();
//     }
//     fn get_mut_style(&mut self) -> &mut CssStyle {
//         return self.box_get_mut_style();
//     }
//     fn set_style(&mut self, _style: CssStyle) {
//         self.box_set_style(_style);
//     }
//
//     fn has_class(&self, class_name: &String) -> bool {
//         return self.box_has_class(class_name);
//     }
//
//     fn get_mut_children(&mut self) -> Option<&mut Vec<Box<dyn HtmlDomNode>>> {
//         return self.box_get_mut_children();
//     }
// }

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

    fn get_mut_children(&mut self) -> Option<&mut Vec<Box<dyn HtmlDomNode>>>;
}
