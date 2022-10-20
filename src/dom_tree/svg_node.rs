use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::line_node::LineNode;
use crate::dom_tree::path_node::PathNode;
use crate::utils::{escape};
use crate::units::make_em;
use crate::{HtmlDomNode, path_get, scriptFromCodepoint, VirtualNode};
use js_sys::Array;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

/**
 * SVG nodes are used to render stretchy wide elements.
 */
#[derive(Clone, Debug)]
pub struct SvgNode {
    children: Vec<Box<dyn VirtualNode>>,
    attributes: HashMap<String, String>,
}

impl SvgNode{
    pub fn new(children:Vec<Box<dyn VirtualNode>>, attributes: HashMap<String,String>)->SvgNode{
        SvgNode{
            children,
            attributes
        }
    }
    pub fn set_attributes(&mut self,k:String,v:String)->&mut Self{
        self.attributes.insert(k,v);
        self
    }
}
impl VirtualNode for SvgNode {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn to_node(&self) -> web_sys::Node {
        let svgNS = "http://www.w3.org/2000/svg";

        let document = web_sys::window().expect("").document().expect("");
        let node = document.create_element_ns(Some(svgNS), "svg").expect("");

        // Apply attributes
        for (k, v) in self.attributes.iter() {
            web_sys::Element::set_attribute(&node, k.as_str(), v.as_str());
        }
        for child in self.children.iter() {
            node.append_child(&child.to_node());
            // if let Some(l) = child.downcast_ref::<LineNode>() {
            //     node.append_child(&l.to_node());
            // } else if let Some(p) = child.downcast_ref::<LineNode>() {
            //     node.append_child(&p.to_node());
            // } else {
            //     //error
            // }
        }

        return web_sys::Node::from(node);
    }

    fn to_markup(&self) -> String {
        let mut markup = "<svg xmlns=\"http://www.w3.org/2000/svg\"".to_string();

        for (k, v) in self.attributes.iter() {
            markup.push_str(format!(" {}='{}'", k, v).as_str());
        }

        markup.push_str(">");

        for child in self.children.iter() {
            markup.push_str(&child.to_markup());
            // if let Some(l) = child.downcast_ref::<LineNode>() {
            //     markup.push_str(&l.to_markup());
            // } else if let Some(p) = child.downcast_ref::<LineNode>() {
            //     markup.push_str(&p.to_markup());
            // } else {
            //     //error
            // }
        }
        markup.push_str("</svg>");

        return markup;
    }
}


impl HtmlDomNode for SvgNode{
    fn get_classes(&self) -> &Vec<String> {
        todo!()
    }

    fn get_mut_classes(&mut self) -> &mut Vec<String> {
        todo!()
    }

    fn set_classes(&mut self, _classes: Vec<String>) {
        todo!()
    }

    fn get_height(&self) -> f64 {
        todo!()
    }

    fn set_height(&mut self, _height: f64) {
        todo!()
    }

    fn get_depth(&self) -> f64 {
        todo!()
    }

    fn set_depth(&mut self, _depth: f64) {
        todo!()
    }

    fn get_max_font_size(&self) -> f64 {
        todo!()
    }

    fn set_max_font_size(&mut self, _max_font_size: f64) {
        todo!()
    }

    fn get_style(&self) -> &CssStyle {
        todo!()
    }

    fn get_mut_style(&mut self) -> &mut CssStyle {
        todo!()
    }

    fn set_style(&mut self, _style: CssStyle) {
        todo!()
    }

    fn has_class(&self, class_name: &String) -> bool {
        todo!()
    }

    fn get_children(&self) -> Option<&Vec<Box<dyn HtmlDomNode>>> {
        todo!()
    }

    fn get_mut_children(&mut self) -> Option<&mut Vec<Box<dyn HtmlDomNode>>> {
        todo!()
    }
}