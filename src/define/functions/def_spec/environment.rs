use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionContext2, FunctionDefSpec, FunctionPropSpec,
    _environments,
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
    let name_group = args[0]
        .as_any()
        .downcast_ref::<parse_node::types::ordgroup>()
        .expect(&format!("Invalid environment name").as_str());

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


    if context.borrow().func_name == "\\begin".to_string() {
        // begin...end is similar to left...right
        // Build the environment object. Arguments and other information will
        // be made available to the begin and end methods using properties.
        let envs = _environments.read().unwrap();
        let env = envs
            .get(&env_name)
            .expect(&*format!("No such environment: {env_name}"));
        let (args, opt_args) = ctx
            .parser
            .parse_arguments(&format!("\\begin{{env_name}}"), env);
        let context = RefCell::new(FunctionContext2 {
            func_name: env_name.clone(),
            token: None,
            parser: ctx.parser,
            break_on_token_text: None,
        });
        let result = env.1(context, args, opt_args);
        ctx.parser.expect("\\end".to_string(), false);
        let end_name_token = ctx.parser.next_token.clone();
        let tmp = ctx.parser
            .parse_function(None,"".to_string())
            .unwrap();
        let end = tmp
            .as_any()
            .downcast_ref::<parse_node::types::environment>()
            .unwrap();
        if end.name != env_name {
            panic!(
                "Mismatch: \\begin{{env_name}} matched by \\end{{end.name}} {:#?}",
                end_name_token
            );
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
