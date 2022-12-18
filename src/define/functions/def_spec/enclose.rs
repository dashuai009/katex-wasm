use crate::build::common::{PositionType, VListChild, VListParam};
use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::path_node::PathNode;
use crate::dom_tree::span::Span;
use crate::dom_tree::svg_node::SvgNode;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::ParseNodeToAny;
use crate::types::ArgType;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{make_em, parse_node, phase_path, AnyParseNode, HtmlDomNode, VirtualNode};
use std::collections::HashMap;
use std::sync::Mutex;

pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::enclose>()
        .unwrap();
    // \cancel, \bcancel, \xcancel, \sout, \fbox, \colorbox, \fcolorbox, \phase
    // Some groups can return document fragments.  Handle those by wrapping
    // them in a span.
    let mut inner = common::wrap_fragment(
        HTML::build_group(Some(group.body.clone()), options.clone(), None),
        &options,
    );

    let label = &group.label[1..];
    let mut scale = options.sizeMultiplier;
    let mut img;
    let mut imgShift = 0.0;

    // In the LaTeX cancel package, line geometry is slightly different
    // depending on whether the subject is wider than it is tall, or vice versa.
    // We don't know the width of a group, so as a proxy, we test if
    // the subject is a single character. This captures most of the
    // subjects that should get the "tall" treatment.
    let isSingleChar = is_character_box(&group.body);

    if label == "sout" {
        img = common::make_span(
            vec!["stretchy".to_string(), "sout".to_string()],
            vec![],
            None,
            Default::default(),
        );
        img.set_height(options.get_font_metrics().defaultRuleThickness / scale);
        imgShift = -0.5 * options.get_font_metrics().xHeight;
    } else if (label == "phase".to_string()) {
        // Set a couple of dimensions from the steinmetz package.
        let lineWeight = crate::units::calculate_size(
            &crate::units::Measurement {
                number: 0.6,
                unit: "pt".to_string(),
            },
            &options,
        );
        let clearance = crate::units::calculate_size(
            &crate::units::Measurement {
                number: 0.35,
                unit: "ex".to_string(),
            },
            &options,
        );

        // Prevent size changes like \Huge from affecting line thickness
        let newOptions = options.having_base_sizing();
        scale = scale / newOptions.sizeMultiplier;

        let angleHeight = inner.get_height() + inner.get_depth() + lineWeight + clearance;
        // Reserve a left pad for the angle.
        inner.get_mut_style().padding_left = Some(make_em(angleHeight / 2.0 + lineWeight));

        // Create an SVG
        let viewBoxHeight = f64::floor(1000.0 * angleHeight * scale);
        let path = phase_path(viewBoxHeight);
        let svg_node_attr = HashMap::from([
            ("width".to_string(), "400em".to_string()),
            ("height".to_string(), make_em(viewBoxHeight / 1000.0)),
            (
                "viewBox".to_string(),
                format!("0 0 400000 ${viewBoxHeight}"),
            ),
            (
                "preserveAspectRatio".to_string(),
                "xMinYMin slice".to_string(),
            ),
        ]);
        let mut svgNode = SvgNode::new(
            vec![Box::new(PathNode::new("phase".to_string(), Some(path))) as Box<dyn VirtualNode>],
            svg_node_attr,
        );

        // Wrap it in a span with overflow: hidden.
        img = Span::new(
            vec!["hide-tail".to_string()],
            vec![Box::new(svgNode) as Box<dyn HtmlDomNode>],
            Some(options.clone()),
            Default::default(),
        );
        img.get_mut_style().height = Some(make_em(angleHeight));
        imgShift = inner.get_depth() + lineWeight + clearance;
    } else {
        // Add horizontal padding
        if label.contains("cancel") {
            if (!isSingleChar) {
                inner.get_mut_classes().push("cancel-pad".to_string());
            }
        } else if label == "angl".to_string() {
            inner.get_mut_classes().push("anglpad".to_string());
        } else {
            inner.get_mut_classes().push("boxpad".to_string());
        }

        // Add vertical padding
        let mut topPad = 0.0;
        let mut bottomPad = 0.0;
        let mut ruleThickness = 0.0;
        // ref: cancel package: \advance\totalheight2\p@ % "+2"
        if label.contains("box") {
            ruleThickness = f64::max(
                options.get_font_metrics().fboxrule, // default
                options.minRuleThickness,            // User override.
            );
            topPad = options.get_font_metrics().fboxsep
                + (if label == "colorbox" {
                    0.0
                } else {
                    ruleThickness
                });
            bottomPad = topPad;
        } else if (label == "angl".to_string()) {
            ruleThickness = f64::max(
                options.get_font_metrics().defaultRuleThickness,
                options.minRuleThickness,
            );
            topPad = 4.0 * ruleThickness; // gap = 3 × line, plus the line itself.
            bottomPad = f64::max(0.0, 0.25 - inner.get_depth());
        } else {
            topPad = if isSingleChar { 0.2 } else { 0.0 };
            bottomPad = topPad;
        }

        img = crate::stretchy::enclose_span(&inner, label.to_string(), topPad, bottomPad, &options);
        if label.contains("fbox") || label.contains("boxed") || label.contains("fcolorbox") {
            img.get_mut_style().border_style = Some("solid".to_string());
            img.get_mut_style().border_width = Some(make_em(ruleThickness));
        } else if (label == "angl" && ruleThickness != 0.049) {
            img.get_mut_style().border_top_width = Some(make_em(ruleThickness));
            img.get_mut_style().border_right_width = Some(make_em(ruleThickness));
        }
        imgShift = inner.get_depth() + bottomPad;

        if (group.backgroundColor.is_some()) {
            img.get_mut_style().background_color = group.backgroundColor.clone();
            if (group.borderColor.is_some()) {
                img.get_mut_style().border_color = group.borderColor.clone();
            }
        }
    }

    let mut vlist;
    if group.backgroundColor.is_some() {
        vlist = common::make_vlist(
            VListParam {
                position_type: PositionType::IndividualShift,
                children: vec![
                    // Put the color background behind inner;
                    VListChild::Elem {
                        elem: Box::new(img) as Box<dyn HtmlDomNode>,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(imgShift),
                    },
                    VListChild::Elem {
                        elem: inner.clone(),
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(0.0),
                    },
                ],
                position_data: None,
            },
            options.clone(),
        );
    } else {
        let classes = if label.contains("cancel") || label.contains("phase") {
            vec!["svg-align".to_string()]
        } else {
            vec![]
        };
        vlist = common::make_vlist(
            VListParam {
                position_type: PositionType::IndividualShift,
                children: vec![
                    // Write the \cancel stroke on top of inner.
                    VListChild::Elem {
                        elem: inner.clone(),
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(0.0),
                    },
                    VListChild::Elem {
                        elem: Box::new(img) as Box<dyn HtmlDomNode>,
                        margin_left: None,
                        shift: Some(imgShift),
                        wrapper_classes: Some(classes),
                        margin_right: None,
                        wrapper_style: None,
                    },
                ],
                position_data: None,
            },
            options.clone(),
        );
    }

    if label.contains("cancel") {
        // The cancel package documentation says that cancel lines add their height
        // to the expression, but tests show that isn't how it actually works.
        vlist.set_height(inner.get_height());
        vlist.set_depth(inner.get_depth());
    }

    let res = if label.contains("cancel") && !isSingleChar {
        // cancel does not create horiz space for its line extension.
        common::make_span(
            vec!["mord".to_string(), "cancel-lap".to_string()],
            vec![Box::new(vlist) as Box<dyn HtmlDomNode>],
            Some(&options),
            Default::default(),
        )
    } else {
        common::make_span(
            vec!["mord".to_string()],
            vec![Box::new(vlist) as Box<dyn HtmlDomNode>],
            Some(&options),
            Default::default(),
        )
    };
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::enclose>()
        .unwrap();
    let mut fboxsep = 0;
    let mut node = MathNode::new(
        if (group.label.contains("colorbox")) {
            MathNodeType::Mpadded
        } else {
            MathNodeType::Menclose
        },
        vec![mathML::build_group(
            Some(group.body.clone()),
            options.clone(),
        )],
        vec![],
    );
    match group.label.as_str() {
        "\\cancel" => {
            node.set_attribute("notation".to_string(), "updiagonalstrike".to_string());
        }
        "\\bcancel" => {
            node.set_attribute("notation".to_string(), "downdiagonalstrike".to_string());
        }
        "\\phase" => {
            node.set_attribute("notation".to_string(), "phasorangle".to_string());
        }
        "\\sout" => {
            node.set_attribute("notation".to_string(), "horizontalstrike".to_string());
        }
        "\\fbox" => {
            node.set_attribute("notation".to_string(), "box".to_string());
        }
        "\\angl" => {
            node.set_attribute("notation".to_string(), "actuarial".to_string());
        }
        "\\fcolorbox" | "\\colorbox" => {
            // <menclose> doesn't have a good notation option. So use <mpadded>
            // instead. Set some attributes that come included with <menclose>.
            fboxsep =
                (options.get_font_metrics().fboxsep * options.get_font_metrics().ptPerEm) as i32;
            node.set_attribute("width".to_string(), format!("+{}pt", 2 * fboxsep));
            node.set_attribute("height".to_string(), format!("+{}pt", 2 * fboxsep));
            node.set_attribute("lspace".to_string(), format!("{fboxsep}pt")); //
            node.set_attribute("voffset".to_string(), format!("{fboxsep}pt"));
            if (group.label == "\\fcolorbox".to_string()) {
                let thk = f64::max(
                    options.get_font_metrics().fboxrule, // default
                    options.minRuleThickness,            // user override
                );
                node.set_attribute(
                    "style".to_string(),
                    format!(
                        "border: {thk} em solid {}",
                        group.borderColor.as_ref().unwrap()
                    ),
                );
            }
        }
        "\\xcancel" => {
            node.set_attribute(
                "notation".to_string(),
                "updiagonalstrike downdiagonalstrike".to_string(),
            );
        }
        _ => {}
    }
    if (group.backgroundColor.is_some()) {
        node.set_attribute(
            "mathbackground".to_string(),
            group.backgroundColor.clone().unwrap(),
        );
    }
    return Box::new(node) as Box<dyn MathDomNode>;
}

