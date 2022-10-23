use std::any::Any;
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
use crate::types::Mode;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, stretchy, types::ArgType, AnyParseNode, HtmlDomNode};
use std::sync::Mutex;
// @flow

// NOTE: Unlike most `html_builder`s, this one handles not only "horizBrace".to_string(), but
// also "supsub" since an over/underbrace can affect super/subscripting.
fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let style = options.get_style();

    // Pull out the `ParseNode<"horizBrace">` if `grp` is a "supsub" node.
    let mut _supSubGroup = None;
    let group =
        if let Some(supsub_node) = _group.as_any().downcast_ref::<parse_node::types::supsub>() {
            // Ref: LaTeX source2e: }}}}\limits}
            // i.e. LaTeX treats the brace similar to an op and passes it
            // with \limits, so we need to assign supsub style.
            let tmp = if let Some(sup) = &supsub_node.sup {
                HTML::build_group(
                    Some(sup.clone()),
                    options.having_style(&style.sup()),
                    Some(options.clone()),
                )
            } else {
                HTML::build_group(
                    supsub_node.sub.clone(),
                    options.having_style(&style.sub()),
                    Some(options.clone()),
                )
            };
            _supSubGroup = Some(tmp);
            supsub_node
                .base
                .as_ref()
                .unwrap()
                .as_any()
                .downcast_ref::<parse_node::types::horizBrace>()
                .unwrap()
        } else {
            _group
                .as_any()
                .downcast_ref::<parse_node::types::horizBrace>()
                .unwrap()
        };
    let _display = crate::Style::DISPLAY.read().unwrap();
    // Build the base group
    let body = HTML::build_group(Some(group.base.clone()), options.having_base_style(&_display), None);

    // Create the stretchy element
    let braceBody = stretchy::svg_span(Box::new(group.clone()) as Box<dyn AnyParseNode>, options.clone());

    // Generate the vlist, with the appropriate kerns        ┏━━━━━━━━┓
    // This first vlist contains the content and the brace:   equation
    let mut vlist;
    if group.isOver {
        vlist = common::make_vlist(
            VListParam {
                position_type: PositionType::FirstBaseline,
                children: vec![
                    VListChild::Elem {
                        elem: body,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: None,
                    },
                    VListChild::Kern { size: 0.1 },
                    VListChild::Elem {
                        elem: Box::new(braceBody) as Box<dyn HtmlDomNode>,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: None,
                    },
                ],
                position_data: None,
            },
            options.clone(),
        );
        // $FlowFixMe: Replace this with passing "svg-align" into makeVList.
        vlist.get_mut_children().unwrap()[0]
            .get_mut_children()
            .unwrap()[0]
            .get_mut_children()
            .unwrap()[1]
            .get_mut_classes()
            .push("svg-align".to_string());
    } else {
        vlist = common::make_vlist(
            VListParam {
                position_type: PositionType::Bottom,
                position_data: Some(body.get_depth() + 0.1 + braceBody.get_height()),
                children: vec![
                    VListChild::Elem {
                        elem: Box::new(braceBody) as Box<dyn HtmlDomNode>,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: None,
                    },
                    VListChild::Kern { size: 0.1 },
                    VListChild::Elem {
                        elem: body,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: None,
                    },
                ],
            },
            options.clone(),
        );
        // $FlowFixMe: Replace this with passing "svg-align" into makeVList.
        vlist.get_mut_children().unwrap()[0]
            .get_mut_children()
            .unwrap()[0]
            .get_mut_children()
            .unwrap()[0]
            .get_mut_classes()
            .push("svg-align".to_string());
    }

    if let Some(supSubGroup) = _supSubGroup {
        // To write the supsub, wrap the first vlist in another vlist:
        // They can't all go in the same vlist, because the note might be
        // wider than the equation. We want the equation to control the
        // brace width.

        //      note          long note           long note
        //   ┏━━━━━━━━┓   or    ┏━━━┓     not    ┏━━━━━━━━━┓
        //    equation           eqn                 eqn

        let v_span = common::make_span(
            vec!["mord".to_string(), if group.isOver  {"mover".to_string()}else{ "munder".to_string()}],
            vec![Box::new(vlist) as Box<dyn HtmlDomNode>],
            Some(&options),
            Default::default(),
        );

        if group.isOver {
            vlist = common::make_vlist(
                VListParam {
                    position_type: PositionType::FirstBaseline,
                    children: vec![
                        VListChild::Elem {
                            elem: Box::new(v_span) as Box<dyn HtmlDomNode>,
                            margin_left: None,
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: None,
                        },
                        VListChild::Kern { size: 0.2 },
                        VListChild::Elem {
                            elem: supSubGroup,
                            margin_left: None,
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: None,
                        },
                    ],
                    position_data: None,
                },
                options.clone(),
            );
        } else {
            vlist = common::make_vlist(
                VListParam {
                    position_type: PositionType::Bottom,
                    position_data: Some(v_span.get_depth()
                        + 0.2
                        + supSubGroup.get_height()
                        + supSubGroup.get_depth()),
                    children: vec![
                        VListChild::Elem {
                            elem: supSubGroup,
                            margin_left: None,
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: None,
                        },
                        VListChild::Kern { size: 0.2 },
                        VListChild::Elem {
                            elem: Box::new(v_span) as Box<dyn HtmlDomNode>,
                            margin_left: None,
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: None,
                        },
                    ],
                },
                options.clone(),
            );
        }
    }

    let res = common::make_span(
        vec![
            "mord".to_string(),
            if group.isOver {
                "mover".to_string()
            } else {
                "munder".to_string()
            },
        ],
        vec![Box::new(vlist) as Box<dyn HtmlDomNode>],
        Some(&options),
        Default::default(),
    );
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined mathml builder");
    // let accentNode = stretchy.mathMLnode(group.label);
    // return MathNode::new(
    //     (group.isOver ? "mover" : "munder".to_string()),
    //         [mathML::build_group(group.base, options), accentNode]
    // );
}

fn handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let is_over = context.func_name.starts_with("\\over");
    let res = parse_node::types::horizBrace {
        mode: context.parser.mode,
        label: context.func_name,
        isOver: is_over,
        base: args[0].clone(),
        loc: None,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

lazy_static! {
    pub static ref HORIZBRACE: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        FunctionDefSpec {
            def_type: "horizBrace".to_string(),
            names: vec!["\\overbrace".to_string(), "\\underbrace".to_string()],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
