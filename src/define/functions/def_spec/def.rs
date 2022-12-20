use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::define::macros::public::MacroArg;
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::ParseNodeToAny;
use crate::token::Token;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::Parser::Parser;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::collections::HashMap;
use std::sync::Mutex;
//@flow

lazy_static! {
    static ref GlobalMap: std::sync::RwLock<HashMap<&'static str, &'static str>> =
        std::sync::RwLock::new({
            let res = HashMap::from([
                ("\\global", "\\global"),
                ("\\long", "\\\\globallong"),
                ("\\\\globallong", "\\\\globallong"),
                ("\\def", "\\gdef"),
                ("\\gdef", "\\gdef"),
                ("\\edef", "\\xdef"),
                ("\\xdef", "\\xdef"),
                ("\\let", "\\\\globallet"),
                ("\\futurelet", "\\\\globalfuture"),
            ]);
            res
        });
    static ref CTRL_SEQ: regex::Regex = regex::Regex::new(r"^(?:[\\{}$&#^_]|EOF)$").unwrap();
}

fn check_control_sequence(tok: Token) -> String {
    if CTRL_SEQ.is_match(tok.text.as_str()) {
        panic!("Expected a control sequence {:#?}", tok);
    }
    return tok.text.clone();
}

fn get_rhs(parser: &mut Parser) -> Token {
    let mut tok = parser.gullet.pop_token();
    if tok.text == "=".to_string() {
        // consume optional equals
        tok = parser.gullet.pop_token();
        if tok.text == " ".to_string() {
            // consume one optional space
            tok = parser.gullet.pop_token();
        }
    }
    return tok;
}

fn let_command(parser: &mut Parser, name: &String, tok: &mut Token, global: bool) {
    let tmp = crate::define::macros::public::MacroDefinition::MacroExpansion(
        crate::define::macros::public::MacroExpansion {
            tokens: vec![tok.clone()],
            num_args: 0,
            // reproduce the same behavior in expansion
            delimiters: None,
            unexpandable: !parser.gullet.is_expandable(&tok.text),
        },
    );
    let mut _macro = parser.gullet.macros.get(&tok.text).unwrap_or({
        // don't expand it later even if a macro with the same name is defined
        // e.g., \let\foo=\frac \def\frac{\relax} \frac12
        tok.noexpand = true;
        &tmp
    });
    parser.gullet.macros.set(name, Some(_macro.clone()), global);
}

// <assignment> -> <non-macro assignment>|<macro assignment>
// <non-macro assignment> -> <simple assignment>|\global<non-macro assignment>
// <macro assignment> -> <definition>|<prefix><macro assignment>
// <prefix> -> \global|\long|\outer

pub fn handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let y = GlobalMap.read().unwrap();
    let mut ctx = context.borrow_mut();
    ctx.parser.consume_spaces();
    let mut token = ctx.parser.fetch();
    if let Some(x) = y.get(token.text.as_str()) {
        // KaTeX doesn't have \par, so ignore \long
        if ctx.func_name == "\\global" || ctx.func_name == "\\\\globallong" {
            token.text = x.to_string();
        }
        let _res = ctx.parser.parse_function(None, "".to_string()).unwrap();
        let res = _res
            .as_any()
            .downcast_ref::<parse_node::types::internal>()
            .unwrap();
        return _res;
    }
    panic!("Invalid token after macro prefix {:#?}", token);
}

lazy_static! {
    pub static ref INTERNAL: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "internal".to_string(),
            names: vec![
        "\\global".to_string(), "\\long".to_string(),
        "\\\\globallong".to_string(), // can’t be entered directly
    ],
            props,
            handler: handler_fn,
            html_builder: None,
            mathml_builder: None,
        }
    });
}

// Basic support for macro definitions: \def, \gdef, \edef, \xdef
// <definition> -> <def><control sequence><definition text>
// <def> -> \def|\gdef|\edef|\xdef
// <definition text> -> <parameter text><left brace><balanced text><right brace>

