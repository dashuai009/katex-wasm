use std::any::Any;
use crate::dom_tree::css_style::CssStyle;
use crate::mathML_tree::public::MathDomNode;
use crate::{HtmlDomNode, VirtualNode};
use web_sys::Node;

/**
 * This node represents a document fragment, which contains elements, but when
 * placed into the DOM doesn't have any representation itself. It only contains
 * children and doesn't have any DOM node properties.
 */
#[derive(Clone)]
pub struct DocumentFragment {
    children: Vec<Box<dyn HtmlDomNode>>,
    // HtmlDomNode
    classes: Vec<String>,
    height: f64,
    depth: f64,
    max_font_size: f64,
    style: CssStyle, // Never used; needed for satisfying interface.
}

impl DocumentFragment {
    pub fn new(children: Vec<Box<dyn HtmlDomNode>>) -> DocumentFragment {
        DocumentFragment {
            children,
            classes: Vec::new(),
            height: 0.0,
            depth: 0.0,
            max_font_size: 0.0,
            style: CssStyle::default(),
        }
    }
}

impl HtmlDomNode for DocumentFragment {
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
        return self.height.clone();
    }
    fn set_height(&mut self, _height: f64) {
        self.height = _height;
    }

    fn get_depth(&self) -> f64 {
        return self.depth.clone();
    }

    fn set_depth(&mut self, _depth: f64) {
        self.depth = _depth;
    }

    fn get_max_font_size(&self) -> f64 {
        return self.max_font_size.clone();
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

    fn get_mut_children(&mut self) -> Option<&mut Vec<Box<dyn HtmlDomNode>>> {
        return None;
    }
}

impl VirtualNode for DocumentFragment  {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    /** Convert the fragment into a node. */
    fn to_node(&self) -> web_sys::Node {
        let document = web_sys::window().expect("").document().expect("");
        let frag = document.create_document_fragment();
        for child in self.children.iter() {
            frag.append_child(&child.to_node());
        }
        return web_sys::Node::from(frag);
    }

    /** Convert the fragment into HTML markup. */
    fn to_markup(&self) -> String {
        let mut markup = String::new();
        // Simply concatenate the markup for the children together.
        for child in self.children.iter() {
            markup.push_str(child.to_markup().as_str());
        }
        return markup;
    }
}

// impl<ChildType: VirtualNode + MathDomNode + Clone + 'static> MathDomNode
//     for DocumentFragment
// {
//     /**
//      * Converts the math node into a string, similar to innerText. Applies to
//      * MathDomNode's only.
//      */
//     fn to_text(self: &DocumentFragment<ChildType>) -> String {
//         // To avoid this, we would subclass documentFragment separately for
//         // MathML, but polyfills for subclassing is expensive per PR 1469.
//         // $FlowFixMe: Only works for ChildType = MathDomNode.
//         //const toText = (child: ChildType): string => child.toText();
//         return self
//             .children
//             .iter()
//             .map(|child| child.to_text())
//             .collect::<Vec<String>>()
//             .join("");
//     }
// }
