use crate::build::HTML::IsRealGroup;
use crate::build::{mathML, HTML, common};
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
use crate::{parse_node, AnyParseNode, HtmlDomNode, types::ArgType};
use std::sync::Mutex;

fn handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let ctx = context.borrow();
    let res = parse_node::types::htmlmathml {
        mode: ctx.parser.mode,
        loc: None,
        html: ord_argument(&args[0]),
        mathml: ord_argument(&args[1]),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::htmlmathml>()
        .unwrap();
    let elements = HTML::build_expression(group.html.clone(), options, IsRealGroup::F, (None, None));
    let res = common::make_fragment(elements);
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::htmlmathml>()
        .unwrap();
    return mathML::build_expression_row(group.mathml.clone(), options, false);
}

lazy_static! {
    pub static ref HTML_MATHML: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(2);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "htmlmathml".to_string(),
            names: vec!["\\html@mathml".to_string()],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