pub fn handler_fn_2(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut ctx = context.borrow_mut();
    let mut tok = ctx.parser.gullet.pop_token();
    let name = tok.text.clone();
    if CTRL_SEQ.is_match(name.as_str()) {
        panic!("Expected a control sequence {:#?}", tok);
    }

    let mut num_args = 0;
    let mut insert = None;
    let mut delimiters = vec![vec![]];
    // <parameter text> contains no braces
    while ctx.parser.gullet.future().text != "{".to_string() {
        tok = ctx.parser.gullet.pop_token();
        if tok.text == "#" {
            // If the very last character of the <parameter text> is #, so that
            // this # is immediately followed by {, TeX will behave as if the {
            // had been inserted at the right end of both the parameter text
            // and the replacement text.
            if ctx.parser.gullet.future().text == "{".to_string() {
                insert = Some(ctx.parser.gullet.future());
                delimiters[num_args].push("{".to_string());
                break;
            }

            // A parameter, the first appearance of # must be followed by 1,
            // the next by 2, and so on; up to nine #’s are allowed
            tok = ctx.parser.gullet.pop_token();
            // lazy_static!{
            //
            // }
            // if (!(/^[1-9]$/.test(tok.text))) {
            //     panic!(`Invalid argument number "${tok.text}"`);
            // }
            if tok.text.parse() != Ok(num_args + 1) {
                panic!("Argument number {} out of order", tok.text);
            }
            num_args += 1;
            delimiters.push(vec![]);
        } else if tok.text == "EOF".to_string() {
            panic!("Expected a macro definition");
        } else {
            delimiters[num_args].push(tok.text);
        }
    }
    // replacement text, enclosed in '{' and '}' and properly nested
    let MacroArg { mut tokens, .. } = ctx.parser.gullet.consume_arg(vec![]).unwrap();
    if let Some(i) = insert {
        tokens.insert(0, i);
    }

    if ctx.func_name == "\\edef" || ctx.func_name == "\\xdef".to_string() {
        tokens = ctx.parser.gullet.expand_tokens(tokens);
        tokens.reverse(); // to fit in with stack order
    }

    let func_name = ctx.func_name.clone();
    let y = GlobalMap.read().unwrap();

    // Final arg is the expansion of the macro
    ctx.parser.gullet.macros.set(
        &name,
        Some(
            crate::define::macros::public::MacroDefinition::MacroExpansion(
                crate::define::macros::public::MacroExpansion {
                    tokens,
                    num_args: num_args as i32,
                    delimiters: Some(delimiters),
                    unexpandable: false,
                },
            ),
        ),
        Some(&func_name.as_str()) ==  y.get(func_name.as_str()).clone(),
    );

    let res = parse_node::types::internal {
        mode: ctx.parser.mode,
        loc: None,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

lazy_static! {
    pub static ref INTERNAL2: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        props.set_allowed_in_text(true);
        props.set_primitive(true);

        FunctionDefSpec {
            def_type: "internal".to_string(),
            names: vec![
                "\\def".to_string(),
                "\\gdef".to_string(),
                "\\edef".to_string(),
                "\\xdef".to_string(),
            ],
            props,
            handler: handler_fn_2,
            html_builder: None,
            mathml_builder: None,
        }
    });
}

// <simple assignment> -> <let assignment>
// <let assignment> -> \futurelet<control sequence><token><token>
//     | \let<control sequence><equals><one optional space><token>
// <equals> -> <optional spaces>|<optional spaces>=

pub fn handler_fn_3(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let func_name = {
        let ctx = context.borrow();
        ctx.func_name.clone()
    };
    let mut ctx = context.borrow_mut();
    let name = check_control_sequence(ctx.parser.gullet.pop_token());
    ctx.parser.gullet.consume_spaces();
    let mut tok = get_rhs(ctx.parser);
    let_command(ctx.parser, &name, &mut tok, func_name == "\\\\globallet");
    let res = parse_node::types::internal {
        mode: ctx.parser.mode,
        loc: None,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}
lazy_static! {
    pub static ref INTERNAL3: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        props.set_allowed_in_text(true);
        props.set_primitive(true);

        FunctionDefSpec {
            def_type: "internal".to_string(),
            names: vec![
        "\\let".to_string(),
        "\\\\globallet".to_string(), // can’t be entered directly
    ],
            props,
            handler: handler_fn_3,
            html_builder: None,
            mathml_builder: None,
        }
    });
}

// ref: https://www.tug.org/TUGboat/tb09-3/tb22bechtolsheim.pdf
pub fn handler_fn_4(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let func_name = {
        let ctx = context.borrow();
        ctx.func_name.clone()
    };
    let mut ctx = context.borrow_mut();
    let name = check_control_sequence(ctx.parser.gullet.pop_token());
    let middle = ctx.parser.gullet.pop_token();
    let mut tok = ctx.parser.gullet.pop_token();
    let_command(
        ctx.parser,
        &name,
        &mut tok,
        func_name == "\\\\globalfuture".to_string(),
    );
    ctx.parser.gullet.push_token(tok);
    ctx.parser.gullet.push_token(middle);
    let res = parse_node::types::internal {
        mode: ctx.parser.mode,
        loc: None,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

lazy_static! {
    pub static ref INTERNAL4: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        props.set_allowed_in_text(true);
        props.set_primitive(true);

        FunctionDefSpec {
            def_type: "internal".to_string(),
            names: vec![
        "\\futurelet".to_string(),
        "\\\\globalfuture".to_string(), // can’t be entered directly
    ],
            props,
            handler: handler_fn_4,
            html_builder: None,
            mathml_builder: None,
        }
    });
}
