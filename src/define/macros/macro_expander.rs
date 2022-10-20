/**
 * This file contains the “gullet” where macros are expanded
 * until only non-macro tokens remain.
 */
use core::num;
use std::{collections::HashMap, sync::Mutex};

use regex::Regex;

use crate::{
    parse_error::ParseError,
    settings::Settings,
    symbols::{get_symbol, public::Mode},
    token::Token,
    Lexer::Lexer,
    Namespace::Namespace,
};

use super::public::{MacroArg, MacroDefinition, MacroExpansion};

// List of commands that act like macros but aren't defined as a macro,
// function, or symbol.  Used in `isDefined`.
pub const IMPLICIT_COMMANDS: [&str; 4] = ["^", "_", "\\limits", "\\nolimits"];
pub struct MacroExpander<'a> {
    settings: &'a Settings,
    expansion_count: i32,
    lexer: Lexer,
    pub macros: Namespace<MacroDefinition>,
    stack: Vec<Token>,
    mode: Mode,
}

pub enum ExpandOneRes {
    token(Token),
    tokens(Vec<Token>),
}
impl MacroExpander<'_> {
    pub fn new(input: String, settings: &Settings, mode: Mode) -> MacroExpander {
        MacroExpander {
            settings: settings,
            expansion_count: 0,
            lexer: Lexer::new(input, settings),
            // Make new global namespace
            macros: Namespace::<MacroDefinition>::new(
                std::sync::Arc::new(HashMap::<String, MacroDefinition>::new()),
                settings.get_ref_macros(),
            ),
            mode,
            stack: vec![], // contains tokens in REVERSE order
        }
    }
    pub fn set_lexer_catcode(&mut self, char: String, code: i32) {
        self.lexer.set_catcode(char, code);
    }

    /**
     * Feed a new input String to the same MacroExpander
     * (with existing macros etc.).
     */
    // pub fn feed(&mut self,input: String) {
    //     self.lexer = new Lexer(input, self.settings);
    // }

    /**
     * Switches between "text" and "math" modes.
     */
    pub fn set_mode(&mut self, new_mode: Mode) {
        self.mode = new_mode;
    }

    /**
     * Start a new group nesting within all namespaces.
     */
    pub fn begin_group(&mut self) {
        self.macros.begin_group();
    }

    /**
     * End current group nesting within all namespaces.
     */
    pub fn end_group(&mut self) {
        self.macros.end_group();
    }

    /**
     * Ends all currently nested groups (if any), restoring values before the
     * groups began.  Useful in case of an error in the middle of parsing.
     */
    pub fn end_groups(&mut self) {
        self.macros.end_all_groups();
    }

    /**
     * Returns the topmost token on the stack, without expanding it.
     * Similar in behavior to TeX's `\futurelet`.
     */
    pub fn future(&mut self) -> Token {
        if (self.stack.len() == 0) {
            let t = self.lexer.lex();
            // println!("MacroExpander future(): {}",t);
            self.push_token(t);
        }
        return self.stack.last().unwrap().clone();
    }

    /**
     * Remove and return the next unexpanded token.
     */
    pub fn pop_token(&mut self) -> Token {
        self.future(); // ensure non-empty stack
        return self.stack.pop().unwrap();
    }

    /**
     * Add a given token to the token stack.  In particular, this get be used
     * to put back a token returned from one of the other methods.
     */
    pub fn push_token(&mut self, token: Token) {
        self.stack.push(token);
    }

    /**
     * Append an array of tokens to the token stack.
     */
    pub fn push_tokens(&mut self, tokens: Vec<Token>) {
        self.stack.extend(tokens);
    }

    /**
     * Find an macro argument without expanding tokens and append the array of
     * tokens to the token stack. Uses Token as a container for the result.
     */
    pub fn scan_argument(&mut self, is_optional: bool) -> Option<Token> {
        let mut _start: Option<Token> = None;
        let res = if (is_optional) {
            self.consume_spaces(); // \@ifnextchar gobbles any space following it
            if (self.future().text != "[") {
                return None;
            }
            _start = Some(self.pop_token()); // don't include [ in tokens
            self.consume_arg(vec!["]".to_string()])
        } else {
            self.consume_arg(vec![])
        };
        match res {
            Ok(arg) => {
                self.push_token(Token {
                    text: "EOF".to_string(),
                    loc: arg.end.loc.clone(),
                    noexpand: None,
                    treatAsRelax: None,
                });
                self.push_tokens(arg.tokens);
                if let Some(start) = _start {
                    return Some(start.range(&arg.end, "".to_string()));
                } else {
                    return Some(arg.start.range(&arg.end, "".to_string()));
                }
            }
            _ => {
                return None;
            }
        }
    }

    /**
     * Consume all following space tokens, without expansion.
     */
    pub fn consume_spaces(&mut self) {
        loop {
            let token = self.future();
            if (token.text == " ") {
                self.stack.pop();
            } else {
                break;
            }
        }
    }

    /**
     * Consume an argument from the token stream, and return the resulting array
     * of tokens and start/end token.
     */
    pub fn consume_arg(&mut self, delims: Vec<String>) -> Result<MacroArg, ParseError> {
        // The argument for a delimited parameter is the shortest (possibly
        // empty) sequence of tokens with properly nested {...} groups that is
        // followed ... by this particular list of non-parameter tokens.
        // The argument for an undelimited parameter is the next nonblank
        // token, unless that token is ‘{’, when the argument will be the
        // entire {...} group that follows.
        let mut tokens: Vec<Token> = vec![];
        let is_delimited = delims.len() > 0;
        if (!is_delimited) {
            // Ignore spaces between arguments.  As the TeXbook says:
            // "After you have said ‘\def\row#1#2{...}’, you are allowed to
            //  put spaces between the arguments (e.g., ‘\row x n’), because
            //  TeX doesn’t use single spaces as undelimited arguments."
            self.consume_spaces();
        }
        let mut start = self.future();
        let mut tok;
        let mut depth = 0;
        let mut match_pos = 0;
        loop {
            tok = self.pop_token();
            tokens.push(tok.clone());
            if (tok.text == "{") {
                depth += 1;
            } else if (tok.text == "}") {
                depth -= 1;
                if (depth == -1) {
                    return Err(ParseError {
                        msg: String::from("Extra }"),
                        loc: tok.loc,
                    });
                }
            } else if (tok.text == "EOF") {
                let msg = format!(
                    "Unexpected end of input in a macro argument, expected '{}'",
                    if is_delimited {
                        delims[match_pos].clone()
                    } else {
                        "}".to_string()
                    }
                );
                return Err(ParseError { msg, loc: tok.loc });
            }
            if (is_delimited) {
                if ((depth == 0 || (depth == 1 && delims[match_pos] == "{"))
                    && tok.text == delims[match_pos])
                {
                    match_pos += 1;
                    if (match_pos == delims.len()) {
                        // don't include delims in tokens
                        // tokens.splice(-match_pos, match_pos);
                        break;
                    }
                } else {
                    match_pos = 0;
                }
            }
            if !(depth != 0 || is_delimited) {
                break;
            }
        }
        // If the argument found ... has the form ‘{<nested tokens>}’,
        // ... the outermost braces enclosing the argument are removed
        if (start.text == "{" && tokens[tokens.len() - 1].text == "}") {
            tokens.pop();
            tokens.remove(0);
        }
        tokens.reverse(); // to fit in with stack order
        return Ok(MacroArg {
            tokens,
            start,
            end: tok,
        });
    }

    /**
     * Consume the specified f64 of (delimited) arguments from the token
     * stream and return the resulting array of arguments.
     */
    pub fn consume_args(
        &mut self,
        num_args: usize,
        delimiters: Vec<Vec<String>>,
    ) -> Result<Vec<Vec<Token>>, ParseError> {
        if delimiters.len() != num_args + 1 {
            return Err(ParseError {
                msg: "The length of delimiters doesn't match the number of args!".to_string(),
                loc: None,
            });
        }
        for delims in delimiters[0].iter() {
            let tok = self.pop_token();
            if delims != &tok.text {
                return Err(ParseError {
                    msg: "Use of the macro doesn't match its definition".to_string(),
                    loc: tok.loc,
                });
            }
        }

        let mut res: Vec<Vec<Token>> = vec![];
        for delims in delimiters[1..].iter() {
            match self.consume_arg(delims.to_vec()) {
                Ok(arg) => {
                    res.push(arg.tokens);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        return Ok(res);
    }

    /**
     * Expand the next token only once if possible.
     *
     * If the token is expanded, the resulting tokens will be pushed onto
     * the stack in reverse order and will be returned as an array,
     * also in reverse order.
     *
     * If not, the next token will be returned without removing it
     * from the stack.  This case can be detected by a `Token` return value
     * instead of an `Array` return value.
     *
     * In either case, the next token will be on the top of the stack,
     * or the stack will be empty.
     *
     * Used to implement `expandAfterFuture` and `expandNextToken`.
     *
     * If expandableOnly, only expandable tokens are expanded and
     * an undefined control sequence results in an error.
     */
    pub fn expand_once(&mut self, expandableOnly: bool) -> Result<ExpandOneRes, ParseError> {
        let topToken = self.pop_token();
        let name = &topToken.text;
        let _expansion = if !topToken.noexpand.unwrap_or(false) {
            self._getExpansion(name)
        } else {
            None
        };
        if _expansion.is_none() {
            let c = name.chars().nth(0).unwrap();
            if expandableOnly && c == '\\' && !self.is_defined(name) {
                return Err(ParseError {
                    msg: format!("Undefined control sequence: {name}"),
                    loc: None,
                });
            }
            self.push_token(topToken.clone());
            return Ok(ExpandOneRes::token(topToken));
        }

        let expansion = _expansion.unwrap();
        if expandableOnly && expansion.unexpandable {
            self.push_token(topToken.clone());
            return Ok(ExpandOneRes::token(topToken));
        }
        self.expansion_count += 1;
        if (self.expansion_count > self.settings.get_max_expand().unwrap_or(0)) {
            return Err(ParseError {
                msg: "Too many expansions: infinite loop or \
                need to increase maxExpand setting"
                    .to_string(),
                loc: None,
            });
        }
        let mut tokens = expansion.tokens;
        let args = self
            .consume_args(expansion.num_args as usize, expansion.delimiters.unwrap())
            .unwrap();
        if (expansion.num_args > 0) {
            // paste arguments in place of the placeholders
            // tokens = tokens.slice(); // make a shallow copy
            let mut i = tokens.len();
            while i >= 0 {
                let mut tok = &tokens[i];
                if (tok.text == "#") {
                    if (i == 0) {
                        return Err(ParseError {
                            msg: "Incomplete placeholder at end of macro body".to_string(),
                            loc: tok.loc.clone(),
                        });
                    }
                    i -= 1;
                    tok = &tokens[i]; // next token on stack
                    let number = Regex::new("^[1-9]$").unwrap();
                    if (tok.text == "#") {
                        // ## → #
                        tokens.remove(i + 1); // drop first #
                    } else if number.is_match(&tok.text) {
                        // replace the placeholder with the indicated argument
                        tokens.splice(
                            i..i + 2,
                            args[(tok.text.parse::<i32>().unwrap() - 1) as usize].clone(),
                        );
                    } else {
                        return Err(ParseError {
                            msg: "Not a valid argument number".to_string(),
                            loc: tok.loc.clone(),
                        });
                    }
                }
                i -= 1;
            }
        }
        // Concatenate expansion onto top of stack.
        self.push_tokens(tokens.clone());
        return Ok(ExpandOneRes::tokens(tokens));
    }

    /**
     * Expand the next token only once (if possible), and return the resulting
     * top token on the stack (without removing anything from the stack).
     * Similar in behavior to TeX's `\expandafter\futurelet`.
     * Equivalent to expandOnce() followed by future().
     */
    pub fn expand_after_future(&mut self) -> Token {
        self.expand_once(false);
        return self.future();
    }

    /**
     * Recursively expand first token, then return first non-expandable token.
     */
    pub fn expand_next_token(&mut self) -> Token {
        loop {
            let mut _expanded = self.expand_once(false).expect("");
            // expandOnce returns Token if and only if it's fully expanded.
            if let ExpandOneRes::token(mut expanded) = _expanded {
                // the token after \noexpand is interpreted as if its meaning
                // were ‘\relax’
                if (expanded.treatAsRelax.unwrap_or(false)) {
                    expanded.text = "\\relax".to_string();
                }
                return self.stack.pop().unwrap(); // == expanded
            }
        }

        // Flow unable to figure out that this pathway is impossible.
        // https://github.com/facebook/flow/issues/4808
        // throw new Error(); // eslint-disable-line no-unreachable
    }

    /**
     * Fully expand the given macro name and return the resulting list of
     * tokens, or return `undefined` if no such macro is defined.
     */
    pub fn expand_macro(&mut self, name: &String) -> Option<Vec<Token>> {
        return if self.macros.has(name) {
            Some(self.expand_tokens(vec![Token::new(name.clone(), None)]))
        } else {
            None
        };
    }

    /**
     * Fully expand the given token stream and return the resulting list of
     * tokens.  Note that the input tokens are in reverse order, but the
     * output tokens are in forward order.
     */
    pub fn expand_tokens(&mut self, tokens: Vec<Token>) -> Vec<Token> {
        let mut output = vec![];
        let oldStackLength = self.stack.len();
        self.push_tokens(tokens);
        while (self.stack.len() > oldStackLength) {
            let _expanded = self.expand_once(true).unwrap(); // expand only expandable tokens
                                                             // expandOnce returns Token if and only if it's fully expanded.
            if let ExpandOneRes::token(mut expanded) = _expanded {
                if (expanded.treatAsRelax.unwrap_or(false)) {
                    // the expansion of \noexpand is the token itself
                    expanded.noexpand = Some(false);
                    expanded.treatAsRelax = Some(false);
                }
                output.push(self.stack.pop().unwrap());
            }
        }
        return output;
    }

    /**
     * Fully expand the given macro name and return the result as a String,
     * or return `undefined` if no such macro is defined.
     */
    pub fn expand_macro_as_text(&mut self, name: &String) -> Option<String> {
        let mut _tokens = self.expand_macro(name);
        if let Some(tokens) = _tokens {
            return Some(
                tokens
                    .iter()
                    .fold(String::new(), |text, token| text + token.text.as_str()),
            );
        } else {
            return None;
        }
    }

    /**
     * Returns the expanded macro as a reversed array of tokens and a macro
     * argument count.  Or returns `null` if no such macro.
     */
    fn _getExpansion(&mut self, name: &String) -> Option<MacroExpansion> {
        let _definition = self.macros.get(name);
        if _definition.is_none() {
            // mainly checking for undefined here
            return None;
        }
        // If a single character has an associated catcode other than 13
        // (active character), then don't expand it.
        if (name.len() == 1) {
            if let Some(catcode) = self.lexer.catcodes_get(name) {
                if catcode != &13 {
                    return None;
                }
            }
        }
        let definition = _definition.unwrap();
        let d = match definition {
            MacroDefinition::MacroContext(ref f) => f(self),

            MacroDefinition::Str(s) => MacroDefinition::Str(s.clone()),
            MacroDefinition::MacroExpansion(m) => MacroDefinition::MacroExpansion(m.clone()),
        };

        match d {
            MacroDefinition::Str(s) => {
                let mut numArgs = 0;
                if s.contains("#") {
                    let stripped = s.replace("##", "");
                    while stripped.contains(format!("#{}", (numArgs + 1)).as_str()) {
                        numArgs += 1;
                    }
                }
                let mut bodyLexer = Lexer::new(s, &self.settings);
                let mut tokens = vec![];
                let mut tok = bodyLexer.lex();
                // println!("tok = {}",tok);
                while (tok.text != "EOF") {
                    tokens.push(tok.clone());
                    tok = bodyLexer.lex();
                    // println!("tok = {}",tok);
                }
                // console.log("_getExpansion",tokens);
                tokens.reverse(); // to fit in with stack using push and pop
                                  // const expanded = {tokens, numArgs};
                return Some(MacroExpansion {
                    tokens: tokens,
                    num_args: numArgs,
                    delimiters: None,
                    unexpandable: false, // used in \let
                });
            }
            MacroDefinition::MacroContext(m) => {
                // ! error
                return None;
            }
            MacroDefinition::MacroExpansion(exp) => {
                return None;
            }
        }
    }

    /**
     * Determine whether a command is currently "defined" (has some
     * functionality), meaning that it's a macro (in the current group),
     * a function, a symbol, or one of the special commands listed in
     * `implicitCommands`.
     */
    pub fn is_defined(&self, name: &String) -> bool {
        return self.macros.has(name) ||
            // functions.hasOwnProperty(name) ||  //!todo
            get_symbol(Mode::math, &name).is_some() ||
            get_symbol(Mode::text, &name).is_some()  ||
            IMPLICIT_COMMANDS.contains(&name.as_str());
    }

    // /**
    //  * Determine whether a command is expandable.
    //  */
    // pub fn isExpandable(&self, name: &String)-> bool {
    //     use crate::define::functions::public::get_function;
    //     let _macro = self.macros.get(name);
    //     if let Some(m) = _macro{
    //         match m{
    //             MacroDefinition::Str(s)=>{
    //                 return true;
    //             },
    //             MacroDefinition::MacroExpansion(expan)=>{
    //                 if !expan.unexpandable{
    //                     return true;
    //                 } else {
    //                     if let Some(f) = get_function(name){
    //                         return !f.primitive;
    //                     }else{
    //                         return false;
    //                     }
    //                 }
    //             },
    //             MacroDefinition::MacroContext(context)=>{
    //                 return true;

    //             }
    //         }
    //     }else{
    //         return false;
    //     }
    // }
}
