use crate::build::mathML::{get_variant, make_text};
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::types::FontVariant;
use crate::Options::Options;
use crate::{parse_node, AnyParseNode, HtmlDomNode};
use std::sync::Mutex;
use crate::define::functions::{FunctionDefSpec, FunctionPropSpec};
use crate::parse_node::types::Atom;

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {    let group = _group
    .as_any()
    .downcast_ref::<parse_node::types::atom>()
    .unwrap();
    return Box::new(crate::build::common::math_sym(
        group.text.clone(),
        group.mode,
        options,
        vec![format!("m{}", group.family.as_str())],
    )) as Box<dyn HtmlDomNode>;
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::atom>()
        .unwrap();
    let mut node = MathNode::new(
        MathNodeType::Mo,
        vec![Box::new(make_text(group.text.clone(), group.mode, None)) as Box<dyn MathDomNode>],
        vec![],
    );
    if group.family == Atom::bin {
        let variant = get_variant(&_group, &options);
        if variant == Some(FontVariant::bold_italic) {
            node.set_attribute("mathvariant".to_string(), variant.unwrap().as_str().to_string());
        }
    } else if group.family == Atom::punct {
        node.set_attribute("separator".to_string(), "true".to_string());
    } else if group.family == Atom::open || group.family == Atom::close {
        // Delims built here should not stretch vertically.
        // See delimsizing.js for stretchy delims.
        node.set_attribute("stretchy".to_string(), "false".to_string());
    }
    return Box::new(node) as Box<dyn MathDomNode>;
}
lazy_static! {
    pub static ref ATOM: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();

        FunctionDefSpec {
            def_type: "atom".to_string(),
            names: vec![],
            props,
            handler: |a, b, c| panic!("error"),
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
