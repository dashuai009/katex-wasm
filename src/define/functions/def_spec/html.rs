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
use crate::settings::TrustContext;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::collections::HashMap;
use std::sync::Mutex;

fn handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let args0 = args[0]
        .as_any()
        .downcast_ref::<parse_node::types::raw>()
        .unwrap();
    let value = &args0.string;
    let body = &args[1];

    if context.parser.settings.get_strict() != "ignore" {
        context.parser.settings.report_nonstrict(
            "htmlExtension",
            "HTML extension is disabled on strict mode",
            None,
        );
    }

    let mut attributes = HashMap::new();

    match context.func_name.as_str() {
        "\\htmlClass" => {
            attributes.insert("class".to_string() , value.clone());
        }
        "\\htmlId" => {
            attributes.insert("id".to_string() , value.clone());
        }
        "\\htmlStyle" => {
            attributes.insert("style".to_string(), value.clone());
        }
        "\\htmlData" => {
            for kv in value.split(',') {
                let key_val = kv.split('=').collect::<Vec<_>>();
                if key_val.len() != 2 {
                    panic!("Error parsing key-value for \\htmlData");
                }
                attributes.insert(format!("data-{}" , key_val[0].trim()) , key_val[1].trim().to_string());
            }
        }
        _ => {
            panic!("Unrecognized html command");
        }
    }
    let trust_context = TrustContext {
        command: context.func_name.clone(),
        context: attributes.clone(),
    };

    if !context.parser.settings.is_trusted(&trust_context) {
        let res = context.parser.format_unsupported_cmd(&context.func_name);
        return Box::new(res) as Box<dyn AnyParseNode>;
    }
    let res = parse_node::types::html {
        mode: context.parser.mode,
        attributes,
        body: ord_argument(body),
        loc: None,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::html>()
        .unwrap();
    let elements = HTML::build_expression(group.body.clone(), options.clone(), IsRealGroup::F, (None, None));

    let mut classes = vec!["enclosing"];
    if let Some(class) = group.attributes.get("class") {
        classes.extend(class.trim().split_whitespace());
    }

    let mut span = common::make_span(classes.into_iter().map(|s|s.to_string()).collect(), elements, Some(&options), Default::default());
    for (a, b) in group.attributes.iter() {
        if a != "class" {
            span.set_attribute(a.clone(), b.clone());
        }
    }
    return Box::new(span) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined mathml builder");
    // return mml.buildExpressionRow(group.body, options);
}

lazy_static! {
    pub static ref HTML_SPEC: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(2);
        props.set_arg_types(vec![ArgType::raw, ArgType::original]);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "html".to_string(),
            names: vec![
                "\\htmlClass".to_string(),
                "\\htmlId".to_string(),
                "\\htmlStyle".to_string(),
                "\\htmlData".to_string(),
            ],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
