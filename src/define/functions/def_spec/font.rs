use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
use crate::define::functions::public::{
    normalize_argument, ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::ParseNodeToAny;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::collections::HashMap;
use std::sync::Mutex;

// TODO(kevinb): implement \\sl and \\sc

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::font>()
        .unwrap();
    let font = &group.font;
    let newOptions = options.with_font(font.clone());
    return HTML::build_group(Some(group.body.clone()), newOptions, None);
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("udnefined")
    // let font = group.font;
    // let newOptions = options.withFont(font);
    // return mathML::build_group(group.body, newOptions);
}

lazy_static! {
    static ref FONT_ALIASES : HashMap<&'static str, String> = {
        let res = HashMap::from([
            ( "\\Bbb", "\\mathbb".to_string()),
            ("\\bold", "\\mathbf".to_string()),
            ("\\frak", "\\mathfrak".to_string()),
    ("\\bm", "\\boldsymbol".to_string()),
        ]);
        res
    };
}

pub fn font_handler_fn_1(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let ctx = context.borrow();
    let body = normalize_argument(&args[0]).clone();
    let func = FONT_ALIASES
        .get(&ctx.func_name.as_str())
        .unwrap_or(&ctx.func_name);

    let res = parse_node::types::font {
        mode: ctx.parser.mode,
        loc: None,
        font: func[1..].to_string(),
        body,
    };
    return Box::new(res) as Box<parse_node::types::font>;
}

lazy_static! {
    pub static ref FONT1: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_allowed_in_argument(true);

        FunctionDefSpec {
            def_type: "font".to_string(),
            names: vec! [
        // styles, except \boldsymbol defined below
        "\\mathrm".to_string(), "\\mathit".to_string(), "\\mathbf".to_string(), "\\mathnormal".to_string(),

        // families
        "\\mathbb".to_string(), "\\mathcal".to_string(), "\\mathfrak".to_string(), "\\mathscr".to_string(), "\\mathsf".to_string(),
        "\\mathtt".to_string(),

        // aliases, except \bm defined below
        "\\Bbb".to_string(), "\\bold".to_string(), "\\frak".to_string(),
    ],
            props,
            handler: font_handler_fn_1,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

pub fn font_handler_fn_2(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let ctx = context.borrow();
    let body = args[0].clone();
    let is_character_box = is_character_box(&body);
    // amsbsy.sty's \boldsymbol uses \binrel spacing to inherit the
    // argument's bin|rel|ord status
    let res = parse_node::types::mclass {
        mode: ctx.parser.mode,
        loc: None,
        mclass: super::mclass::binrel_class(&body),
        body: vec![Box::new(parse_node::types::font {
            mode: ctx.parser.mode,
            loc: None,
            font: "boldsymbol".to_string(),
            body,
        }) as Box<dyn AnyParseNode>],
        is_character_box,
    };
    return  Box::new(res) as Box<dyn AnyParseNode>;
}

lazy_static! {
    pub static ref FONT2: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);

        FunctionDefSpec {
            def_type: "mclass".to_string(),
            names: vec!["\\boldsymbol".to_string(), "\\bm".to_string()],
            props,
            handler: font_handler_fn_2,
            html_builder: None,
            mathml_builder: None,
        }
    });
}

pub fn font_handler_fn_3(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let b = {
        let m = context.borrow();
        m.break_on_token_text.clone()
    };
    let mut ctx = context.borrow_mut();
    let mode = ctx.parser.mode;
    let body = ctx.parser.parse_expression(true, b);
    let style = format!("math{}",ctx.func_name[1..].to_string());

    let res = parse_node::types::font {
        mode: mode,
        loc: None,
        font: style,
        body: Box::new(parse_node::types::ordgroup {
            mode: ctx.parser.mode,
            loc: None,
            body,
            semisimple: false,
        }) as Box<dyn AnyParseNode>,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

// Old font changing functions
lazy_static! {
    pub static ref FONT3: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "font".to_string(),
            names: vec![
                "\\rm".to_string(),
                "\\sf".to_string(),
                "\\tt".to_string(),
                "\\bf".to_string(),
                "\\it".to_string(),
                "\\cal".to_string(),
            ],
            props,
            handler: font_handler_fn_3,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
