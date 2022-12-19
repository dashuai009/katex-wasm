// Horizontal overlap functions
use crate::build::common::{make_span, PositionType, VListChild, VListParam};
use crate::build::mathML;
use crate::build::HTML::IsRealGroup;
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::Options::Options;
use crate::{build, parse_node, stretchy, AnyParseNode, HtmlDomNode};
use std::sync::Mutex;

pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::accentUnder>()
        .unwrap();
    // Treat under accents much like underlines.
    let inner_group =
        crate::build::HTML::build_group(Some(group.base.clone()), options.clone(), None);

    let accent_body = stretchy::svg_span(
        Box::new(group.clone()) as Box<dyn AnyParseNode>,
        options.clone(),
    );
    let kern = if group.label == "\\utilde" { 0.12 } else { 0.0 };

    // Generate the vlist, with the appropriate kerns
    let vlist = Box::new(build::common::make_vlist(
        VListParam {
            position_type: PositionType::Top,
            position_data: Some(inner_group.get_height()),
            children: vec![
                build::common::VListChild::Elem {
                    elem: Box::new(accent_body) as Box<dyn HtmlDomNode>,
                    margin_left: None,
                    margin_right: None,
                    wrapper_classes: Some(vec!["svg-align".to_string()]),
                    wrapper_style: None,
                    shift: None,
                },
                VListChild::Kern { size: kern },
                VListChild::Elem {
                    elem: inner_group,
                    margin_left: None,
                    margin_right: None,
                    wrapper_classes: None,
                    wrapper_style: None,
                    shift: None,
                },
            ],
        }
    )) as Box<dyn HtmlDomNode>;
    return Box::new(build::common::make_span(
        vec!["mord".to_string(), "accentunder".to_string()],
        vec![vlist],
        Some(&options),
        Default::default(),
    )) as Box<dyn HtmlDomNode>;
}
pub fn handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let base = &args[0];
    return Box::new(parse_node::types::accentUnder {
        mode: context.parser.mode,
        loc: None,
        label: context.func_name.clone(),
        isStretchy: false,
        isShifty: false,
        base: base.clone(),
    }) as Box<dyn AnyParseNode>;
}
pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::accentUnder>()
        .unwrap();
    let accent_node = stretchy::math_ml_node(&group.label);
    let mut node = MathNode::new(
        MathNodeType::Munder,
        vec![
            mathML::build_group(Some(group.base.clone()), options),
            Box::new(accent_node) as Box<dyn MathDomNode>,
        ],
        vec![],
    );
    node.set_attribute("accentunder".to_string(), "true".to_string());
    return Box::new(node) as Box<dyn MathDomNode>;
}

lazy_static! {
    pub static ref ACCENTUNDER: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);

        FunctionDefSpec {
            def_type: "accentUnder".to_string(),
            names: vec![
                "\\underleftarrow".to_string(),
                "\\underrightarrow".to_string(),
                "\\underleftrightarrow".to_string(),
                "\\undergroup".to_string(),
                "\\underlinesegment".to_string(),
                "\\utilde".to_string(),
            ],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
