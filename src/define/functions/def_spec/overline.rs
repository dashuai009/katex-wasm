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

fn handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let body = args[0].clone();
    let res = parse_node::types::overline {
        mode: context.parser.mode,
        loc: None,
        body,
    };

    return Box::new(res) as Box<dyn AnyParseNode>;
}

pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    // Overlines are handled in the TeXbook pg 443, Rule 9.
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::overline>()
        .unwrap();
    // Build the inner group in the cramped style.
    let inner_group = HTML::build_group(
        Some(group.body.clone()),
        options.having_cramped_style(),
        None,
    );

    // Create the line above the body
    let line = common::make_line_span("overline-line".to_string(), &options, None);

    // Generate the vlist, with the appropriate kerns
    let default_rule_thickness = options.get_font_metrics().defaultRuleThickness;
    let vlist = common::make_vlist(
        VListParam {
            position_type: PositionType::FirstBaseline,
            children: vec![
                VListChild::Elem {
                    elem: inner_group,
                    margin_left: None,
                    margin_right: None,
                    wrapper_classes: None,
                    wrapper_style: None,
                    shift: None,
                },
                VListChild::Kern {
                    size: 3.0 * default_rule_thickness,
                },
                VListChild::Elem {
                    elem: Box::new(line) as Box<dyn HtmlDomNode>,
                    margin_left: None,
                    margin_right: None,
                    wrapper_classes: None,
                    wrapper_style: None,
                    shift: None,
                },
                VListChild::Kern {
                    size: default_rule_thickness,
                },
            ],
            position_data: None,
        },
        options.clone(),
    );

    let res = common::make_span(
        vec!["mord".to_string(), "overline".to_string()],
        vec![Box::new(vlist) as Box<dyn HtmlDomNode>],
        Some(&options),
        Default::default(),
    );
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined mathml_builder");
    // let operator = MathNode::new(
    // "mo".to_string(), [new mathMLTree.TextNode("\u203e".to_string())]);
    // operator.set_attribute("stretchy".to_string(), "true".to_string());
    //
    // let node = MathNode::new(
    // "mover".to_string(),
    // [mathML::build_group(group.body, options), operator]);
    // node.set_attribute("accent".to_string(), "true".to_string());
    //
    // return node;
}

lazy_static! {
    pub static ref OVERLINE: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);

        FunctionDefSpec {
            def_type: "overline".to_string(),
            names: vec!["\\overline".to_string()],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
