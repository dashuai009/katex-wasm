use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
use crate::define::environments::_environments;
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionContext2, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::ParseNodeToAny;
use crate::Options::Options;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::cell::RefCell;
use std::sync::Mutex;

fn env_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    _opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut ctx = context.borrow_mut();
    let Some(name_group) = args[0]
        .as_any()
        .downcast_ref::<parse_node::types::ordgroup>()
    else {
        ctx.parser
            .report_parse_error("Invalid environment name".to_string(), None);
        return Box::new(parse_node::types::ordgroup {
            mode: ctx.parser.mode,
            loc: None,
            body: vec![],
            semisimple: false,
        }) as Box<dyn AnyParseNode>;
    };

    let env_name = name_group
        .body
        .iter()
        .map(|item| {
            let t = item
                .as_any()
                .downcast_ref::<parse_node::types::textord>()
                .unwrap();
            t.text.clone()
        })
        .collect::<Vec<_>>()
        .concat();

    if ctx.func_name == "\\begin".to_string() {
        // begin...end is similar to left...right
        // Build the environment object. Arguments and other information will
        // be made available to the begin and end methods using properties.
        let envs = _environments.read().unwrap();
        let Some(env) = envs.get(&env_name) else {
            ctx.parser.report_parse_error(
                format!("No such environment: {}", env_name),
                name_group.loc.clone(),
            );
            return Box::new(parse_node::types::ordgroup {
                mode: ctx.parser.mode,
                loc: None,
                body: vec![],
                semisimple: false,
            }) as Box<dyn AnyParseNode>;
        };
        let (args, opt_args) = ctx
            .parser
            .parse_arguments(&format!("\\begin{env_name}"), env);
        if ctx.parser.error.is_some() {
            return Box::new(parse_node::types::ordgroup {
                mode: ctx.parser.mode,
                loc: None,
                body: vec![],
                semisimple: false,
            }) as Box<dyn AnyParseNode>;
        }
        let context = RefCell::new(FunctionContext2 {
            func_name: env_name.clone(),
            token: None,
            parser: ctx.parser,
            break_on_token_text: None,
        });
        let result = env.1(context, args, opt_args);
        if ctx.parser.error.is_some() {
            return result;
        }
        ctx.parser.expect("\\end".to_string(), false);
        if ctx.parser.error.is_some() {
            return result;
        }
        let end_name_token = ctx.parser.next_token.clone();
        let Some(tmp) = ctx.parser.parse_function(None, "".to_string()) else {
            return result;
        };
        if ctx.parser.error.is_some() {
            return result;
        }
        let end = tmp
            .as_any()
            .downcast_ref::<parse_node::types::environment>()
            .unwrap();
        if end.name != env_name {
            let msg = format!(
                "Mismatch: \\begin{{{}}} matched by \\end{{{}}}",
                env_name, end.name
            );
            if let Some(token) = end_name_token.as_ref() {
                ctx.parser.report_token_error(msg, token);
            } else {
                ctx.parser.report_parse_error(msg, None);
            }
        }
        return result;
    }

    let res = parse_node::types::environment {
        mode: ctx.parser.mode,
        loc: None,
        name: env_name,
        name_group: args[0].clone(),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

// Environment delimiters. HTML/MathML rendering is defined in the corresponding
// defineEnvironment definitions.

lazy_static! {
    pub static ref ENV: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_arg_types(vec![ArgType::text]);

        FunctionDefSpec {
            def_type: "environment".to_string(),
            names: vec!["\\begin".to_string(), "\\end".to_string()],
            props,
            handler: env_handler_fn,
            html_builder: None,
            mathml_builder: None,
        }
    });
}
