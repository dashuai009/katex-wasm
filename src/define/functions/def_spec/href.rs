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


fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group.as_any().downcast_ref::<parse_node::types::href>().unwrap();
    let elements = HTML::build_expression(group.body.clone(), options.clone(), IsRealGroup::F, (None, None));
    let res = common::make_anchor(group.href.clone(), vec![], elements, options);
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined mathml builder");
    // let math = mml.buildExpressionRow(group.body, options);
    // if (!(math instanceof MathNode)) {
    // math = new MathNode("mrow".to_string(), [math]);
    // }
    // math.set_attribute("href".to_string(), group.href);
    // return math;
}

pub fn href_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let body = &args[1];
    let href = args[0].as_any().downcast_ref::<parse_node::types::url>().unwrap();
    //TODO
    // if (!context.parser.settings.isTrusted({
    //     command: "\\href".to_string(),
    //     url: href,
    // })) {
    //     return parser.formatUnsupportedCmd("\\href".to_string());
    // }

    let res = parse_node::types::href{
        mode: context.parser.mode,
        loc: None,
        href:href.url.clone(),
        body: ord_argument(body),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}


lazy_static! {
    pub static ref HREF : Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(2);
        props.set_arg_types(vec![ArgType::url ,ArgType::original]);
        props.set_allowed_in_text(true);

        FunctionDefSpec{
            def_type: "href".to_string(),
            names: vec!["\\href".to_string()],
            props,
            handler:href_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}



pub fn url_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let href = args[0].as_any().downcast_ref::<parse_node::types::url>().unwrap();

    //TODO
    // if (!parser.settings.isTrusted({
    //     command: "\\url".to_string(),
    //     url: href,
    // })) {
    //     return parser.formatUnsupportedCmd("\\url".to_string());
    // }

    let chars = href.url.chars().map(|c|{
        let res = parse_node::types::textord{
            mode: Mode::text,
            loc: None,
            text: if c == '~'{
                "\\textasciitilde".to_string()
            }else{
                c.to_string()
            }
        };
        Box::new(res) as Box<dyn AnyParseNode>
    }).collect::<Vec<_>>();
    let body = parse_node::types::text{
        mode: context.parser.mode,
        font: Some("\\texttt".to_string()),
        body: chars,
        loc: None
    };
    let res =  parse_node::types::href{
        mode: context.parser.mode,
        loc: None,
        href: href.url.clone(),
        body: vec![Box::new(body) as Box<dyn AnyParseNode>],
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}



lazy_static! {
    pub static ref URL : Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_arg_types(vec![ArgType::url ]);
        props.set_allowed_in_text(true);

        FunctionDefSpec{
            def_type: "href".to_string(),
            names: vec!["\\url".to_string()],
            props,
            handler:url_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

