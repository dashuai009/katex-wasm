use crate::build::common::{make_span, make_vlist, PositionType, VListChild, VListParam};
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
use crate::{parse_node, stretchy, AnyParseNode, HtmlDomNode};
use std::sync::Mutex;

// Helper function
fn paddedNode(group: Option<Box<dyn MathDomNode>>) -> MathNode {
    let mut node = MathNode::new(
        MathNodeType::Mpadded,
        if let Some(g) = group { vec![g] } else { vec![] },
        vec![],
    );
    node.set_attribute("width".to_string(), "+0.6em".to_string());
    node.set_attribute("lspace".to_string(), "0.3em".to_string());
    return node;
}

fn handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    return Box::new(parse_node::types::xArrow {
        mode: context.parser.mode,
        loc: None,
        label: context.func_name.clone(),
        body: args[0].clone(),
        below: opt_args[0].clone(),
    }) as Box<dyn AnyParseNode>;
}

// Flow is unable to correctly infer the type of `group`, even though it's
// unamibiguously determined from the passed-in `type` above.
pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::xArrow>()
        .unwrap();
    let style = options.get_style();

    // Build the argument groups in the appropriate style.
    // Ref: amsmath.dtx:   \hbox{$\scriptstyle\mkern#3mu{#6}\mkern#4mu$}%

    // Some groups can return document fragments.  Handle those by wrapping
    // them in a span.
    let mut new_options = options.having_style(&style.sup());
    let mut upper_group = common::wrap_fragment(
        HTML::build_group(Some(group.body.clone()), new_options, Some(options.clone())),
        &options,
    );
    let arrow_prefix = if &group.label[0..2] == "\\x" {
        "x"
    } else {
        "cd"
    };
    upper_group
        .get_mut_classes()
        .push(format!("{arrow_prefix}-arrow-pad"));

    let mut lower_group_tmp = None;
    if let Some(below) = &group.below {
        // Build the lower group
        new_options = options.having_style(&style.sub());
        let mut tmp = common::wrap_fragment(
            HTML::build_group(Some(below.clone()), new_options, Some(options.clone())),
            &options,
        );
        tmp.get_mut_classes()
            .push(format!("{arrow_prefix}-arrow-pad"));
        lower_group_tmp = Some(tmp);
    }

    let arrowBody = stretchy::svg_span(
        Box::new(group.clone()) as Box<dyn AnyParseNode>,
        options.clone(),
    );

    // Re shift: Note that stretchy.svgSpan returned arrowBody.depth = 0.
    // The point we want on the math axis is at 0.5 * arrowBody.height.
    let arrow_shift = -options.get_font_metrics().axisHeight + 0.5 * arrowBody.get_height();
    // 2 mu kern. Ref: amsmath.dtx: #7\if0#2\else\mkern#2mu\fi
    let mut upper_shift =
        -options.get_font_metrics().axisHeight - 0.5 * arrowBody.get_height() - 0.111; // 0.111 em = 2 mu
    if upper_group.get_depth() > 0.25 || group.label == "\\xleftequilibrium" {
        upper_shift -= upper_group.get_depth(); // shift up if depth encroaches
    }

    // Generate the vlist
    let mut vlist;
    if let Some(lowerGroup) = lower_group_tmp {
        let lower_shift = -options.get_font_metrics().axisHeight
            + lowerGroup.get_height()
            + 0.5 * arrowBody.get_height()
            + 0.111;
        vlist = common::make_vlist(
            VListParam {
                position_type: PositionType::IndividualShift,
                children: vec![
                    VListChild::Elem {
                        elem: upper_group,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(upper_shift),
                    },
                    VListChild::Elem {
                        elem: Box::new(arrowBody) as Box<dyn HtmlDomNode>,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(arrow_shift),
                    },
                    VListChild::Elem {
                        elem: lowerGroup,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(lower_shift),
                    },
                ],
                position_data: None,
            },
            options.clone(),
        );
    } else {
        vlist = make_vlist(
            VListParam {
                position_type: PositionType::IndividualShift,
                children: vec![
                    VListChild::Elem {
                        elem: upper_group,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(upper_shift),
                    },
                    VListChild::Elem {
                        elem: Box::new(arrowBody) as Box<dyn HtmlDomNode>,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(arrow_shift),
                    },
                ],
                position_data: None,
            },
            options.clone(),
        );
    }

    vlist.get_mut_children().unwrap()[0]
        .get_mut_children()
        .unwrap()[0]
        .get_mut_children()
        .unwrap()[1]
        .get_mut_classes()
        .push("svg-align".to_string());

    return Box::new(common::make_span(
        vec!["mrel".to_string(), "x-arrow".to_string()],
        vec![Box::new(vlist) as Box<dyn HtmlDomNode>],
        Some(&options),
        Default::default(),
    )) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::xArrow>()
        .unwrap();
    let mut arrowNode = stretchy::math_ml_node(&group.label);
    arrowNode.set_attribute(
        "minsize".to_string(),
        if group.label.chars().nth(0).unwrap() == 'x' {
            "1.75em"
        } else {
            "3.0em"
        }
        .to_string(),
    );
    let node;

    // if (group.body) {
    let upper_node = paddedNode(Some(mathML::build_group(
        Some(group.body.clone()),
        options.clone(),
    )));
    if let Some(below) = &group.below {
        let lower_node = paddedNode(Some(mathML::build_group(
            Some(below.clone()),
            options.clone(),
        )));
        node = MathNode::new(
            MathNodeType::Munderover,
            vec![
                Box::new(arrowNode) as Box<dyn MathDomNode>,
                Box::new(lower_node) as Box<dyn MathDomNode>,
                Box::new(upper_node) as Box<dyn MathDomNode>,
            ],
            vec![],
        );
    } else {
        node = MathNode::new(
            MathNodeType::Mover,
            vec![
                Box::new(arrowNode) as Box<dyn MathDomNode>,
                Box::new(upper_node) as Box<dyn MathDomNode>,
            ],
            vec![],
        );
    }
    // } else if (group.below) {
    //     let lowerNode = paddedNode(mml.buildGroup(group.below, options));
    //     node = MathNode::new(MathNodeType::Munder, [arrowNode, lowerNode]);
    // } else {
    //     // This should never happen.
    //     // Parser.js throws an error if there is no argument.
    //     node = paddedNode(None);
    //     node = MathNode::new("mover", [arrowNode, node]);
    // }
    return Box::new(node) as Box<dyn MathDomNode>;
}

