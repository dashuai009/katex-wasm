use crate::build::common::make_span;
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
use crate::{parse_node, AnyParseNode, HtmlDomNode, types::ArgType};
use std::sync::Mutex;

fn options_with_font(group: &parse_node::types::text, options: &Options) -> Options {
    // Checks if the argument is a font family or a font style.
    if let Some(font) = &group.font {
        return match font.as_str() {
            //text font famliy  // Non-mathy text, possibly in a font
            "\\textrm" | "\\textnormal" => options.with_text_font_family("textrm".to_string()),
            "\\textsf" => options.with_text_font_family("textsf".to_string()),
            "\\texttt" => options.with_text_font_family("texttt".to_string()),
            // text font weight
            "\\textbf" => options.with_text_font_weight("textbf".to_string()),
            "\\textmd" => options.with_text_font_weight("textmd".to_string()),
            "\\textit" => options.with_text_font_shape(Some("textit".to_string())),
            "\\textup" => options.with_text_font_shape(Some("textup".to_string())),
            &_ => options.with_text_font_shape(None),
        };
    } else {
        options.clone()
    }
}

pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::text>()
        .unwrap();
    let new_options = options_with_font(group, &options);
    let inner = HTML::build_expression(
        group.body.clone(),
        new_options.clone(),
        IsRealGroup::T,
        (None, None),
    );
    let res = common::make_span(
        vec!["mord".to_string(), "text".to_string()],
        inner,
        Some(&new_options),
        Default::default(),
    );
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::text>()
        .unwrap();
    let new_options = options_with_font(group, &options);
    return mathML::build_expression_row(group.body.clone(), new_options, false);
}

pub fn text_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let body = &args[0];
    let res = parse_node::types::text {
        mode: context.parser.mode,
        loc: None,
        body: ord_argument(body),
        font: Some(context.func_name.clone()),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}
lazy_static! {
    pub static ref TEXT : Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_arg_types(vec![ArgType::text ]);
        props.set_allowed_in_argument(true);
        props.set_allowed_in_text(true);

        FunctionDefSpec{
            def_type: "text".to_string(),
            names: vec![
            // Font families
            "\\text".to_string(), "\\textrm".to_string(), "\\textsf".to_string(), "\\texttt".to_string(), "\\textnormal".to_string(),
            // Font weights
            "\\textbf".to_string(), "\\textmd".to_string(),
            // Font Shapes
            "\\textit".to_string(), "\\textup".to_string(),
        ],
            props,
            handler:text_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
