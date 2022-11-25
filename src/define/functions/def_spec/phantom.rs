use crate::build::common::{PositionType, VListChild, VListParam};
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

fn phantom_html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::phantom>()
        .unwrap();
    let elements = HTML::build_expression(
        group.body.clone(),
        options.with_phantom(),
        IsRealGroup::F,
        (None, None),
    );

    // \phantom isn't supposed to affect the elements it contains.
    // See "color" for more details.
    let res = common::make_fragment(elements);
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

fn phantom_mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined")
    // let inner = mml.buildExpression(group.body, options);
    // return MathNode::new("mphantom".to_string(), inner);
}

fn phantom_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let body = &args[0];
    let res = parse_node::types::phantom {
        mode: context.parser.mode,
        loc: None,
        body: ord_argument(body),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

lazy_static! {
    pub static ref PHANTOM: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "phantom".to_string(),
            names: vec!["\\phantom".to_string()],
            props,
            handler: phantom_handler_fn,
            html_builder: Some(phantom_html_builder),
            mathml_builder: Some(phantom_mathml_builder),
        }
    });
}

fn hphantom_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let body = &args[0];
    let res = parse_node::types::hphantom {
        mode: context.parser.mode,
        loc: None,
        body: body.clone(),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

fn hphantom_html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::hphantom>()
        .unwrap();
    let mut node = common::make_span(
        vec![],
        vec![HTML::build_group(
            Some(group.body.clone()),
            options.with_phantom(),
            None,
        )],
        None,
        Default::default(),
    );
    node.set_height(0.0);
    node.set_depth(0.0);
    for child in node.get_mut_children().unwrap_or(&mut vec![]).into_iter() {
        child.set_height(0.0);
        child.set_depth(0.0);
    }
    // See smash for comment re: use of makeVList
    node = common::make_vlist(
        VListParam {
            position_type: PositionType::FirstBaseline,
            children: vec![VListChild::Elem {
                elem: Box::new(node) as Box<dyn HtmlDomNode>,
                margin_left: None,
                margin_right: None,
                wrapper_classes: None,
                wrapper_style: None,
                shift: None,
            }],
            position_data: None,
        },
        options.clone(),
    );

    // For spacing, TeX treats \smash as a math group (same spacing as ord).
    let res = common::make_span(
        vec!["mord".to_string()],
        vec![Box::new(node) as Box<dyn HtmlDomNode>],
        Some(&options),
        Default::default(),
    );
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

fn hphantom_mathml_builder(
    _group: Box<dyn AnyParseNode>,
    options: Options,
) -> Box<dyn MathDomNode> {
    panic!("undefined");
    // let inner = mml.buildExpression(ordargument(group.body), options);
    // let phantom = MathNode::new( MathNodeType::Mphantom, inner);
    // let node = MathNode::new("mpadded".to_string(), [phantom]);
    // node.set_attribute("height".to_string(), "0px".to_string());
    // node.set_attribute("depth".to_string(), "0px".to_string());
    // return node;
}

lazy_static! {
    pub static ref HPHANTOM: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "hphantom".to_string(),
            names: vec!["\\hphantom".to_string()],
            props,
            handler: hphantom_handler_fn,
            html_builder: Some(hphantom_html_builder),
            mathml_builder: Some(hphantom_mathml_builder),
        }
    });
}

fn vphantom_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let body = args[0].clone();
    let res = parse_node::types::vphantom {
        mode: context.parser.mode,
        loc: None,
        body,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

fn vphantom_html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::vphantom>()
        .unwrap();
    let inner = common::make_span(
        vec!["inner".to_string()],
        vec![HTML::build_group(
            Some(group.body.clone()),
            options.with_phantom(),
            None,
        )],
        None,
        Default::default(),
    );
    let fix = common::make_span(vec!["fix".to_string()], vec![], None, Default::default());
    let res = common::make_span(vec!["mord".to_string(), "rlap".to_string()], vec![
        Box::new(inner) as Box<dyn HtmlDomNode>,
        Box::new(fix) as Box<dyn HtmlDomNode>,
    ], Some(&options), Default::default());
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

fn vphantom_mathml_builder(
    _group: Box<dyn AnyParseNode>,
    options: Options,
) -> Box<dyn MathDomNode> {
    panic!("undefined");
    // let inner = mml.buildExpression(ordargument(group.body), options);
    // let phantom = MathNode::new("mphantom".to_string(), inner);
    // let node = MathNode::new("mpadded".to_string(), [phantom]);
    // node.set_attribute("width".to_string(), "0px".to_string());
    // return node;
}

lazy_static! {
    pub static ref VPHANTOM: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "vphantom".to_string(),
            names: vec!["\\vphantom".to_string()],
            props,
            handler: vphantom_handler_fn,
            html_builder: Some(vphantom_html_builder),
            mathml_builder: Some(vphantom_mathml_builder),
        }
    });
}
