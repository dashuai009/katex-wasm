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
use crate::types::Mode;

// Horizontal spacing commands

fn handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut context = ctx.borrow_mut();
    let size = args[0].as_any().downcast_ref::<parse_node::types::size>().unwrap();
    if /*parser.settings.strict*/ true {
        let math_function = (context.func_name.chars().nth(1) == Some('m'));  // \mkern, \mskip
        let mu_unit = (size.value.unit == "mu");
        if math_function {
            if !mu_unit {
                context.parser.settings.report_nonstrict("mathVsTextUnits",
                                                         format!("LaTeX's {} supports only mu units,  not {} units", context.func_name, size.value.unit).as_str(), None);
            }
            if context.parser.mode != Mode::math {
                context.parser.settings.report_nonstrict("mathVsTextUnits",
                                                        &format!("LaTeX's {} works only in math mode",context.func_name),None);
            }
        } else {  // !math_function
            if mu_unit {
                context.parser.settings.report_nonstrict("mathVsTextUnits",
                                                        &format!("LaTeX's {} doesn't support mu units",context.func_name),None);
            }
        }
    }
    let res = parse_node::types::kern {
        mode: context.parser.mode,
        loc: None,
        dimension: size.value.clone(),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}


fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::kern>()
        .unwrap();
    let res = common::make_glue(&group.dimension, &options);
    return Box::new(res) as Box<dyn HtmlDomNode>;
}


fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined");
// let dimension = crate::units::calculate_size(group.dimension, options);
// return new mathMLTree.SpaceNode(dimension);
}


// TODO: \hskip and \mskip should support plus and minus in lengths

lazy_static! {
    pub static ref KERN: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_arg_types(vec![ArgType::size]);
        props.set_primitive(true);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "kern".to_string(),
            names: vec!["\\kern".to_string(), "\\mkern".to_string(), "\\hskip".to_string(), "\\mskip".to_string()],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}


