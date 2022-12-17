use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
use crate::define::functions::def_spec::sizing::sizing_group;
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::{color_token, ParseNodeToAny};
use crate::types::StyleStr;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::str::FromStr;
use std::sync::Mutex;

fn handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let tmp_b = {
        let ctx = context.borrow();
        ctx.break_on_token_text.clone()
    };
    let mut ctx = context.borrow_mut();
    // parse out the implicit body
    let body = ctx.parser.parse_expression(true, tmp_b);

    // TODO: Refactor to avoid duplicating styleMap in multiple places (e.g.
    // here and in buildHTML and de-dupe the enumeration of all the styles).
    // $FlowFixMe: The names above exactly match the styles.
    let style = &ctx.func_name[1..(ctx.func_name.len()-5)];
    let res = parse_node::types::styling {
        mode: ctx.parser.mode,
        // Figure out what style to use by pulling out the style from
        // the function name
        loc: None,
        style: StyleStr::from_str(style).unwrap(),
        body,
    };

    return Box::new(res) as Box<dyn AnyParseNode>;
}

pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::styling>()
        .unwrap();
    // Style changes are handled in the TeXbook on pg. 442, Rule 3.
    let new_style = group.style.as_style();
    let new_options = options.having_style(&new_style).with_font("".to_string());
    let res = sizing_group(group.body.clone(), new_options, options);
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::styling>()
        .unwrap();
    // Figure out what style we're changing to.
    let new_style = group.style.as_style();
    let new_options = options.having_style(&new_style);

    let inner = mathML::build_expression(group.body.clone(), new_options, false);

    let mut node = MathNode::new(
        MathNodeType::Mstyle,
        inner
            .into_iter()
            .map(|x| Box::new(x) as Box<dyn MathDomNode>)
            .collect(),
        vec![],
    );

    node.set_attribute(
        "scriptlevel".to_string(),
        match group.style {
            StyleStr::text | StyleStr::display => "0".to_string(),
            StyleStr::script => "1".to_string(),
            StyleStr::scriptscript => "2".to_string(),
        },
    );
    node.set_attribute(
        "displaystyle".to_string(),
        match group.style {
            StyleStr::text => "true".to_string(),
            _ => "false".to_string(),
        },
    );

    return Box::new(node) as Box<dyn MathDomNode>;
}

lazy_static! {
    pub static ref STYLING: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        props.set_allowed_in_text(true);
        props.set_primitive(true);

        FunctionDefSpec {
            def_type: "styling".to_string(),
            names: vec![
                "\\displaystyle".to_string(),
                "\\textstyle".to_string(),
                "\\scriptstyle".to_string(),
                "\\scriptscriptstyle".to_string(),
            ],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
