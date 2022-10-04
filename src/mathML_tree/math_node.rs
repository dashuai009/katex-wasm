use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::units::make_em;
use crate::utils::escape;
use crate::Options::Options;
use crate::{path_get, scriptFromCodepoint, HtmlDomNode, VirtualNode};
use js_sys::Array;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
/**
 * This node represents a general purpose MathML node of any type. The
 * constructor requires the type of node to create (for example, `"mo"` or
 * `"mspace"`, corresponding to `<mo>` and `<mspace>` tags).
 */
#[derive(Clone)]
pub struct MathNode {
    node_type: MathNodeType,
    attributes: HashMap<String, String>,
    children: Vec<Box<dyn MathDomNode>>,
    classes: Vec<String>,
}

impl MathNode {
    pub fn new(
        t: MathNodeType,
        children: Vec<Box<dyn MathDomNode>>,
        classes: Vec<String>,
    ) -> MathNode {
        MathNode {
            node_type: t,
            attributes: HashMap::new(),
            children,
            classes,
        }
    }

    /**
     * Sets an attribute on a MathML node. MathML depends on attributes to convey a
     * semantic content, so this is used heavily.
     */
    pub fn set_attribute(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }

    /**
     * Gets an attribute on a MathML node.
     */
    pub fn get_attribute(&self, name: &String) -> Option<&String> {
        return self.attributes.get(name);
    }

    pub fn get_node_type(&self) -> MathNodeType {
        self.node_type.clone()
    }
    pub fn set_node_type(&mut self, t: MathNodeType) {
        self.node_type = t;
    }
}
impl VirtualNode for MathNode {
    /**
     * Converts the math node into a MathML-namespaced DOM element.
     */
    fn to_node(&self) -> web_sys::Node {
        let document = web_sys::window().expect("").document().expect("");
        let mut node = document
            .create_element_ns(
                Some("http://www.w3.org/1998/Math/MathML"),
                self.node_type.as_str(),
            )
            .expect("");

        // Apply attributes
        for (k, v) in self.attributes.iter() {
            web_sys::Element::set_attribute(&node, k.as_str(), v.as_str());
        }

        // Apply the class
        web_sys::Element::set_attribute(&node, "className", self.classes.join(" ").as_str());

        for child in self.children.iter() {
            node.append_child(&child.to_node());
        }
        return web_sys::Node::from(node);
    }

    /**
     * Converts the math node into an HTML markup string.
     */
    fn to_markup(&self) -> String {
        let tag_name = self.node_type.as_str();
        let mut markup = format!("<{}", tag_name);
        // Add the attributes
        for (k, v) in self.attributes.iter() {
            markup.push_str(&format!(" {}={}", k, escape(&v)));
        }
        // Add the class
        if self.classes.len() > 0 {
            let cl = self.classes.join(" ");

            markup.push_str(&format!(" class=\"{}\"", escape(&cl)));
        }

        markup.push_str(">");

        // Add the markup of the children, also as markup
        for child in self.children.iter() {
            markup.push_str(&child.to_markup());
        }

        markup.push_str(&format!("</{}>", tag_name));

        markup
    }
}
impl MathDomNode for MathNode {
    /**
     * Converts the math node into a string, similar to innerText, but escaped.
     */
    fn to_text(&self) -> String {
        return self
            .children
            .iter()
            .map(|child| child.to_text())
            .collect::<Vec<String>>()
            .join(" ");
    }
}
