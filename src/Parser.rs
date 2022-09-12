use std::{any::Any, str::FromStr};

/**
 * This file contains the parser used to parse out a TeX expressi&on from the
 * input. Since TeX isn't context-free, standard parsers don't work particularly
 * well.
 *
 * The strategy of this parser is as such:
 *
 * The main functions (the `.parse...` ones) take a position in the current
 * parse String to parse tokens from. The lexer (found in Lexer.js, stored at
 * self.gullet.lexer) also supports pulling out tokens at arbitrary places. When
 * individual tokens are needed at a position, the lexer is called to pull out a
 * token, which is then used.
 *
 * The parser has a property called "mode" indicating the mode that
 * the parser is currently in. Currently it has to be one of "math" or
 * "text", which denotes whether the current environment is a math-y
 * one or a text-y one (e.g. inside \text). Currently, this serves to
 * limit the functions which can be used in text mode.
 *
 * The main functions then return an object which contains the useful data that
 * was parsed at its given point, and a new position at the end of the parsed
 * data. The main functions can call each other and continue the parsing by
 * using the returned position as a new starting point.
 *
 * There are also extra `.handle...` functions, which pull out some reused
 * functionality into self-contained functions.
 *
 * The functions return ParseNodes.
 */
use crate::{
    define::{
        functions::public::{FunctionContext, FunctionSpec, _functions},
        macros::{macro_expander::MacroExpander, public::MacroDefinition},
    },
    parse_node::{
        self,
        types::{ordgroup, AnyParseNode, Atom},
    },
    settings::Settings,
    sourceLocation::SourceLocation,
    symbols::public::{Group, Mode},
    token::Token,
    types::{ArgType, BreakToken},
    unicodeSupOrSub::U_SUBS_AND_SUPS,
};

pub struct Parser<'a> {
    pub mode: Mode,
    gullet: MacroExpander<'a>,
    settings: &'a Settings,
    left_right_depth: i32,
    next_token: Option<Token>,
}

