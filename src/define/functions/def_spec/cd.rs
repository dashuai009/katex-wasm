use crate::build::{common, mathML, HTML};
use crate::define::functions::public::{FunctionContext, FunctionDefSpec, FunctionPropSpec};
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::ParseNodeToAny;
use crate::units::make_em;
use crate::Options::Options;
use crate::{parse_node, AnyParseNode, HtmlDomNode};
use std::sync::Mutex;

fn padded_node(group: Box<dyn MathDomNode>) -> MathNode {
    let mut node = MathNode::new(MathNodeType::Mpadded, vec![group], vec![]);
    node.set_attribute("width".to_string(), "0".to_string());
    node
}

fn cd_label_handler(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    _opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let side = if context.func_name.ends_with("left") {
        "left"
    } else {
        "right"
    };

    Box::new(parse_node::types::cdlabel {
        mode: context.parser.mode,
        loc: None,
        side: side.to_string(),
        label: args[0].clone(),
    }) as Box<dyn AnyParseNode>
}

fn cd_label_html_builder(group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = group
        .as_any()
        .downcast_ref::<parse_node::types::cdlabel>()
        .unwrap();
    let new_options = options.having_style(&options.get_style().sup());
    let mut label = common::wrap_fragment(
        HTML::build_group(Some(group.label.clone()), new_options, Some(options.clone())),
        &options,
    );
    label
        .get_mut_classes()
        .push(format!("cd-label-{}", group.side));
    label.get_mut_style().bottom = Some(make_em(0.8 - label.get_depth()));
    label.set_height(0.0);
    label.set_depth(0.0);
    label
}

fn cd_label_mathml_builder(
    group: Box<dyn AnyParseNode>,
    options: Options,
) -> Box<dyn MathDomNode> {
    let group = group
        .as_any()
        .downcast_ref::<parse_node::types::cdlabel>()
        .unwrap();
    let mut label = MathNode::new(
        MathNodeType::Mrow,
        vec![mathML::build_group(Some(group.label.clone()), options.clone())],
        vec![],
    );
    label = {
        let mut padded = padded_node(Box::new(label) as Box<dyn MathDomNode>);
        if group.side == "left" {
            padded.set_attribute("lspace".to_string(), "-1width".to_string());
        }
        padded.set_attribute("voffset".to_string(), "0.7em".to_string());
        padded
    };

    let mut styled = MathNode::new(
        MathNodeType::Mstyle,
        vec![Box::new(label) as Box<dyn MathDomNode>],
        vec![],
    );
    styled.set_attribute("displaystyle".to_string(), "false".to_string());
    styled.set_attribute("scriptlevel".to_string(), "1".to_string());
    Box::new(styled) as Box<dyn MathDomNode>
}

fn cd_parent_handler(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    _opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    Box::new(parse_node::types::cdlabelparent {
        mode: context.parser.mode,
        loc: None,
        fragment: args[0].clone(),
    }) as Box<dyn AnyParseNode>
}

fn cd_parent_html_builder(
    group: Box<dyn AnyParseNode>,
    options: Options,
) -> Box<dyn HtmlDomNode> {
    let group = group
        .as_any()
        .downcast_ref::<parse_node::types::cdlabelparent>()
        .unwrap();
    let mut parent = common::wrap_fragment(
        HTML::build_group(Some(group.fragment.clone()), options.clone(), None),
        &options,
    );
    parent.get_mut_classes().push("cd-vert-arrow".to_string());
    parent
}

fn cd_parent_mathml_builder(
    group: Box<dyn AnyParseNode>,
    options: Options,
) -> Box<dyn MathDomNode> {
    let group = group
        .as_any()
        .downcast_ref::<parse_node::types::cdlabelparent>()
        .unwrap();
    Box::new(MathNode::new(
        MathNodeType::Mrow,
        vec![mathML::build_group(Some(group.fragment.clone()), options)],
        vec![],
    )) as Box<dyn MathDomNode>
}

lazy_static! {
    pub static ref CD_LABEL: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);

        FunctionDefSpec {
            def_type: "cdlabel".to_string(),
            names: vec!["\\\\cdleft".to_string(), "\\\\cdright".to_string()],
            props,
            handler: cd_label_handler,
            html_builder: Some(cd_label_html_builder),
            mathml_builder: Some(cd_label_mathml_builder),
        }
    });
    pub static ref CD_LABEL_PARENT: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);

        FunctionDefSpec {
            def_type: "cdlabelparent".to_string(),
            names: vec!["\\\\cdparent".to_string()],
            props,
            handler: cd_parent_handler,
            html_builder: Some(cd_parent_html_builder),
            mathml_builder: Some(cd_parent_mathml_builder),
        }
    });
}
