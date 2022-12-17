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

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::color>()
        .unwrap();
    let elements = HTML::build_expression(
        group.body.clone(),
        options.with_color(group.color.clone()),
        IsRealGroup::F,
        (None, None),
    );

    // \color isn't supposed to affect the type of the elements it contains.
    // To accomplish this, we wrap the results in a fragment, so the inner
    // elements will be able to directly interact with their neighbors. For
    // example, `\color{red}{2 +} 3` has the same spacing as `2 + 3`
    let res = common::make_fragment(elements);
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::color>()
        .unwrap();
    let inner = mathML::build_expression(
        group.body.clone(),
        options.with_color(group.color.clone()),
        false,
    );

    let mut node = MathNode::new(MathNodeType::Mstyle, inner
        .into_iter()
        .map(|x| Box::new(x) as Box<dyn MathDomNode>)
        .collect(), vec![]);

    node.set_attribute("mathcolor".to_string(), group.color.clone());

    return Box::new(node) as Box<dyn MathDomNode>;
}

fn handler_fn_1(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let _color = args[0]
        .as_ref()
        .as_any()
        .downcast_ref::<parse_node::types::color_token>()
        .unwrap();
    let color = _color.color.clone();
    let body = args[1].clone();
    let res = parse_node::types::color {
        mode: context.borrow().parser.mode,
        loc: None,
        color,
        body: ord_argument(&body),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

lazy_static! {
    pub static ref COLOR: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(2);
        props.set_allowed_in_text(true);
        props.set_arg_types(vec![ArgType::color, ArgType::original]);

        FunctionDefSpec {
            def_type: "color".to_string(),
            names: vec!["\\textcolor".to_string()],
            props,
            handler: handler_fn_1,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

fn handler_fn_2(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut ctx = context.borrow_mut();
    let _color = args[0]
        .as_ref()
        .as_any()
        .downcast_ref::<parse_node::types::color_token>()
        .unwrap();
    let color = _color.color.clone();

    // Set macro \current@color in current namespace to store the current
    // color, mimicking the behavior of color.sty.
    // This is currently used just to correctly color a \right
    // that follows a \color command.
    ctx.parser.gullet.macros.set(
        &"\\current@color".to_string(),
        Some(crate::define::macros::public::MacroDefinition::Str(
            color.clone(),
        )),
        false,
    );

    // Parse out the implicit body that should be colored.
    let tmp = {
      let t = context.borrow();
        t.break_on_token_text.clone()
    };
    let body = ctx.parser.parse_expression(true, tmp);

    let res = parse_node::types::color {
        mode: ctx.parser.mode,
        loc: None,
        color,
        body,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

lazy_static! {
    pub static ref COLOR2: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_allowed_in_text(true);
        props.set_arg_types(vec![ArgType::color]);

        FunctionDefSpec {
            def_type: "color".to_string(),
            names: vec!["\\textcolor".to_string()],
            props,
            handler: handler_fn_2,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
