use crate::build::HTML::IsRealGroup;
use crate::build::{mathML, HTML, common};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::{internal, ParseNodeToAny};
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
    let res = parse_node::types::internal{
        mode: ctx.parser.mode,
        loc:None
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}
lazy_static! {
    pub static ref RULE: Mutex<FunctionDefSpec> = Mutex::new({
    let mut props = FunctionPropSpec::new();
    props.set_num_args(0);
    props.set_allowed_in_text(true);

FunctionDefSpec {
    def_type: "internal".to_string(),
        names: vec!["\\relax".to_string()],
        props,
        handler: handler_fn,
        html_builder: None,
        mathml_builder: None,
}
});
}

