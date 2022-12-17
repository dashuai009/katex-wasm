use crate::build::HTML::IsRealGroup;
use crate::build::{mathML, HTML, common};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::{ParseNodeToAny};
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
    let arg = args[0].as_ref().as_any().downcast_ref::<parse_node::types::ordgroup>().unwrap();
    let group = &arg.body;
    let number = group.iter().map(|x| {
        let y = x.as_any().downcast_ref::<parse_node::types::textord>().unwrap();
        y.text.clone()
    }).collect::<Vec<String>>().concat();//("".to_string());
    let text;
    if let Ok(code) = number.parse() {
        if code < 0 || code >= 0x10ffff {
            panic!("\\@char with invalid code point {number}");
        } else if code <= 0xffff {
            text = std::char::from_u32(code).unwrap().to_string();// String.fromCharCode(code);
        } else { // Astral code point; split into surrogate halves
            let _code = code - 0x10000;
            text = format!("{}{}",char::from_u32((_code >> 10) + 0xd800).unwrap(), char::from_u32( (_code & 0x3ff) + 0xdc00).unwrap());
        }
    } else {
        panic!("\\@char has non-numeric argument {number}");
        // If we drop IE support, the following code could be replaced with
        // text = String.fromCodePoint(code)
    }
    let res = parse_node::types::textord {
        mode: ctx.parser.mode,
        loc: None,
        text: text.to_string(),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}



// \@char is an internal function that takes a grouped decimal argument like
// {123} and converts into symbol with code 123.  It is used by the *macro*
// \char defined in macros.js.
lazy_static! {
    pub static ref CHAR: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "textord".to_string(),
            names: vec!["\\@char".to_string()],
            props,
            handler: handler_fn,
            html_builder: None,
            mathml_builder: None,
        }
    });
}

