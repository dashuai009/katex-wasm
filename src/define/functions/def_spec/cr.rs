// Row breaks within tabular environments, and line breaks at top level
use crate::build::common::{make_fragment, make_span};
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
use crate::{parse_node, AnyParseNode, HtmlDomNode, make_em, calculate_size};
use std::sync::Mutex;
use crate::types::ArgType;



fn handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let m = if let  Some(_size) = &opt_args[0]{
        let size = _size.as_any().downcast_ref::<parse_node::types::size>().unwrap();
        Some(size.value.clone())
    }else{
        None
    };
    let  newLine =  !context.parser.settings.get_display_mode() ||
        !context.parser.settings.use_strict_behavior("newLineInDisplayMode".to_string(), "In LaTeX, \\\\ or \\newline does nothing in display mode".to_string() );
    return Box::new(parse_node::types::cr{
        mode: context.parser.mode,
        loc: None,
        newLine,
        size: m,
    }) as Box<dyn AnyParseNode>;
}

// The following builders are called only at the top level,
// not within tabular/array environments.

pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group.as_any().downcast_ref::<parse_node::types::cr>().unwrap();
    let mut span = common::make_span(vec!["mspace".to_string()], vec![], Some(&options), Default::default());
    if group.newLine {
        span.get_mut_classes().push("newline".to_string());
        if let Some(s) = &group.size {
            span.get_mut_style().margin_top =
                Some(make_em( calculate_size(s, &options)));
        }
    }
    return Box::new(span) as Box<dyn HtmlDomNode>;
}


pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group.as_any().downcast_ref::<parse_node::types::cr>().unwrap();
    let mut node =  MathNode::new(MathNodeType::Mspace, vec![], vec![]);
    if group.newLine {
        node.set_attribute("linebreak".to_string(), "newline".to_string());
        if let Some(s) = &group.size {
            node.set_attribute("height".to_string(),
                              make_em(calculate_size(s, &options)));
        }
    }
    return Box::new(node) as Box<dyn MathDomNode>;
}

// \DeclareRobustCommand\\{...\@xnewline}
lazy_static! {
    pub static ref CR : Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        props.set_num_optional_args(1);
        props.set_arg_types(vec![ArgType::size]);
        props.set_allowed_in_text(true);

        FunctionDefSpec{
            def_type: "cr".to_string(),
            names: vec!["\\\\".to_string()
            ],
            props,
            handler:handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

