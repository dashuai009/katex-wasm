// "mathord" and "textord" ParseNodes created in Parser.js from symbol Groups in
// src/symbols.js.

use crate::build::mathML::{get_variant, make_text};
use crate::define::functions::{FunctionDefSpec, FunctionPropSpec};
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::types::{FontVariant, Mode};
use crate::Options::Options;
use crate::{parse_node, AnyParseNode, HtmlDomNode};
use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;

pub fn mathord_html_builder(
    _group: Box<dyn AnyParseNode>,
    options: Options,
) -> Box<dyn HtmlDomNode> {
    return Box::new(crate::build::common::make_ord(
        _group,
        options,
        "mathord".to_string(),
    )) as Box<dyn HtmlDomNode>;
}

pub fn mathord_mathml_builder(
    _group: Box<dyn AnyParseNode>,
    options: Options,
) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::mathord>()
        .unwrap();
    let mut node = MathNode::new(
        MathNodeType::Mi,
        vec![Box::new(make_text(
            group.text.clone(),
            group.mode,
            Some(&options.clone()),
        )) as Box<dyn MathDomNode>],
        vec![],
    );

    let variant = get_variant(&_group, &options).unwrap_or(FontVariant::italic);
    let default_variant = DEFAULT_VARIANT.lock().unwrap();
    if &variant.as_str() != default_variant.get(node.get_node_type().as_str()).unwrap() {
        node.set_attribute("mathvariant".to_string(), variant.as_str().to_string());
    }
    return Box::new(node) as Box<dyn MathDomNode>;
}

fn textord_mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let variant = get_variant(&_group, &options).unwrap_or(FontVariant::normal);
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::textord>()
        .unwrap();
    let text = make_text(group.text.clone(), group.mode, Some(&options));

    let num = Regex::new("[0-9]").unwrap();
    let node_type = if group.mode == Mode::text {
        MathNodeType::Mtext
    } else if num.is_match(&group.text) {
        MathNodeType::Mn
    } else if group.text == "\\prime" {
        MathNodeType::Mo
    } else {
        MathNodeType::Mi
    };
    let mut node = MathNode::new(
        node_type,
        vec![Box::new(text) as Box<dyn MathDomNode>],
        vec![],
    );

    let default_variant = DEFAULT_VARIANT.lock().unwrap();
    if &variant.as_str() != default_variant.get(node.get_node_type().as_str()).unwrap() {
        node.set_attribute("mathvariant".to_string(), variant.as_str().to_string());
    }

    return Box::new(node) as Box<dyn MathDomNode>;
}
lazy_static! {
    static ref DEFAULT_VARIANT: Mutex<HashMap<&'static str, &'static str>> =
        Mutex::new({ HashMap::from([("mi", "italic"), ("mn", "normal"), ("mtext", "normal"),]) });
    pub static ref MATHORD: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();

        FunctionDefSpec {
            def_type: "mathord".to_string(),
            names: vec![],
            props,
            handler: |a, b, c| panic!("error"),
            html_builder: Some(mathord_html_builder),
            mathml_builder: Some(mathord_mathml_builder),
        }
    });
    pub static ref TEXTORD: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();

        FunctionDefSpec {
            def_type: "textord".to_string(),
            names: vec![],
            props,
            handler: |a, b, c| panic!("error"),
            html_builder: Some(|group, options| -> Box<dyn HtmlDomNode> {
                return Box::new(crate::build::common::make_ord(
                    group,
                    options,
                    "textord".to_string(),
                )) as Box<dyn HtmlDomNode>;
            }),
            mathml_builder: Some(mathord_mathml_builder),
        }
    });
}
