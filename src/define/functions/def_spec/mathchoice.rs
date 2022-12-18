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
// @flow


fn chooseMathStyle<'a>(group: &'a parse_node::types::mathchoice, options: &Options) -> &'a Vec<Box<dyn AnyParseNode>> {
    let _display = crate::Style::DISPLAY.read().unwrap().size;
    let _text = crate::Style::TEXT.read().unwrap().size;
    let _script = crate::Style::SCRIPT.read().unwrap().size;
    let _script_script = crate::Style::SCRIPTSCRIPT.read().unwrap().size;

    return match options.get_style().size {
        _display  => &group.display,
        _text  =>& group.text,
        _script  => &group.script,
        _script_script  => &group.scriptscript,
        _ => &group.text
    };
}

fn handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let res = parse_node::types::mathchoice {
        mode: context.parser.mode,
        loc: None,
        display: ord_argument(&args[0]),
        text: ord_argument(&args[1]),
        script: ord_argument(&args[2]),
        scriptscript: ord_argument(&args[3]),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::mathchoice>()
        .unwrap();
    let body = chooseMathStyle(group, &options);
    let elements = HTML::build_expression(body.clone(), options, IsRealGroup::F, (None, None));
    let res = common::make_fragment(elements);
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::mathchoice>()
        .unwrap();
    let body = chooseMathStyle(group, &options);
    return mathML::build_expression_row(body.clone(), options, false);
}


lazy_static! {
    pub static ref MATH_CHOICE: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(4);
        props.set_primitive(true);

        FunctionDefSpec {
            def_type: "mathchoice".to_string(),
            names: vec!["\\mathchoice".to_string()],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}


