use crate::build::common::{PositionType, VListParam, VListChild};
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

fn handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let a0 = args[0]
        .as_any()
        .downcast_ref::<parse_node::types::size>()
        .unwrap();
    let amount = a0.value.clone();
    let body = args[1].clone();
    let res = parse_node::types::raisebox {
        mode: context.parser.mode,
        loc: None,
        dy: amount,
        body,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::raisebox>()
        .unwrap();
    let body = HTML::build_group(Some(group.body.clone()), options.clone(), None);
    let dy = crate::units::calculate_size(&group.dy, &options);
    let res = common::make_vlist(
        VListParam {
            position_type: PositionType::Shift,
            position_data: Some(-dy),
            children: vec![VListChild::Elem { elem: body, margin_left: None, margin_right: None, wrapper_classes: None, wrapper_style: None, shift: None }],
        },
        options,
    );
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined")
    // let node = MathNode::new(
    // "mpadded".to_string(), [mathML::build_group(group.body, options)]);
    // let dy = group.dy.number + group.dy.unit;
    // node.set_attribute("voffset".to_string(), dy);
    // return node;
}

// Box manipulation
lazy_static! {
    pub static ref RAISEBOX: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(2);
        props.set_arg_types(vec![ArgType::size  , ArgType::hbox]);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "raisebox".to_string(),
            names: vec!["\\raisebox".to_string()],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
