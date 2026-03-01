use crate::build::HTML::IsRealGroup;
use crate::build::{mathML, HTML, common};
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
use crate::{parse_node, AnyParseNode, HtmlDomNode, types::ArgType};
use std::sync::Mutex;


fn handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let shift = opt_args[0].as_ref().map(|x| { let s = x.as_any().downcast_ref::<parse_node::types::size>().unwrap(); s.value.clone() });
    let width = args[0].as_any().downcast_ref::<parse_node::types::size>().unwrap();// assertNodeType(args[0], "size".to_string());
    let height = args[1].as_any().downcast_ref::<parse_node::types::size>().unwrap();
    let context = ctx.borrow();
    let res = parse_node::types::rule {
        mode: context.parser.mode,
        loc: None,
        shift,
        width: width.value.clone(),
        height: height.value.clone(),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}


fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::rule>()
        .unwrap();
    // Make an empty span for the rule
    let mut rule = common::make_span(vec!["mord".to_string(), "rule".to_string()], vec![], Some(&options), Default::default());

    // Calculate the shift, width, and height of the rule, and account for units
    let width = crate::units::calculate_size(&group.width, &options);
    let height = crate::units::calculate_size(&group.height, &options);
    let shift = if let Some(s) = &group.shift { crate::units::calculate_size(s, &options) } else { 0.0 };

    // Style the rule to the right size
    rule.get_mut_style().border_right_width = Some(crate::units::make_em(width));
    rule.get_mut_style().border_top_width = Some(crate::units::make_em(height));
    rule.get_mut_style().bottom = Some(crate::units::make_em(shift));

    // Record the height and width
    rule.set_width(width);
    rule.set_height(height + shift);
    rule.set_depth(-shift);
    // Font size is the number large enough that the browser will
    // reserve at least `absHeight` space above the baseline.
    // The 1.125 factor was empirically determined
    rule.set_max_font_size(height * 1.125 * options.sizeMultiplier);

    return Box::new(rule);
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined")
    // let width = crate::units::calculate_size(group.width, options);
    // let height = crate::units::calculate_size(group.get_height(), options);
    // let shift = (group.shift) ? crate::units::calculate_size(group.shift, options) : 0;
    // let color = options.color && options.getColor() || "black";
    //
    // let rule = MathNode::new("mspace".to_string());
    // rule.set_attribute("mathbackground".to_string(), color);
    // rule.set_attribute("width".to_string(), crate::units::make_em(width));
    // rule.set_attribute("height".to_string(), crate::units::make_em(height));
    //
    // let wrapper = MathNode::new("mpadded".to_string(), [rule]);
    // if (shift >= 0) {
    //     wrapper.set_attribute("height".to_string(), crate::units::make_em(shift));
    // } else {
    //     wrapper.set_attribute("height".to_string(), crate::units::make_em(shift));
    //     wrapper.set_attribute("depth".to_string(), crate::units::make_em(-shift));
    // }
    // wrapper.set_attribute("voffset".to_string(), crate::units::make_em(shift));
    //
    // return wrapper;
}




lazy_static! {
    pub static ref RULE: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(2);
        props.set_num_optional_args(1);
        props.set_arg_types(vec![ArgType::size,ArgType::size,ArgType::size]);
        props.set_allowed_in_text(true);
        props.set_allowed_in_math(true);

        FunctionDefSpec {
            def_type: "rule".to_string(),
            names: vec!["\\rule".to_string()],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