const END_OF_EXPRESSION: [&'static str; 5] = ["}", "\\endgroup", "\\end", "\\right", "&"];
impl Parser<'_> {
    pub fn new(input: String, settings: &Settings) -> Parser {
        Parser {
            // Start in math mode
            mode: Mode::math,
            // Create a new macro expander (gullet) and (indirectly via that) also a
            // new lexer (mouth) for this parser (stomach, in the language of TeX)
            gullet: MacroExpander::new(input, settings, Mode::math),
            // Store the settings for use in parsing
            settings,
            // Count leftright depth (for \middle errors)
            left_right_depth: 0,
            next_token: None,
        }
    }

    /**
     * Checks a result to make sure it has the right type, and throws an
     * appropriate error otherwise.
     */
    pub fn expect(&mut self, text: String, consume: bool) {
        if (self.fetch().text != text) {
            // throw new ParseError(
            //     `Expected '${text}', got '${self.fetch().text}'`, self.fetch()
            // );
            panic!("Expected '{text}', got '{} ", self.fetch().text);
        }
        if (consume) {
            self.consume();
        }
    }

    /**
     * Discards the current lookahead token, considering it consumed.
     */
    pub fn consume(&mut self) {
        self.next_token = None;
    }

    /**
     * Return the current lookahead token, or if there isn't one (at the
     * beginning, or if the previous lookahead token was consume()d),
     * fetch the next token as the new lookahead token and return it.
     */
    pub fn fetch(&mut self) -> Token {
        if self.next_token.is_none() {
            self.next_token = Some(self.gullet.expand_next_token());
        }
        return self.next_token.clone().unwrap();
    }

    /**
     * Switches between "text" and "math" modes.
     */
    pub fn switch_mode(&mut self, newMode: Mode) {
        self.mode = newMode;
        self.gullet.set_mode(newMode);
    }

    /**
     * Main parsing function, which parses an entire input.
     */
    pub fn parse(&mut self) -> Vec<Box<dyn AnyParseNode>> {
        if (!self.settings.get_global_group()) {
            // Create a group namespace for the math expression.
            // (LaTeX creates a new group for every $...$, $$...$$, \[...\].)
            self.gullet.begin_group();
        }

        // Use old \color behavior (same as LaTeX's \textcolor) if requested.
        // We do this within the group for the math expression, so it doesn't
        // pollute settings.macros.
        if (self.settings.get_color_is_text_color()) {
            self.gullet.macros.set(
                &("\\color".to_string()),
                Some(MacroDefinition::Str("\\textcolor".to_string())),
                false,
            );
        }

        // Try to parse the input
        let parse = self.parse_expression(false, None);
        for t in parse.iter() {
            print!("{},", t.get_type());
        }

        // If we succeeded, make sure there's an EOF at the end
        self.expect("EOF".to_string(), true);

        // End the group namespace for the expression
        if (!self.settings.get_global_group()) {
            self.gullet.end_group();
        }

        return parse;

        // Close any leftover groups in case of a parse error.

        self.gullet.end_groups();
    }

    /**
     * Fully parse a separate sequence of tokens as a separate job.
     * Tokens should be specified in reverse order, as in a MacroDefinition.
     */
    pub fn subparse(&mut self, tokens: Vec<Token>) -> Vec<Box<dyn AnyParseNode>> {
        // Save the next token from the current job.
        let oldToken = self.next_token.clone();
        self.consume();

        // Run the new job, terminating it with an excess '}'
        self.gullet.push_token(Token::new("}".to_string(), None));
        self.gullet.push_tokens(tokens);
        let parse = self.parse_expression(false, None);
        self.expect("}".to_string(), true);

        // Restore the next token from the current job.
        self.next_token = oldToken;

        return parse;
    }

    /**
     * Parses an "expression", which is a list of atoms.
     *
     * `breakOnInfix`: Should the parsing stop when we hit infix nodes? This
     *                 happens when functions have higher precendence han infix
     *                 nodes in implicit parses.
     *
     * `breakOnTokenText`: The text of the token that the expression should end
     *                     with, or `null` if something else should end the
     *                     expression.
     */
    pub fn parse_expression(
        &mut self,
        break_on_infix: bool,
        break_on_token_text: Option<BreakToken>,
    ) -> Vec<Box<dyn AnyParseNode>> {
        let mut body = vec![];
        // Keep adding atoms to the body until we can't parse any more atoms (either
        // we reached the end, a }, or a \right)
        loop {
            // Ignore spaces in math mode
            if (self.mode == Mode::math) {
                self.consume_spaces();
            }
            let lex = self.fetch();
            println!("lex = {}", lex);
            if (END_OF_EXPRESSION.contains(&lex.text.as_str())) {
                break;
            }
            if let Some(t) = &break_on_token_text {
                if t.as_str() == lex.text {
                    break;
                }
            }
            if (break_on_infix) {
                let funcs = crate::define::functions::public::_functions.lock().unwrap();
                if let Some((f1, f2)) = funcs.get(&lex.text) {
                    if f1.get_infix() {
                        break;
                    }
                }
            }
            let atom = self.parse_atom(break_on_token_text.clone());
            // println!("atom = {:?}",atom);
            if let Some(_atom) = atom {
                println!("atom = {}", _atom.get_type());
                if _atom.get_type() == "internal" {
                    continue;
                }
                body.push(_atom);
            } else {
                println!("_atom is None");
                break;
            }
        }
        if (self.mode == Mode::text) {
            body = self.form_ligatures(body);
        }
        return self.handle_infix_nodes(body);
    }

    /**
     * Rewrites infix operators such as \over with corresponding commands such
     * as \frac.
     *
     * There can only be one infix operator per group.  If there's more than one
     * then the expression is ambiguous.  This can be resolved by adding {}.
     */
    pub fn handle_infix_nodes(
        &mut self,
        body: Vec<Box<dyn AnyParseNode>>,
    ) -> Vec<Box<dyn AnyParseNode>> {
        if body.len() == 0 {
            return body;
        }

        let frac = body.split(|x| x.get_type() == "infix").collect::<Vec<_>>();

        if frac.len() > 2 {
            //多于二个
            panic!("only one infix operator per group");
        }

        if frac[0].last().unwrap().get_type() != "infix" {
            //，或者没有
            return frac[0].to_vec();
        }

        let mut infix = (&frac[0].last().unwrap()).as_any();
        let numerNode = if (frac[0].len() == 2 && frac[0][0].get_type() == "ordgroup") {
            frac[0][0].clone()
        } else {
            Box::new(parse_node::types::ordgroup {
                mode: self.mode,
                body: frac[0][0..(frac[0].len() - 1)].to_vec(),
                loc: None,
                semisimple: false,
            })
        };
        let denomBody = frac[1];

        let denomNode = if (denomBody.len() == 1 && denomBody[0].get_type() == "ordgroup") {
            denomBody[0].clone()
        } else {
            Box::new(parse_node::types::ordgroup {
                mode: self.mode,
                body: frac[1].to_vec(),
                loc: None,
                semisimple: false,
            })
        };
        let func_name = infix
            .downcast_ref::<parse_node::types::infix>()
            .unwrap()
            .get_replace_with();

        let node = if (func_name == "\\\\abovefrac") {
            self.call_function(
                &func_name,
                vec![numerNode, (frac[0].last().unwrap().clone()), denomNode],
                vec![],
                None,
                None,
            )
        } else {
            self.call_function(&func_name, vec![numerNode, denomNode], vec![], None, None)
        };
        return vec![node];
    }

    // /**
    //  * Handle a subscript or superscript with nice errors.
    //  */
    pub fn handle_sup_sub_script(
        &mut self,
        name: String, // For error reporting.
    ) -> Option<Box<dyn AnyParseNode>> {
        let symbolToken = self.fetch();
        let symbol = symbolToken.text;
        self.consume();
        self.consume_spaces(); // ignore spaces before sup/subscript argument
        let group = self.parse_group(name, None);

        if group.is_none() {
            panic!("Expected group after")
            // throw new ParseError(
            //     "Expected group after '" + symbol + "'",
            //     symbolToken
            // );
        }

        return group;
    }

    // /**
    //  * Converts the textual input of an unsupported command into a text node
    //  * contained within a color node whose color is determined by errorColor
    //  */
    pub fn format_unsupported_cmd(&mut self, text: String) -> parse_node::types::color {
        let textordArray = text
            .chars()
            .map(|c| {
                Box::new(parse_node::types::textord {
                    mode: Mode::text,
                    text: c.to_string(),
                    loc: None,
                }) as Box<dyn AnyParseNode>
            })
            .collect::<Vec<_>>();

        let textNode = Box::new(parse_node::types::text {
            mode: self.mode,
            body: textordArray,
            loc: None,
            font: None,
        }) as Box<dyn AnyParseNode>;

        return parse_node::types::color {
            mode: self.mode,
            color: self.settings.get_error_color(),
            body: vec![textNode],
            loc: None,
        };
    }

    /**
     * Parses a group with optional super/subscripts.
     */
    pub fn parse_atom(
        &mut self,
        breakOnTokenText: Option<BreakToken>,
    ) -> Option<Box<dyn AnyParseNode>> {
        // The body of an atom is an implicit group, so that things like
        // \left(x\right)^2 work correctly.
        let mut _base = self.parse_group("atom".to_string(), breakOnTokenText);

        // In text mode, we don't have superscripts or subscripts
        if (self.mode == Mode::text) {
            return _base.clone();
        }

        // Note that base may be empty (i.e. null) at this point.

        let mut superscript = None;
        let mut subscript = None;
        loop {
            // Guaranteed in math mode, so eat any spaces first.
            self.consume_spaces();

            // Lex the first token
            let lex = self.fetch();

            if (lex.text == "\\limits" || lex.text == "\\nolimits") {
                // We got a limit control
                if let Some(mut base) = _base {
                    _base = match base.get_type() {
                        "op" => {
                            if let Some(op) =
                                base.as_mut_any().downcast_mut::<parse_node::types::op>()
                            {
                                let limits = lex.text == "\\limits";
                                op.limits = limits;
                                op.alwaysHandleSupSub = true;
                                Some(Box::new(op.clone()) as Box<dyn AnyParseNode>)
                            } else {
                                unreachable!();
                            }
                        }
                        "operatorname" => {
                            if let Some(op) = base
                                .as_mut_any()
                                .downcast_mut::<parse_node::types::operatorname>()
                            {
                                if (op.alwaysHandleSupSub) {
                                    op.limits = lex.text == "\\limits";
                                }
                                Some(Box::new(op.clone()) as Box<dyn AnyParseNode>)
                            } else {
                                unreachable!()
                            }
                        }
                        _ => {
                            panic!("Limit controls must follow a math operator");
                        }
                    }
                } else {
                    self.consume();
                }
            } else if (lex.text == "^") {
                // We got a superscript start
                if (superscript.is_some()) {
                    panic!("Double superscript");
                    // throw new ParseError("Double superscript", lex);
                }
                superscript = self.handle_sup_sub_script("superscript".to_string());
            } else if (lex.text == "_") {
                // We got a subscript start
                if (subscript.is_some()) {
                    panic!("Double subscript");
                    // throw new ParseError("Double subscript", lex);
                }
                subscript = self.handle_sup_sub_script("subscript".to_string());
            } else if (lex.text == "'") {
                // We got a prime
                if (superscript.is_some()) {
                    panic!("double superscript");
                    // throw new ParseError("Double superscript", lex);
                }
                let prime = parse_node::types::textord {
                    mode: self.mode,
                    text: "\\prime".to_string(),
                    loc: None,
                };

                // Many primes can be grouped together, so we handle this here
                let mut primes: Vec<Box<dyn AnyParseNode>> = vec![Box::new(prime.clone())];
                self.consume();
                // Keep lexing tokens until we get something that's not a prime
                while (self.fetch().text == "'") {
                    // For each one, add another prime to the list
                    primes.push(Box::new(prime.clone()));
                    self.consume();
                }
                // If there's a superscript following the primes, combine that
                // superscript in with the primes.
                if (self.fetch().text == "^") {
                    primes.push(
                        self.handle_sup_sub_script("superscript".to_string())
                            .unwrap(),
                    );
                }
                // Put everything into an ordgroup as the superscript
                superscript = Some(Box::new(parse_node::types::ordgroup {
                    mode: self.mode,
                    body: primes,
                    loc: None,
                    semisimple: false,
                }));
            } else {
                let u_subs_and_sups = U_SUBS_AND_SUPS.lock().unwrap();
                if let Some(mut _s) = u_subs_and_sups.get(&lex.text.as_str()) {
                    let mut _str = _s.to_string();
                    // A Unicode subscript or superscript character.
                    // We treat these similarly to the unicode-math package.
                    // So we render a String of Unicode (sub|super)scripts the
                    // same as a (sub|super)script of regular characters.
                    lazy_static! {
                        static ref unicode_sub_re: regex::Regex =
                            regex::Regex::new(r"^[₊₋₌₍₎₀₁₂₃₄₅₆₇₈₉ₐₑₕᵢⱼₖₗₘₙₒₚᵣₛₜᵤᵥₓᵦᵧᵨᵩᵪ]").unwrap();
                    }
                    let isSub = unicode_sub_re.is_match(&lex.text.as_str());
                    self.consume();
                    // Continue fetching tokens to fill out the String.
                    loop {
                        let token = self.fetch().text;
                        let _token = u_subs_and_sups.get(&token.as_str());
                        if _token.is_none() {
                            break;
                        }
                        if (unicode_sub_re.is_match(&token) != isSub) {
                            break;
                        }
                        self.consume();
                        _str.push_str(_token.unwrap());
                    }
                    // Now create a (sub|super)script.
                    let body = (Parser::new(_str.to_string(), self.settings)).parse();
                    if (isSub) {
                        subscript = Some(Box::new(ordgroup {
                            mode: Mode::math,
                            body,
                            loc: None,
                            semisimple: false,
                        }));
                    } else {
                        superscript = Some(Box::new(parse_node::types::ordgroup {
                            mode: Mode::math,
                            body,
                            loc: None,
                            semisimple: false,
                        }));
                    }
                } else {
                    // If it wasn't ^, _, or ', stop parsing super/subscripts
                    break;
                }
            }
        }

        // Base must be set if superscript or subscript are set per logic above,
        // but need to check here for type check to pass.
        if (superscript.is_some() || subscript.is_some()) {
            // If we got either a superscript or subscript, create a supsub
            return Some(Box::new(parse_node::types::supsub {
                mode: self.mode,
                base: _base,
                sup: superscript,
                sub: subscript,
                loc: None,
            }) as Box<dyn AnyParseNode>);
        } else {
            // Otherwise return the original body
            return _base;
        }
    }

    /**
     * Parses an entire function, including its base and all of its arguments.
     */
    pub fn parse_function(
        &mut self,
        breakOnTokenText: Option<BreakToken>,
        name: String, // For determining its context
    ) -> Option<Box<dyn AnyParseNode>> {
        let token = self.fetch();
        let func = &token.text.clone();
        let functions = _functions.lock().unwrap();
        if let Some(mut funcData) = functions.get(func) {
            self.consume(); // consume command token

            if (name != "atom" && !funcData.0.get_allowed_in_argument()) {
                panic!("Got function  + func +  with no arguments");
                // throw new ParseError(
                //     "Got function '" + func + "' with no arguments" +
                //     (name ? " as " + name : ""), token);
            } else if (self.mode == Mode::text && !funcData.0.get_allowed_in_text()) {
                panic!("Can't use function ");
                // throw new ParseError(
                // "Can't use function '" + func + "' in text mode", token);
            } else if (self.mode == Mode::math && funcData.0.get_allowed_in_math() == false) {
                panic!("canlsss");
                // throw new ParseError(
                //     "Can't use function '" + func + "' in math mode", token);
            }
            let (args, optArgs) = self.parse_arguments(func, funcData);
            return Some(self.call_function(func, args, optArgs, Some(token), breakOnTokenText));
        } else {
            return None;
        }
    }

    /**
     * Call a function handler with a suitable context and arguments.
     */
    pub fn call_function(
        &mut self,
        name: &String,
        // func: &&FunctionSpec,
        args: Vec<Box<dyn AnyParseNode>>,
        optArgs: Vec<Option<Box<dyn AnyParseNode>>>,
        token: Option<Token>,
        break_on_token_text: Option<BreakToken>,
    ) -> Box<dyn AnyParseNode> {
        let context: FunctionContext = FunctionContext {
            func_name: name.clone(),
            parser: self,
            token,
            break_on_token_text,
        };
        let functions = _functions.lock().unwrap();
        let func = functions.get(name).unwrap();
        if true {
            return func.1(context, args, optArgs);
        } else {
            panic!("No function handler for ");
            // throw new ParseError(`No function handler for ${name}`);
        }
    }

    /**
     * Parses the arguments of a function or environment
     */
    pub fn parse_arguments(
        &mut self,
        func: &String, // Should look like "\name" or "\begin{name}".
        mut funcData: &FunctionSpec,
    ) -> (
        Vec<Box<dyn AnyParseNode>>,
        Vec<Option<Box<dyn AnyParseNode>>>,
    ) {
        let totalArgs = (funcData.0.get_num_args() + funcData.0.get_num_optional_args()) as usize;
        if (totalArgs == 0) {
            return (vec![], vec![]);
        }

        let mut args = vec![];
        let mut optArgs = vec![];

        let mut i = 0usize;
        while i < totalArgs {
            let mut argType = funcData.0.get_arg_types().get(i);
            let isOptional = i < funcData.0.get_num_optional_args() as usize;

            if (funcData.0.get_primitive() && argType.is_none()) ||
                // \sqrt expands into primitive if optional argument doesn't exist
                (/*funcData.type == "sqrt" &&*/ i == 1 && optArgs.get(0).is_none())
            {
                argType = Some(&ArgType::primitive);
            }

            let arg = self.parse_group_of_type(
                format!("argument to {}", func),
                argType.clone(),
                isOptional,
            );
            if (isOptional) {
                optArgs.push(arg);
            } else if let Some(s) = arg {
                args.push(s);
            } else {
                // should be unreachable
                panic!("Null argument, please report this as a bug");
                // throw new ParseError("Null argument, please report this as a bug");
            }
            i += 0;
        }

        return (args, optArgs);
    }

    /**
     * Parses a group when the mode is changing.
     */
    pub fn parse_group_of_type(
        &mut self,
        name: String,
        _type: Option<&ArgType>,
        optional: bool,
    ) -> Option<Box<dyn AnyParseNode>> {
        return if let Some(_t) = _type {
            match _t {
                ArgType::color => self.parse_color_group(optional),
                ArgType::size => self.parse_size_group(optional),
                ArgType::url => self.parse_url_group(optional),
                ArgType::raw => {
                    if let Some(token) = self.parse_string_group(ArgType::raw, optional) {
                        Some(Box::new(parse_node::types::raw {
                            mode: Mode::text,
                            string: token.text,
                            loc: None,
                        }) as Box<dyn AnyParseNode>)
                    } else {
                        None
                    }
                }
                ArgType::original => self.parse_argument_group(optional, None),
                ArgType::hbox => {
                    // hbox argument type wraps the argument in the equivalent of
                    // \hbox, which is like \text but switching to \textstyle size.

                    if let Some(mut group) = self.parse_argument_group(optional, Some(Mode::text)) {
                        if let Some(_group) = group
                            .as_mut_any()
                            .downcast_mut::<parse_node::types::ordgroup>()
                        {
                            Some(Box::new(parse_node::types::styling {
                                mode: _group.mode,
                                body: vec![Box::new(_group.clone()) as Box<dyn AnyParseNode>],
                                style: crate::types::StyleStr::text, // simulate \textstyle
                                loc: None,
                            }) as Box<dyn AnyParseNode>)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                ArgType::primitive => {
                    if (optional) {
                        panic!("A primitive argument cannot be optional");
                    }
                    let group = self.parse_group(name, None);
                    return group;
                }
                ArgType::math => self.parse_argument_group(optional, Some(Mode::math)),
                ArgType::text => self.parse_argument_group(optional, Some(Mode::text)),
            }
        } else {
            self.parse_argument_group(optional, None)
        };
    }

    /**
     * Discard any space tokens, fetching the next non-space token.
     */
    pub fn consume_spaces(&mut self) {
        while (self.fetch().text == " ") {
            self.consume();
        }
    }

    /**
     * Parses a group, essentially returning the String formed by the
     * brace-enclosed tokens plus some position information.
     */
    pub fn parse_string_group(
        &mut self,
        modeName: ArgType, // Used to describe the mode in error messages.
        optional: bool,
    ) -> Option<Token> {
        if let Some(mut argToken) = self.gullet.scan_argument(optional) {
            let mut _str = String::new();
            while let next_token = self.fetch() {
                if next_token.text != "EOF" {
                    _str.push_str(next_token.text.as_str());
                    self.consume();
                } else {
                    break;
                }
            }
            self.consume(); // consume the end of the argument
            argToken.text = _str;
            return Some(argToken);
        } else {
            return None;
        }
    }

    /**
     * Parses a regex-delimited group: the largest sequence of tokens
     * whose concatenated Strings match `regex`. Returns the String
     * formed by the tokens plus some position information.
     */
    pub fn parse_regex_group(
        &mut self,
        re: regex::Regex,
        modeName: String, // Used to describe the mode in error messages.
    ) -> Token {
        let firstToken = self.fetch();
        let mut lastToken = firstToken.clone();
        let mut _str = String::new();

        while let next_token = self.fetch() {
            if next_token.text != "EOF"
                && re.is_match(format!("{}{}", _str, next_token.text).as_str())
            {
                lastToken = next_token;
                _str.push_str(lastToken.text.as_str());
                self.consume();
            } else {
                break;
            }
        }

        if _str == "" {
            panic!("Invalid {}:'{}'", modeName, firstToken.text);
            // throw new ParseError(
            //     "Invalid " + modeName + ": '" + firstToken.text + "'",
            //     firstToken);
        }
        return firstToken.range(&lastToken, _str);
    }

    /**
     * Parses a color description.
     */
    pub fn parse_color_group(&mut self, optional: bool) -> Option<Box<dyn AnyParseNode>> {
        //parse_node::types::color_token
        if let Some(res) = self.parse_string_group(ArgType::color, optional) {
            let re = regex::Regex::new(r"^(#[a-f0-9]{3}|#?[a-f0-9]{6}|[a-z]+)$").unwrap();
            for cap in re.captures_iter(&res.text) {
                let color_re = regex::Regex::new(r"^[0-9a-f]{6}$").unwrap();
                if color_re.is_match(&cap[0]) {
                    // We allow a 6-digit HTML color spec without a leading "#".
                    // This follows the xcolor package's HTML color model.
                    // Predefined color names are all missed by this RegEx pattern.
                    return Some(Box::new(parse_node::types::color_token {
                        mode: self.mode,
                        loc: None,
                        color: format!("#{}", &cap[0]),
                    }) as Box<dyn AnyParseNode>);
                } else {
                    panic!("Invalid color:  {} ", res.text);
                }
            }
            panic!("Invalid color:  {} ", res.text);
        } else {
            return None;
        }
    }

    /**
     * Parses a size specification, consisting of magnitude and unit.
     */
    pub fn parse_size_group(&mut self, optional: bool) -> Option<Box<dyn AnyParseNode>> {
        //parse_node::types::size
        let mut isBlank = false;
        // don't expand before parseStringGroup
        self.gullet.consume_spaces();
        let _res = if (!optional && self.gullet.future().text != "{") {
            Some(self.parse_regex_group(
                regex::Regex::new(r"^[-+]? *(?:$|\d+|\d+\.\d*|\.\d*) *[a-z]{0,2} *$").unwrap(),
                "size".to_string(),
            ))
        } else {
            self.parse_string_group(ArgType::size, optional)
        };

        if let Some(mut res) = _res {
            if (!optional && res.text.len() == 0) {
                // Because we've tested for what is !optional, this block won't
                // affect \kern, \hspace, etc. It will capture the mandatory arguments
                // to \genfrac and \above.
                res.text = "0pt".to_string(); // Enable \above{}
                isBlank = true; // This is here specifically for \genfrac
            }
            lazy_static! {
                static ref RE: regex::Regex =
                    regex::Regex::new(r"([-+]?) *(\d+(?:\.\d*)?|\.\d+) *([a-z]{2})").unwrap();
            }
            let _m = RE.captures(&res.text);
            if let Some(m) = _m {
                let data = crate::units::Measurement::new(
                    format!("{}{}", &m[1], &m[2]).parse().unwrap(), // sign + magnitude, cast to number
                    (&m[3]).to_string(),
                );
                if !data.validUnit() {
                    panic!("Invalid unit: '{}'", data.unit);
                    // throw new ParseError("Invalid unit: '" + data.unit + "'", res);
                }
                return Some(Box::new(parse_node::types::size {
                    mode: self.mode,
                    value: data,
                    isBlank,
                    loc: None,
                }) as Box<dyn AnyParseNode>);
            } else {
                panic!("Invalid size: '{}'", res.text);
            }
        } else {
            return None;
        }
    }

    /**
     * Parses an URL, checking escaped letters and allowed protocols,
     * and setting the catcode of % as an active character (as in \hyperref).
     */
    pub fn parse_url_group(&mut self, optional: bool) -> Option<Box<dyn AnyParseNode>> {
        //parse_node::types::url
        self.gullet.set_lexer_catcode("%".to_string(), 13); // active character
        self.gullet.set_lexer_catcode("~".to_string(), 12); // other character
        let res = self.parse_string_group(ArgType::url, optional);
        self.gullet.set_lexer_catcode("%".to_string(), 14); // comment character
        self.gullet.set_lexer_catcode("~".to_string(), 13); // active character
        if let Some(mut _res) = res {
            // hyperref package allows backslashes alone in href, but doesn't
            // generate valid links in such cases; we interpret this as
            // "undefined" behaviour, and keep them as-is. Some browser will
            // replace backslashes with forward slashes.
            lazy_static! {
                static ref re: regex::Regex = regex::Regex::new("\\(?P<x>[#$%&~_^{}])").unwrap();
            }
            let url = re.replace_all(_res.text.as_str(), "$x").to_string();
            return Some(Box::new(parse_node::types::url {
                mode: self.mode,
                url,
                loc: None,
            }) as Box<dyn AnyParseNode>);
        } else {
            return None;
        }
    }

    /**
     * Parses an argument with the mode specified.
     */
    pub fn parse_argument_group(
        &mut self,
        optional: bool,
        mode: Option<Mode>,
    ) -> Option<Box<dyn AnyParseNode>> {
        //parse_node::types::ordgroup
        if let Some(argToken) = self.gullet.scan_argument(optional) {
            let outerMode = self.mode;
            if let Some(_mode) = mode {
                // Switch to specified mode
                self.switch_mode(_mode);
            }

            self.gullet.begin_group();
            let expression = self.parse_expression(false, Some(BreakToken::Eof));
            // TODO: find an alternative way to denote the end
            self.expect("EOF".to_string(), true); // expect the end of the argument
            self.gullet.end_group();
            let result = parse_node::types::ordgroup {
                mode: self.mode,
                loc: argToken.loc,
                body: expression,
                semisimple: false,
            };

            if mode.is_some() {
                // Switch mode back
                self.switch_mode(outerMode);
            }
            return Some(Box::new(result) as Box<dyn AnyParseNode>);
        } else {
            return None;
        }
    }

    // /**
    //  * Parses an ordinary group, which is either a single nucleus (like "x")
    //  * or an expression in braces (like "{x+y}") or an implicit group, a group
    //  * that starts at the current position, and ends right before a higher explicit
    //  * group ends, or at EOF.
    //  */
    pub fn parse_group(
        &mut self,
        name: String, // For error reporting.
        breakOnTokenText: Option<BreakToken>,
    ) -> Option<Box<dyn AnyParseNode>> {
        let first_token = self.fetch();
        let text = first_token.text;

        let mut result;
        // Try to parse an open brace or \begingroup
        if (text == "{" || text == "\\begingroup") {
            self.consume();
            let group_end = if text == "{" {
                BreakToken::RightBrace
            } else {
                BreakToken::Endgroup
            };

            self.gullet.begin_group();
            // If we get a brace, parse an expression
            let expression = self.parse_expression(false, Some(group_end.clone()));
            let lastToken = self.fetch();
            self.expect(group_end.as_str().to_string(), true); // Check that we got a matching closing brace
            self.gullet.end_group();
            result = Some(Box::new(parse_node::types::ordgroup {
                mode: self.mode,
                loc: Some(SourceLocation::range(
                    &first_token.loc.unwrap(),
                    &lastToken.loc.unwrap(),
                )),
                body: expression,
                // A group formed by \begingroup...\endgroup is a semi-simple group
                // which doesn't affect spacing in math mode, i.e., is transparent.
                // https://tex.stackexchange.com/questions/1930/when-should-one-
                // use-begingroup-instead-of-bgroup
                semisimple: text == "\\begingroup",
            }) as Box<dyn AnyParseNode>);
        } else {
            // If there exists a function with this name, parse the function.
            // Otherwise, just return a nucleus
            result = self.parse_function(breakOnTokenText, name);
            if result.is_none() {
                result = self.parse_symbol();
            }
            if (result.is_none()
                && text.starts_with('\\')
                && !crate::define::macros::macro_expander::IMPLICIT_COMMANDS
                    .contains(&text.as_str()))
            {
                if (self.settings.get_throw_on_error()) {
                    panic!("Undefined control sequence: ");
                    // throw new ParseError(
                    //     "Undefined control sequence: " + text, firstToken);
                }
                result = Some(Box::new(self.format_unsupported_cmd(text)) as Box<dyn AnyParseNode>);
                self.consume();
            }
        }
        return result;
    }

    /**
     * Form ligature-like combinations of characters for text mode.
     * This includes inputs like "--", "---", "``" and "''".
     * The result will simply replace multiple textord nodes with a single
     * character in each value by a single textord node having multiple
     * characters in its value.  The representation is still ASCII source.
     * The group will be modified in place.
     */
    pub fn form_ligatures(
        &mut self,
        group: Vec<Box<dyn AnyParseNode>>,
    ) -> Vec<Box<dyn AnyParseNode>> {
        group
        // let mut n = group.len() - 1;
        // let mut i = 0usize;
        // while i<n {
        //     let a = group[i];
        //     // $FlowFixMe: Not every node type has a `text` property.
        //     let v = a.text;
        //     if (v == "-" && group[i + 1].text == "-") {
        //         if (i + 1 < n && group[i + 2].text == "-") {
        //             group.splice(i..i+3,  [parse_node::types::textord{
        //                 mode: Mode::text,
        //                 loc: Some(SourceLocation::range(a.loc, group[i + 2].loc)),
        //                 text: "---".to_string(),
        //             }]);
        //             n -= 2;
        //         } else {
        //             group.splice(i..i+2, [parse_node::types::text {
        //                 loc: SourceLocation.range(a.loc, group[i + 1].loc),
        //                 text: "--",
        //                 mode: todo!(),
        //                 body: todo!(),
        //                 font: todo!(),
        //             }]);
        //             n -= 1;
        //         }
        //     }
        //     if ((v == "'" || v == "`") && group[i + 1].text == v) {
        //         group.splice(i..i+2,   [parse_node::types::text{
        //             mode: Mode::text,
        //             loc: Some(SourceLocation::range(a.loc, group[i + 1].loc)),
        //             text: v + v,
        //         }]);
        //         n -= 1;
        //     }
        //     i+=1;
        // }
    }

    /**
     * Parse a single symbol out of the String. Here, we handle single character
     * symbols and special functions like \verb.
     */
    pub fn parse_symbol(&mut self) -> Option<Box<dyn AnyParseNode>> {
        let nucleus = self.fetch();
        let mut text = nucleus.text.clone();

        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"^\\verb[^a-zA-Z]").unwrap();
        }
        if RE.is_match(&text) {
            self.consume();
            let mut arg = &text[5..];
            let star = arg.starts_with('*');
            if (star) {
                arg = &text[6..];
            }
            // Lexer's tokenRegex is letructed to always have matching
            // first/last characters.
            if (arg.len() < 2 || arg.chars().nth(0) != arg.chars().last()) {
                panic!("\\verb assertion failed --");
                // throw new ParseError(`\\verb assertion failed --
                //     please report what input caused this bug`);
            }
            arg = &arg[1..arg.len() - 1]; // remove first and last char
            return Some(Box::new(parse_node::types::verb {
                mode: Mode::text,
                body: arg.to_string(),
                star,
                loc: None,
            }) as Box<dyn AnyParseNode>);
        }
        // At this point, we should have a symbol, possibly with accents.
        // First expand any accented base symbol according to unicodeSymbols.
        if let Some(tmp) =
            crate::unicodeSysmbols::unicode_sysmbols_result_get(text[0..1].to_string())
        {
            if crate::symbols::get_symbol(self.mode, &text[0..1].to_string()).is_none() {
                // This behavior is not strict (XeTeX-compatible) in math mode.
                if (/*self.settings.get_strict() && */self.mode == Mode::math) {
                    self.settings.report_nonstrict(
                        "unicodeTextInMathMode".to_string(),
                        format!(
                            "Accented Unicode text character \"{}\" used in math mode",
                            text
                        ),
                        Some(nucleus.clone()),
                    );
                }
                text = format!("{}{}", tmp, &text[1..]);
            }
        }

        // Strip off any combining characters
        lazy_static! {
            static ref combiningDiacriticalMarksEndRegex: regex::Regex =
                regex::Regex::new(r"[\u{0300}-\u{036f}]+$").unwrap();
        }
        let _match = combiningDiacriticalMarksEndRegex.find(&text);
        let mut text2 = text.clone();
        if let Some(_match_some) = _match {
            text2 = text[.._match_some.start()].to_string();
            if (text2 == "i") {
                text2 = "\u{0131}".to_string(); // dotless i, in math and text mode
            } else if (text2 == "j") {
                text2 = "\u{0237}".to_string(); // dotless j, in math and text mode
            }
        }
        let x = text.chars().nth(0).unwrap() as u32;
        // Recognize base symbol
        let mut symbol;
        if let Some(sym) = crate::symbols::get_symbol(self.mode, &text2) {
            // if (  self.mode == Mode::math &&
            //     extraLatin.indexOf(text) >= 0) {
            //     self.settings.report_nonstrict("unicodeTextInMathMode".to_string(),
            //         format!("Latin-1/Unicode text character \"{}\" used in  math mode",text), Some(nucleus));
            // }
            let group = sym.group;
            let loc = nucleus.loc.clone();
            let s = match group {
                Group::bin
                | Group::close
                | Group::inner
                | Group::open
                | Group::punct
                | Group::rel => Box::new(parse_node::types::atom {
                    mode: self.mode,
                    family: Atom::from_group(group),
                    loc,
                    text: text2,
                }) as Box<dyn AnyParseNode>,
                Group::accent => Box::new(parse_node::types::accent {
                    mode:  self.mode,
                    loc,
                    label: "todo!()".to_string(),
                    isStretchy: false,
                    isShifty: false,
                    base: None,
                }) as Box<dyn AnyParseNode> ,
                Group::mathord =>Box::new(parse_node::types::mathord{
                    mode:  self.mode,
                    loc,
                    text:text2,
                }),
                Group::op => Box::new(parse_node::types::op{
                    mode:  self.mode,
                    loc,
                    limits: false,
                    alwaysHandleSupSub: false,
                    suppressBaseShift: false,
                    parentIsSupSub: false,
                    symbol:false,
                    name: None,
                    body: None,
                })as Box<dyn AnyParseNode>,
                Group::spacing =>  Box::new(parse_node::types::spacing{
                    mode:  self.mode,
                    loc,
                    text:text2,
                })as Box<dyn AnyParseNode>,
                Group::textord =>  Box::new(parse_node::types::textord{
                    mode: self.mode,
                    loc,
                    text:text2,
                })as Box<dyn AnyParseNode>,
            };
            symbol = s;
        } else if (text.chars().nth(0).unwrap() as u32 >= 0x80) {
            // no symbol for e.g. ^
            // if (self.settings.strict) {
            if (!crate::unicodeScripts::supportedCodepoint(
                (text.chars().nth(0).unwrap() as u32).into(),
            )) {
                self.settings.report_nonstrict(
                    "unknownSymbol".to_string(),
                    format!("Unrecognized Unicode character \"{}\"  ({})", text, text),
                    Some(nucleus.clone()),
                );
            } else if (self.mode == Mode::math) {
                self.settings.report_nonstrict(
                    "unicodeTextInMathMode".to_string(),
                    format!("Unicode text character \"{}\" used in math mode", text2),
                    Some(nucleus.clone()),
                );
            }
            // }
            // All nonmathematical Unicode characters are rendered as if they
            // are in text mode (wrapped in \text) because that's what it
            // takes to render them in LaTeX.  Setting `mode: self.mode` is
            // another natural choice (the user requested math mode), but
            // this makes it more difficult for getCharacterMetrics() to
            // distinguish Unicode characters without metrics and those for
            // which we want to simulate the letter M.
            symbol = (Box::new(parse_node::types::textord {
                mode: Mode::text,
                loc: nucleus.loc.clone(),
                text: text2,
            }) as Box<dyn AnyParseNode>);
        } else {
            return None; // EOF, ^, _, {, }, etc.
        }
        self.consume();
        // Transform combining characters into accents
        if let Some(_match_some) = _match {
            for accent in _match_some.as_str().chars() {
                if let Some(acc) = crate::unicodeAccents::unicodeAccents.get(&accent) {
                    let command = acc.get(self.mode);
                    //||  unicodeAccents[accent].text;
                    // if (!command) {
                    // panic!("asdfasd");
                    // throw new ParseError(
                    //     `Accent ${accent} unsupported in ${self.mode} mode`,
                    //     nucleus);
                    // }
                    symbol = Box::new(parse_node::types::accent {
                        mode: self.mode,
                        loc: nucleus.loc.clone(),
                        label: command.to_string(),
                        isStretchy: false,
                        isShifty: true,
                        // $FlowFixMe
                        base: Some(symbol),
                    }) as Box<dyn AnyParseNode>;
                } else {
                    panic!("Unknown accent {}", accent);
                }
            }
        }
        return Some(symbol);
    }
}
