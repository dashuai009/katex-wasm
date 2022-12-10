use std::borrow::BorrowMut;
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
use crate::{build, parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::sync::Mutex;
use crate::dom_tree::document_fragment::DocumentFragment;
// @flow

pub fn sizing_group(
    value: Vec<Box<dyn AnyParseNode>>,
    options: Options,
    base_options: Options,
) -> DocumentFragment {
    let mut inner = HTML::build_expression(value, options.clone(), IsRealGroup::F, (None, None));
    let multiplier = options.sizeMultiplier / base_options.sizeMultiplier;

    // Add size-resetting classes to the inner list and set maxFontSize
    // manually. Handle nested size changes.
    for it in inner.iter_mut() {
        if let Some(pos) = it.get_classes().iter().position(|i| i == "sizing") {
            if it.get_mut_classes()[pos+1]  == format!("reset-size{}", options.size) {
                // This is a nested size change: e.g., inner[i] is the "b" in
                // `\Huge a \small b`. Override the old size (the `reset-` class)
                // but not the new size.
                it.get_mut_classes()[pos+1] = format!("reset-size{}", base_options.size);
            }
        } else {
            it.get_mut_classes()
                .append(&mut options.sizing_classes(&base_options));
        }

        it.set_height(it.get_height() * multiplier);
        it.set_depth(it.get_depth() * multiplier);
    }

    return build::common::make_fragment(inner);
}

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::sizing>()
        .unwrap();
    // Handle sizing operators like \Huge. Real TeX doesn't actually allow
    // these functions inside of math expressions, so we do some special
    // handling.
    let new_options = options.having_size(group.size as f64);
    let res = sizing_group(group.body.clone(), new_options, options);
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

lazy_static! {
    static ref SIZE_FUNCS: Vec<String> = {
        vec![
            "\\tiny".to_string(),
            "\\sixptsize".to_string(),
            "\\scriptsize".to_string(),
            "\\footnotesize".to_string(),
            "\\small".to_string(),
            "\\normalsize".to_string(),
            "\\large".to_string(),
            "\\Large".to_string(),
            "\\LARGE".to_string(),
            "\\huge".to_string(),
            "\\Huge".to_string(),
        ]
    };
}
fn handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let tmp = {
        let context = ctx.borrow();
        context.break_on_token_text.clone()
    };
    let mut context = ctx.borrow_mut();
    let body = context.parser.parse_expression(false, tmp);

    let res = parse_node::types::sizing {
        mode: context.parser.mode,
        // Figure out what size to use based on the list of functions above
        loc: None,
        size: if let Some(pos) = SIZE_FUNCS.iter().position(|i| i == &context.func_name) { pos + 1 } else { 0 },
        body,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined")
    // let newOptions = options.havingSize(group.size);
    // let inner = mml.buildExpression(group.body, newOptions);
    //
    // let mut node = MathNode::new("mstyle".to_string(), inner);
    //
    // // TODO(emily): This doesn't produce the correct size for nested size
    // // changes, because we don't keep state of what style we're currently
    // // in, so we can't reset the size to normal before changing it.  Now
    // // that we're passing an options parameter we should be able to fix
    // // this.
    // node.set_attribute("mathsize".to_string(), crate::units::make_em(newOptions.sizeMultiplier));
    //
    // return node;
}

lazy_static! {
    pub static ref SIZING: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "sizing".to_string(),
            names: SIZE_FUNCS.clone(),
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
