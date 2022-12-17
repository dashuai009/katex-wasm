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
use crate::types::{BreakToken, Mode, StyleStr};
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::str::FromStr;
use std::sync::Mutex;

pub fn handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut context = ctx.borrow_mut();
    let outer_mode = context.parser.mode;
    context.parser.switch_mode(Mode::math);
    let close = if context.func_name == "\\(" {
        "\\)".to_string()
    } else {
        "$".to_string()
    };
    let body = context
        .parser
        .parse_expression(false, BreakToken::from_str(close.as_str()).ok());
    context.parser.expect(close, false);
    context.parser.switch_mode(outer_mode);
    let res = parse_node::types::styling {
        mode: context.parser.mode,
        loc: None,
        style: StyleStr::text,
        body,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

// Switching from text mode back to math mode
lazy_static! {
    pub static ref STYLING: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        props.set_allowed_in_text(true);
        props.set_allowed_in_math(false);

        FunctionDefSpec {
            def_type: "styling".to_string(),// no matter
            names: vec!["\\(".to_string(), "$".to_string()],
            props,
            handler: handler_fn,
            html_builder: None,
            mathml_builder: None,
        }
    });
}

// Check for extra closing math delimiters
lazy_static! {
    pub static ref TEXT: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        props.set_allowed_in_text(true);
        props.set_allowed_in_math(false);

        FunctionDefSpec {
            def_type: "text".to_string(),// Doesn't matter what this is.
            names: vec!["\\)".to_string(), "\\]".to_string()],
            props,
            handler: |a,b,c|{panic!("Mismatched hahha")},
            html_builder: None,
            mathml_builder: None,
        }
    });
}
