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
use crate::build::common::{PositionType, VListChild, VListParam};

fn handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let res = parse_node::types::vcenter {
        mode: context.borrow().parser.mode,
        loc: None,
        body: args[0].clone(),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::vcenter>()
        .unwrap();
    let body = HTML::build_group(Some(group.body.clone()), options.clone(), None);
    let axis_height = options.get_font_metrics().axisHeight;
    let dy = 0.5 * ((body.get_height() - axis_height) - (body.get_depth() + axis_height));
    let res = common::make_vlist(VListParam {
        position_type: PositionType::Shift,
        position_data: Some(dy),
        children: vec![VListChild::Elem { elem: body, margin_left: None, margin_right: None, wrapper_classes: None, wrapper_style: None, shift: None }],
    });
    return Box::new(res) as Box::<dyn HtmlDomNode>;
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::vcenter>()
        .unwrap();
// There is no way to do this in MathML.
// Write a class as a breadcrumb in case some post-processor wants
// to perform a vcenter adjustment.
    let res = MathNode::new(MathNodeType::Mpadded, vec![mathML::build_group(Some(group.body.clone()), options)], vec!["vcenter".to_string()]);
    return Box::new(res) as Box<dyn MathDomNode>;
}

// \vcenter:  Vertically center the argument group on the math axis.

lazy_static! {
    pub static ref VCENTER: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_arg_types(vec![ArgType::original]);
        props.set_allowed_in_text(false);

        FunctionDefSpec {
            def_type: "vcenter".to_string(),
            names: vec!["\\vcenter".to_string()],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}