// Stretchy arrows with an optional argument
lazy_static! {
    pub static ref XARROW : Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_num_optional_args(1);

        FunctionDefSpec{
            def_type: "xArrow".to_string(),
            names: vec![
                "\\xleftarrow".to_string(), "\\xrightarrow".to_string(), "\\xLeftarrow".to_string(), "\\xRightarrow".to_string(),
                "\\xleftrightarrow".to_string(), "\\xLeftrightarrow".to_string(), "\\xhookleftarrow".to_string(),
                "\\xhookrightarrow".to_string(), "\\xmapsto".to_string(), "\\xrightharpoondown".to_string(),
                "\\xrightharpoonup".to_string(), "\\xleftharpoondown".to_string(), "\\xleftharpoonup".to_string(),
                "\\xrightleftharpoons".to_string(), "\\xleftrightharpoons".to_string(), "\\xlongequal".to_string(),
                "\\xtwoheadrightarrow".to_string(), "\\xtwoheadleftarrow".to_string(), "\\xtofrom".to_string(),
                // The next 3 functions are here to support the mhchem extension.
                // Direct use of these functions is discouraged and may break someday.
                "\\xrightleftarrows".to_string(), "\\xrightequilibrium".to_string(), "\\xleftequilibrium".to_string(),
                // The next 3 functions are here only to support the {CD} environment.
                "\\\\cdrightarrow".to_string(), "\\\\cdleftarrow".to_string(), "\\\\cdlongequal".to_string()
            ],
            props,
            handler:handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
