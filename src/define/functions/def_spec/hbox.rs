use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::ParseNodeToAny;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::sync::Mutex;
// @flow

pub fn hbox_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let res = parse_node::types::hbox {
        mode: context.parser.mode,
        loc: None,
        body: ord_argument(&args[0]),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::hbox>()
        .unwrap();
    let elements =
        HTML::build_expression(group.body.clone(), options, IsRealGroup::F, (None, None));
    let res = common::make_fragment(elements);
    return Box::new(res) as Box<dyn HtmlDomNode>;
}
pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined mathml builder");
    // return MathNode::new(
    // "mrow".to_string(), mml.buildExpression(group.body, options)
    // );
}

// \hbox is provided for compatibility with LaTeX \vcenter.
// In LaTeX, \vcenter can act only on a box, as in
// \vcenter{\hbox{$\frac{a+b}{\dfrac{c}{d}}$}}
// This function by itself doesn't do anything but prevent a soft line break.

lazy_static! {
    pub static ref HBOX: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_arg_types(vec![ArgType::text]);
        props.set_allowed_in_text(true);
        props.set_primitive(true);

        FunctionDefSpec {
            def_type: "hbox".to_string(),
            names: vec!["\\hbox".to_string()],
            props,
            handler: hbox_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
