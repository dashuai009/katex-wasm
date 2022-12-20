use std::default;

use crate::{types::Mode, settings::Settings, Lexer::Lexer, Namespace::Namespace, token::Token};

use super::macro_expander::MacroExpander;
// /**
//  * Provides context to macros defined by functions. Implemented by
//  * MacroExpander.
//  */
// struct MacroContextInterface {
//     mode: Mode,
//     /**
//      * Object mapping macros to their expansions.
//      */
//     macros: Namespace<MacroDefinition>,

//     /**
//      * Returns the topmost token on the stack, without expanding it.
//      * Similar in behavior to TeX's `\futurelet`.
//      */
//     future()-> Token;

//     /**
//      * Remove and return the next unexpanded token.
//      */
//     popToken(): Token;

//     /**
//      * Consume all following space tokens, without expansion.
//      */
//     consumeSpaces(): void;

//     /**
//      * Expand the next token only once if possible.
//      */
//     expandOnce(expandableOnly?: boolean): Token | Token[];

//     /**
//      * Expand the next token only once (if possible), and return the resulting
//      * top token on the stack (without removing anything from the stack).
//      * Similar in behavior to TeX's `\expandafter\futurelet`.
//      */
//     expandAfterFuture(): Token;

//     /**
//      * Recursively expand first token, then return first non-expandable token.
//      */
//     expandNextToken(): Token;

//     /**
//      * Fully expand the given macro name and return the resulting list of
//      * tokens, or return `undefined` if no such macro is defined.
//      */
//     expandMacro(name: string): Token[] | void;

//     /**
//      * Fully expand the given macro name and return the result as a string,
//      * or return `undefined` if no such macro is defined.
//      */
//     expandMacroAsText(name: string): string | void;

//     /**
//      * Fully expand the given token stream and return the resulting list of
//      * tokens.  Note that the input tokens are in reverse order, but the
//      * output tokens are in forward order.
//      */
//     expandTokens(tokens: Token[]): Token[];

//     /**
//      * Consume an argument from the token stream, and return the resulting array
//      * of tokens and start/end token.
//      */
//     consumeArg(delims?: ?string[]): MacroArg;

//     /**
//      * Consume the specified number of arguments from the token stream,
//      * and return the resulting array of arguments.
//      */
//     consumeArgs(numArgs: number): Token[][];

//     /**
//      * Determine whether a command is currently "defined" (has some
//      * functionality), meaning that it's a macro (in the current group),
//      * a function, a symbol, or one of the special commands listed in
//      * `implicitCommands`.
//      */
//     isDefined(name: string): boolean;

//     /**
//      * Determine whether a command is expandable.
//      */
//     isExpandable(name: string): boolean;
// }

// pub struct MacroExpander{
//     settings: Settings,
//     expansionCount: f64,
//     lexer: Lexer,
//     macros: Namespace<MacroDefinition>,
//     stack: Vec<Token>,
//     mode: Mode
// }

pub struct MacroArg{
    pub tokens: Vec<Token>,
    pub start: Token,
    pub end: Token
}

/** Macro tokens (in reverse order). */
#[derive(Clone,Debug)]
pub struct MacroExpansion {
    pub tokens: Vec<Token>,
    pub num_args: i32,
    pub delimiters: Option<Vec<Vec<String>>>,
    pub unexpandable: bool, // used in \let
}

#[derive(Clone)]
pub enum MacroDefinition{
    // #[default]
    Str(String),
    MacroExpansion(MacroExpansion),
    MacroContext(fn(&mut MacroExpander) -> MacroDefinition)
}

impl std::fmt::Debug for MacroDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f , "MacroDefinition deasdfaoiwrjtgnasdjikvn")
    }
}
// pub struct MacroMap = {[string]: MacroDefinition};

// /**
//  * All registered global/built-in macros.
//  * `macros.js` exports this same dictionary again and makes it public.
//  * `Parser.js` requires this dictionary via `macros.js`.
//  */
// pub _macros: MacroMap = {};

// This function might one day accept an additional argument and do more things.
// export default function defineMacro(name: string, body: MacroDefinition) {
//     _macros[name] = body;
// }
