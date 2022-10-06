use std::any::Any;
use crate::dom_tree::css_style::CssStyle;
use crate::mathML_tree::public::MathDomNode;
use crate::{HtmlDomNode, VirtualNode};
use web_sys::Node;
use struct_format::html_dom_node;

/**
 * This node represents a document fragment, which contains elements, but when
 * placed into the DOM doesn't have any representation itself. It only contains
 * children and doesn't have any DOM node properties.
 */
#[derive(Clone, html_dom_node, Debug)]
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
