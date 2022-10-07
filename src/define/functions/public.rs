use crate::parse_node::types::ParseNodeToAny;
pub(crate) use crate::AnyParseNode;
use crate::{
    mathML_tree::public::MathDomNode,
    parse_node,
    token::Token,
    tree::HtmlDomNode,
    types::{ArgType, BreakToken},
    Options::Options,
    Parser::Parser,
};
use lazy_static::lazy_static;
use std::collections::HashMap;

/** Context provided to function pub(crate) handlers for error messages. */
pub struct FunctionContext<'a> {
    pub func_name: String,
    pub parser: &'a Parser<'a>,
    pub token: Option<Token>,
    pub break_on_token_text: Option<BreakToken>,
}

type FunctionHandler = fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode>;
// Note: reverse the order of the return type union will cause a flow error.
// See https://github.com/facebook/flow/issues/3663.

type HtmlBuilder =
    fn(group: Box<dyn AnyParseNode>, options: crate::Options::Options) -> Box<dyn HtmlDomNode>;
type MathMLBuilder = fn(group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode>;

// More general version of `HtmlBuilder` for nodes (e.g. \sum, accent types)
// whose presence impacts super/subscripting. In this case, ParseNode<"supsub">
// delegates its HTML building to the HtmlBuilder corresponding to these nodes.
type HtmlBuilderSupSub = fn(Box<dyn AnyParseNode>, Options) -> Box<dyn HtmlDomNode>;

#[derive(Clone)]
pub struct FunctionPropSpec {
    // The number of arguments the function takes.
    num_args: i32,

    // An array corresponding to each argument of the function, giving the
    // type of argument that should be parsed. Its length should be equal
    // to `numOptionalArgs + numArgs`, and types for optional arguments
    // should appear before types for mandatory arguments.
    arg_types: Vec<ArgType>,

    // Whether it expands to a single token or a braced group of tokens.
    // If it's grouped, it can be used as an argument to primitive commands,
    // such as \sqrt (without the optional argument) and super/subscript.
    allowed_in_argument: bool,

    // Whether or not the function is allowed inside text mode
    // (default false)
    allowed_in_text: bool,

    // Whether or not the function is allowed inside text mode
    // (default true)
    allowed_in_math: bool,

    // (optional) The number of optional arguments the function
    // should parse. If the optional arguments aren't found,
    // `null` will be passed to the handler in their place.
    // (default 0)
    num_optional_args: i32,

    // Must be true if the function is an infix operator.
    infix: bool,

    // Whether or not the function is a TeX primitive.
    primitive: bool,
}

impl FunctionPropSpec {
    pub fn new() -> FunctionPropSpec {
        return FunctionPropSpec {
            num_args: 0,
            arg_types: Vec::new(),
            allowed_in_argument: false,
            allowed_in_text: false,
            allowed_in_math: true,
            num_optional_args: 0,
            infix: false,
            primitive: false,
        };
    }
    pub fn set_num_args(&mut self, num_args: i32) -> &mut Self {
        self.num_args = num_args;
        self
    }
    pub fn get_num_args(&self) -> i32 {
        self.num_args
    }
    pub fn set_arg_types(&mut self, arg_types: Vec<ArgType>) -> &mut Self {
        self.arg_types = arg_types;
        self
    }
    pub fn get_arg_types(&self) -> &Vec<ArgType> {
        &self.arg_types
    }
    pub fn set_allowed_in_argument(&mut self, allowed_in_argument: bool) -> &mut Self {
        self.allowed_in_argument = allowed_in_argument;
        self
    }
    pub fn get_allowed_in_argument(&self) -> bool {
        self.allowed_in_argument
    }

    pub fn set_allowed_in_text(&mut self, allowed_in_text: bool) -> &mut Self {
        self.allowed_in_text = allowed_in_text;
        self
    }
    pub fn get_allowed_in_text(&self) -> bool {
        self.allowed_in_text
    }
    pub fn set_allowed_in_math(&mut self, allowed_in_math: bool) -> &mut Self {
        self.allowed_in_math = allowed_in_math;
        self
    }
    pub fn get_allowed_in_math(&self) -> bool {
        self.allowed_in_math
    }
    pub fn set_num_optional_args(&mut self, num_optional_args: i32) -> &mut Self {
        self.num_optional_args = num_optional_args;
        self
    }
    pub fn get_num_optional_args(&self) -> i32 {
        self.num_optional_args
    }

    pub fn get_infix(&self) -> bool {
        self.infix
    }
    pub fn set_infix(&mut self, infix: bool) -> &mut Self {
        self.infix = infix;
        self
    }
    pub fn set_primitive(&mut self, primitive: bool) -> &mut Self {
        self.primitive = primitive;
        self
    }
    pub fn get_primitive(&self) -> bool {
        self.primitive
    }
}
#[derive(Clone)]
pub struct FunctionDefSpec {
    pub def_type: String,

    // The first argument to defineFunction is a single name or a list of names.
    // All functions named in such a list will share a single implementation.
    pub names: Vec<String>,

    // Properties that control how the functions are parsed.
    pub props: FunctionPropSpec,

    // The handler is called to handle these functions and their arguments and
    // returns a `ParseNode`.
    pub handler: FunctionHandler,

    // This function returns an object representing the DOM structure to be
    // created when rendering the defined LaTeX function.
    // This should not modify the `ParseNode`.
    pub html_builder: Option<HtmlBuilder>,

    // This function returns an object representing the MathML structure to be
    // created when rendering the defined LaTeX function.
    // This should not modify the `ParseNode`.
    pub mathml_builder: Option<MathMLBuilder>,
}

pub type FunctionSpec = (FunctionPropSpec, FunctionHandler);
/**
 * Final function spec for use at parse time.
 * This is almost identical to `FunctionPropSpec`, except it
 * 1. includes the function handler, and
 * 2. requires all arguments except argTypes.
 * It is generated by `defineFunction()` below.
 */
lazy_static! {
    /**
     * All registered functions.
     * `functions.js` just exports this same dictionary again and makes it public.
     * `Parser.js` requires this dictionary.
     */
    pub static ref _functions: std::sync::Mutex<HashMap<String,FunctionSpec>> =  std::sync::Mutex::new({
        let mut res = HashMap::new();
        for data in super::def_spec::FUNCS.lock().unwrap().iter(){
             for name in data.names.iter() {
                res.insert(name.clone(), (data.props.clone(), data.handler));
            }
        }
        res
    });
    /**
     * All HTML builders. Should be only used in the `define*` and the `build*ML`
     * functions.
     */
    pub static ref _HTML_GROUP_BUILDERS: std::sync::RwLock<HashMap<String, HtmlBuilder>> =  std::sync::RwLock::new({
        let mut res = HashMap::new();
        for data in super::def_spec::FUNCS.lock().unwrap().iter(){
            if let Some(h) = data.html_builder{
                res.insert(data.def_type.clone(),h);
            }
        }
        res
    });
    /**
     * All MathML builders. Should be only used in the `define*` and the `build*ML`
     * functions.
     */
    pub static ref _mathmlGroupBuilders: std::sync::Mutex<HashMap<String,MathMLBuilder>> =  std::sync::Mutex::new({
        let mut res = HashMap::new();
        for data in super::def_spec::FUNCS.lock().unwrap().iter(){
            if let Some(h) = data.mathml_builder{
                res.insert(data.def_type.clone(),h);
            }
        }
        res
    });
}

// pub fn get_function(name:&String)->Option<&(FunctionPropSpec, FunctionHandler)>{
//     let funcs = _functions.lock().unwrap();
//     return funcs.get(name);
//

// pub fn define_function(data: FunctionDefSpec) {
//     for name in data.names {
//         let mut _f = _functions.lock().unwrap();
//         _f.insert(name, (data.props.clone(), data.handler));
//     }
//     if let Some(hB) = data.html_builder {
//         let _h = _HTML_GROUP_BUILDERS.lock().unwrap();
//         // _h.insert(NODETYPE::as_str(), hB);
//     }
//     if let Some(mB) = data.mathml_builder {
//         let _m = _mathmlGroupBuilders.lock().unwrap();
//         // _m.insert(NODETYPE::as_str(), mB);
//     }
// }

pub fn test(a: i32, b: i32) -> i32 {
    return a + b;
}

// /**
//  * Use this to register only the HTML and MathML builders for a function (e.g.
//  * if the function's ParseNode is generated in Parser.js rather than via a
//  * stand-alone handler provided to `defineFunction`).
//  */
// export function defineFunctionBuilders<NODETYPE: NodeType>({
//     type, htmlBuilder, mathmlBuilder,
// }: {|
//     type: NODETYPE,
//     htmlBuilder?: HtmlBuilder<NODETYPE>,
//     mathmlBuilder: MathMLBuilder<NODETYPE>,
// |}) {
//     defineFunction({
//         type,
//         names: [],
//         props: {numArgs: 0},
//         handler() { throw new Error('Should never be called.'); },
//         htmlBuilder,
//         mathmlBuilder,
//     });
// }

// export const normalizeArgument = function(arg: AnyParseNode): AnyParseNode {
//     return arg.type === "ordgroup" && arg.body.length === 1 ? arg.body[0] : arg;
// };

// Since the corresponding buildHTML/buildMathML function expects a
// list of elements, we normalize for different kinds of arguments
pub fn ord_argument(arg: &Box<dyn AnyParseNode>) -> Vec<Box<dyn AnyParseNode>> {
    if let Some(ord_group) = arg.as_any().downcast_ref::<parse_node::types::ordgroup>() {
        return ord_group.body.clone();
    } else {
        return vec![arg.clone()];
    }
}