pub fn color_box_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let ct = args[0]
        .as_any()
        .downcast_ref::<parse_node::types::color_token>()
        .unwrap();

    let color = &ct.color;
    let body = args[1].clone();
    let res = parse_node::types::enclose {
        mode: context.parser.mode,
        loc: None,
        label: context.func_name.clone(),
        backgroundColor: Some(color.clone()),
        borderColor: None,
        body,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}
lazy_static! {
    pub static ref COLOR_BOX: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(2);
        props.set_allowed_in_text(true);
        props.set_arg_types(vec![ArgType::color, ArgType::text]);

        FunctionDefSpec {
            def_type: "enclose".to_string(),
            names: vec!["\\colorbox".to_string()],
            props,
            handler: color_box_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

pub fn fcolor_box_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let ct0 = args[0]
        .as_any()
        .downcast_ref::<parse_node::types::color_token>()
        .unwrap();
    let ct1 = args[1]
        .as_any()
        .downcast_ref::<parse_node::types::color_token>()
        .unwrap();
    let borderColor = Some(ct0.color.clone());
    let backgroundColor = Some(ct1.color.clone());
    let body = args[2].clone();
    let res = parse_node::types::enclose {
        mode: context.parser.mode,
        loc: None,
        label: context.func_name.clone(),
        backgroundColor,
        borderColor,
        body,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

lazy_static! {
    pub static ref FCOLOR_BOX: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(3);
        props.set_allowed_in_text(true);
        props.set_arg_types(vec![ArgType::color, ArgType::color, ArgType::text]);

        FunctionDefSpec {
            def_type: "enclose".to_string(),
            names: vec!["\\fcolorbox".to_string()],
            props,
            handler: fcolor_box_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

pub fn fbox_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let res = parse_node::types::enclose {
        mode: context.parser.mode,
        loc: None,
        label: "\\fbox".to_string(),
        backgroundColor: None,
        borderColor: None,
        body: args[0].clone(),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}
lazy_static! {
    pub static ref FBOX: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_allowed_in_text(true);
        props.set_arg_types(vec![ArgType::hbox]);

        FunctionDefSpec {
            def_type: "enclose".to_string(),
            names: vec!["\\fbox".to_string()],
            props,
            handler: fbox_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

pub fn cancel_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let res = parse_node::types::enclose {
        mode: context.parser.mode,
        loc: None,
        label: context.func_name.clone(),
        backgroundColor: None,
        borderColor: None,
        body: args[0].clone(),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}
lazy_static! {
    pub static ref CANCEL: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);

        FunctionDefSpec {
            def_type: "enclose".to_string(),
            names: vec![
                "\\cancel".to_string(),
                "\\bcancel".to_string(),
                "\\xcancel".to_string(),
                "\\sout".to_string(),
                "\\phase".to_string(),
            ],
            props,
            handler: cancel_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

pub fn angl_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let res = parse_node::types::enclose {
        mode: context.parser.mode,
        loc: None,
        label: "\\angl".to_string(),
        backgroundColor: None,
        borderColor: None,
        body: args[0].clone(),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}
lazy_static! {
    pub static ref ANGL: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_allowed_in_text(false);
        props.set_arg_types(vec![ArgType::hbox]);

        FunctionDefSpec {
            def_type: "enclose".to_string(),
            names: vec!["\\angl".to_string()],
            props,
            handler: angl_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
