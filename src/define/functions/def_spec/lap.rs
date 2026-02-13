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
// @flow
// Horizontal overlap functions

fn lap_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let body = args[0].clone();
    let res = parse_node::types::lap {
        mode: context.parser.mode,
        loc: None,
        alignment: context.func_name[5..].to_string(),
        body,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::lap>()
        .unwrap();
    // mathllap, mathrlap, mathclap
    let mut inner;
    if (group.alignment == "clap".to_string()) {
        // ref: https://www.math.lsu.edu/~aperlis/publications/mathclap/
        inner = common::make_span(
            vec![],
            vec![HTML::build_group(Some(group.body.clone()), options.clone(), None)],
            None,
            Default::default(),
        );
        // wrap, since CSS will center a .clap > .inner > span
        inner = common::make_span(
            vec!["inner".to_string()],
            vec![Box::new(inner) as Box<dyn HtmlDomNode>],
            Some(&options),
            Default::default(),
        );
    } else {
        inner = common::make_span(
            vec!["inner".to_string()],
            vec![HTML::build_group(Some(group.body.clone()), options.clone(), None)],
            None,
            Default::default(),
        );
    }
    let fix = common::make_span(vec!["fix".to_string()], vec![], None, Default::default());
    let mut node = common::make_span(
        vec![group.alignment.clone()],
        vec![
            Box::new(inner) as Box<dyn HtmlDomNode>,
            Box::new(fix) as Box<dyn HtmlDomNode>,
        ],
        Some(&options),
        Default::default(),
    );

    // At this point, we have correctly set horizontal alignment of the
    // two items involved in the lap.
    // Next, use a strut to set the height of the HTML bounding box.
    // Otherwise, a tall argument may be misplaced.
    // This code resolved issue #1153
    let mut strut = common::make_span(vec!["strut".to_string()], vec![], None, Default::default());
    strut.get_mut_style().height =
        Some(crate::units::make_em(node.get_height() + node.get_depth()));
    if node.get_depth() != 0.0 {
        strut.get_mut_style().vertical_align = Some(crate::units::make_em(-node.get_depth()));
    }
    node.get_mut_children().unwrap().insert(0,Box::new(strut));

    // Next, prevent vertical misplacement when next to something tall.
    // This code resolves issue #1234
    node = common::make_span(
        vec!["thinbox".to_string()],
        vec![Box::new(node) as Box<dyn HtmlDomNode>],
        Some(&options),
        Default::default(),
    );
    let res = common::make_span(
        vec!["mord".to_string(), "vbox".to_string()],
        vec![Box::new(node) as Box<dyn HtmlDomNode>],
        Some(&options),
        Default::default(),
    );
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::lap>()
        .unwrap();
    // mathllap, mathrlap, mathclap
    let mut node = MathNode::new(
        MathNodeType::Mpadded,
        vec![mathML::build_group(Some(group.body.clone()), options)],
        vec![],
    );

    if group.alignment != "rlap".to_string() {
        let offset = if group.alignment == "llap" {
            "-1".to_string()
        } else {
            "-0.5".to_string()
        };
        node.set_attribute("lspace".to_string(), format!("{}width", offset));
    }
    node.set_attribute("width".to_string(), "0px".to_string());

    return Box::new(node) as Box<dyn MathDomNode>;
}

lazy_static! {
    pub static ref LAP: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "lap".to_string(),
            names: vec![
                "\\mathllap".to_string(),
                "\\mathrlap".to_string(),
                "\\mathclap".to_string(),
            ],
            props,
            handler: lap_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
