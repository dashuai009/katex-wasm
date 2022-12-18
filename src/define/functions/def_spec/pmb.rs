/// \pmb is a simulation of bold font.
/// The version of \pmb in ambsy.sty works by typesetting three copies
/// with small offsets. We use CSS text-shadow.
/// It's a hack. Not as good as a real bold font. Better than nothing.
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
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let res = parse_node::types::pmb {
        mode: context.parser.mode,
        loc: None,
        mclass: super::mclass::binrel_class(&args[0]),
        body: ord_argument(&args[0]),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}
fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::pmb>()
        .unwrap();
    let elements =
        HTML::build_expression(group.body.clone(), options.clone(), IsRealGroup::T, (None, None));
    let mut node = common::make_span(
        vec![group.mclass.clone()],
        elements,
        Some(&options),
        Default::default(),
    );
    node.get_mut_style().text_shadow = Some("0.02em 0.01em 0.04px".to_string());
    return Box::new(node) as Box<dyn HtmlDomNode>;
}
fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undeinfed")
    // let inner = mml.buildExpression(group.body, style);
    // // Wrap with an <mstyle> element.
    // let node = MathNode::new("mstyle".to_string(), inner);
    // node.set_attribute("style".to_string(), "text-shadow: 0.02em 0.01em 0.04px".to_string());
    // return node;
}

lazy_static! {
    pub static ref PMB: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "pmb".to_string(),
            names: vec!["\\pmb".to_string()],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
