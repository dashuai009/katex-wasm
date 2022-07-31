use wasm_bindgen::prelude::*;

use crate::{
    dom_tree::css_style::CssStyle,
    mathML_tree::public::{MathDomNode, MathNodeType, ToText},
};
pub trait VirtualNode {
    fn to_node(&self) -> web_sys::Node;
    fn to_markup(&self) -> String;
}

//export interface HtmlDomNode extends VirtualNode {
//     classes: string[];
//     height: number;
//     depth: number;
//     maxFontSize: number;
//     style: CssStyle;
//
//     hasClass(className: string): boolean;
// }
pub trait HtmlDomNode: VirtualNode{
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
}

/**
 * This node represents a document fragment, which contains elements, but when
 * placed into the DOM doesn't have any representation itself. It only contains
 * children and doesn't have any DOM node properties.
 */
pub struct DocumentFragment<ChildType: VirtualNode> {
    children: Vec<ChildType>,
    // HtmlDomNode
    classes: Vec<String>,
    height: f64,
    depth: f64,
    maxFontSize: f64,
    style: CssStyle, // Never used; needed for satisfying interface.
}

impl<ChildType: VirtualNode> DocumentFragment<ChildType> {
    pub fn new(children: Vec<ChildType>) -> DocumentFragment<ChildType> {
        DocumentFragment::<ChildType> {
            children,
            classes: Vec::new(),
            height: 0.0,
            depth: 0.0,
            maxFontSize: 0.0,
            style: CssStyle::default(),
        }
    }
}

impl<ChildType: VirtualNode> DocumentFragment<ChildType> {
    fn has_class(&self, class_name: String) -> bool {
        return self.classes.contains(&class_name);
    }
}

impl<ChildType: VirtualNode> DocumentFragment<ChildType> {
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

impl<ChildType: VirtualNode + MathDomNode> DocumentFragment<ChildType> {
    /**
     * Converts the math node into a string, similar to innerText. Applies to
     * MathDomNode's only.
     */
    fn to_text(&mut self) -> String {
        // To avoid this, we would subclass documentFragment separately for
        // MathML, but polyfills for subclassing is expensive per PR 1469.
        // $FlowFixMe: Only works for ChildType = MathDomNode.
        //const toText = (child: ChildType): string => child.toText();
        return self
            .children
            .iter()
            .map(|child| child.to_text())
            .collect::<Vec<String>>()
            .join("");
    }
}
