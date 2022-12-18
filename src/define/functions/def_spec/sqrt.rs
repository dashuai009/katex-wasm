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


fn sqrt_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let index = opt_args[0].clone();
    let body = args[0].clone();
    let res = parse_node::types::sqrt {
        mode: context.parser.mode,
        loc: None,
        body,
        index,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::sqrt>()
        .unwrap();
    // Square roots are handled in the TeXbook pg. 443, Rule 11.

    // First, we do the same steps as in overline to build the inner group
    // and line
    let mut inner = HTML::build_group(
        Some(group.body.clone()),
        options.having_cramped_style(),
        None,
    );
    //println!("inner = {:#?}", inner);
    if inner.get_height() == 0.0 {
        // Render a small surd.
        inner.set_height(options.get_font_metrics().xHeight);
    }

    // Some groups can return document fragments.  Handle those by wrapping
    // them in a span.
    inner = common::wrap_fragment(inner, &options);

    // Calculate the minimum size for the \surd delimiter
    let metrics = options.get_font_metrics();
    let theta = metrics.defaultRuleThickness;

    let mut phi = theta;
    let _text = crate::Style::TEXT.read().unwrap();
    if options.get_style().id < _text.id {
        phi = options.get_font_metrics().xHeight;
    }

    // Calculate the clearance between the body and line
    let mut line_clearance = theta + phi / 4.0;

    let min_delimiter_height = (inner.get_height() + inner.get_depth() + line_clearance + theta);

    // Create a sqrt SVG of the required minimum size
    let (img, advance_width, rule_width) =
        crate::delimiter::make_sqrt_image(min_delimiter_height, &options);

    let delim_depth = img.get_height() - rule_width;

    // Adjust the clearance based on the delimiter size
    if delim_depth > inner.get_height() + inner.get_depth() + line_clearance {
        line_clearance = (line_clearance + delim_depth - inner.get_height() - inner.get_depth()) / 2.0;
    }

    // Shift the sqrt image
    let img_shift = img.get_height() - inner.get_height() - line_clearance - rule_width;

    inner.get_mut_style().padding_left = Some(crate::units::make_em(advance_width));

    // Overlay the image and the argument.
    let body = common::make_vlist(
        VListParam {
            position_type: PositionType::FirstBaseline,
            children: vec![
                VListChild::Elem {
                    elem: inner.clone(),
                    margin_left: None,
                    margin_right: None,
                    wrapper_classes: Some(vec!["svg-align".to_string()]),
                    wrapper_style: None,
                    shift: None,
                },
                VListChild::Kern {
                    size: -(inner.get_height() + img_shift),
                },
                VListChild::Elem {
                    elem: Box::new(img) as Box<dyn HtmlDomNode>,
                    margin_left: None,
                    margin_right: None,
                    wrapper_classes: None,
                    wrapper_style: None,
                    shift: None,
                },
                VListChild::Kern { size: rule_width },
            ],
            position_data: None,
        },
        options.clone(),
    );

    let res = if let Some(group_index) = &group.index {
        // Handle the optional root index

        // The index is always in scriptscript style
        let _scriptscript = crate::Style::SCRIPTSCRIPT.read().unwrap();
        let new_options = options.having_style(&_scriptscript);
        let rootm = HTML::build_group(Some(group_index.clone()), new_options, Some(options.clone()));

        // The amount the index is shifted by. This is taken from the TeX
        // source, in the definition of `\r@@t`.
        let to_shift = 0.6 * (body.get_height() - body.get_depth());

        // Build a VList with the superscript shifted up correctly
        let root_vlist = common::make_vlist(
            VListParam {
                position_type: PositionType::Shift,
                position_data: Some(-to_shift),
                children: vec![VListChild::Elem {
                    elem: rootm,
                    margin_left: None,
                    margin_right: None,
                    wrapper_classes: None,
                    wrapper_style: None,
                    shift: None,
                }],
            },
            options.clone(),
        );
        // Add a class surrounding it so we can add on the appropriate
        // kerning
        let root_vlist_wrap = common::make_span(
            vec!["root".to_string()],
            vec![Box::new(root_vlist) as Box<dyn HtmlDomNode>],
            None,
            Default::default(),
        );

        common::make_span(
            vec!["mord".to_string(), "sqrt".to_string()],
            vec![
                Box::new(root_vlist_wrap) as Box<dyn HtmlDomNode>,
                Box::new(body) as Box<dyn HtmlDomNode>,
            ],
            Some(&options),
            Default::default(),
        )
    } else {
        common::make_span(
            vec!["mord".to_string(), "sqrt".to_string()],
            vec![Box::new(body) as Box<dyn HtmlDomNode>],
            Some(&options),
            Default::default(),
        )
    };
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::sqrt>()
        .unwrap();
    let res = if group.index.is_some() {
        MathNode::new(
            MathNodeType::Mroot,
            vec![
                mathML::build_group(Some(group.body.clone()), options.clone()),
                mathML::build_group(group.index.clone(), options),
            ],
            vec![],
        )
    } else {
        MathNode::new(
            MathNodeType::Msqrt,
            vec![mathML::build_group(Some(group.body.clone()), options)],
            vec![],
        )
    };
    return Box::new(res) as Box<dyn MathDomNode>;
}

lazy_static! {
    pub static ref SQRT: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_num_optional_args(1);

        FunctionDefSpec {
            def_type: "sqrt".to_string(),
            names: vec!["\\sqrt".to_string()],
            props,
            handler: sqrt_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
