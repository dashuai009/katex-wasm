use super::public::MacroDefinition;
use crate::symbols::{get_symbol, public::Group};
use crate::token::Token;
use crate::types::Mode;
use super::macro_expander::MacroExpander;

fn new_me(tokens: Vec<Token>, num_args: i32) -> MacroDefinition {
    MacroDefinition::MacroExpansion(super::public::MacroExpansion {
        tokens,
        num_args,
        delimiters: None,
        unexpandable: false,
    })
}

fn tokens_from_texts(texts: &[&str]) -> Vec<Token> {
    texts
        .iter()
        .map(|text| Token::new((*text).to_string(), None))
        .collect()
}

fn expand_braket_tokens(
    mut arg_tokens: Vec<Token>,
    single_middle: &[&str],
    double_middle: Option<&[&str]>,
    replace_only_first: bool,
) -> Vec<Token> {
    arg_tokens.reverse();
    let mut expanded = Vec::new();
    let mut depth = 0i32;
    let mut used_special_middle = false;
    let mut i = 0usize;

    while i < arg_tokens.len() {
        let token = arg_tokens[i].clone();

        if token.text == "{" {
            depth += 1;
            expanded.push(token);
            i += 1;
            continue;
        }
        if token.text == "}" {
            depth -= 1;
            expanded.push(token);
            i += 1;
            continue;
        }

        let can_replace = depth == 0 && (!replace_only_first || !used_special_middle);
        if can_replace {
            if token.text == "\\|" {
                if let Some(double_middle_tokens) = double_middle {
                    expanded.extend(tokens_from_texts(double_middle_tokens));
                    used_special_middle = true;
                    i += 1;
                    continue;
                }
            } else if token.text == "|" {
                if let Some(double_middle_tokens) = double_middle {
                    if i + 1 < arg_tokens.len() && arg_tokens[i + 1].text == "|" {
                        expanded.extend(tokens_from_texts(double_middle_tokens));
                        used_special_middle = true;
                        i += 2;
                        continue;
                    }
                }

                expanded.extend(tokens_from_texts(single_middle));
                used_special_middle = true;
                i += 1;
                continue;
            }
        }

        expanded.push(token);
        i += 1;
    }

    expanded
}

fn braket_macro(context: &mut MacroExpander) -> MacroDefinition {
    match context.consume_arg(None) {
        Ok(arg) => {
            let mut expanded = tokens_from_texts(&["\\left", "\\langle"]);
            expanded.extend(expand_braket_tokens(
                arg.tokens,
                &["\\,", "\\middle", "\\vert", "\\,"],
                Some(&["\\,", "\\middle", "\\vert", "\\,"]),
                false,
            ));
            expanded.extend(tokens_from_texts(&["\\right", "\\rangle"]));
            expanded.reverse();
            new_me(expanded, 0)
        }
        Err(err) => report_macro_error(context, err.msg, err.loc),
    }
}

fn set_macro(context: &mut MacroExpander) -> MacroDefinition {
    match context.consume_arg(None) {
        Ok(arg) => {
            let mut expanded = tokens_from_texts(&["\\left", "\\{", "\\:"]);
            expanded.extend(expand_braket_tokens(
                arg.tokens,
                &["\\;", "\\middle", "\\vert", "\\;"],
                Some(&["\\;", "\\middle", "\\Vert", "\\;"]),
                true,
            ));
            expanded.extend(tokens_from_texts(&["\\:", "\\right", "\\}"]));
            expanded.reverse();
            new_me(expanded, 0)
        }
        Err(err) => report_macro_error(context, err.msg, err.loc),
    }
}

fn set_small_macro(context: &mut MacroExpander) -> MacroDefinition {
    match context.consume_arg(None) {
        Ok(arg) => {
            let mut expanded = tokens_from_texts(&["\\{", "\\,"]);
            expanded.extend(expand_braket_tokens(arg.tokens, &["\\mid"], None, true));
            expanded.extend(tokens_from_texts(&["\\,", "\\}"]));
            expanded.reverse();
            new_me(expanded, 0)
        }
        Err(err) => report_macro_error(context, err.msg, err.loc),
    }
}

fn tag_literal_macro(context: &mut MacroExpander) -> MacroDefinition {
    if context.macros.get("\\df@tag").is_some() {
        return report_macro_error(context, "Multiple \\tag".to_string(), None);
    }

    match context.consume_arg(None) {
        Ok(arg) => {
            let mut expanded = tokens_from_texts(&["\\gdef", "\\df@tag", "{", "\\text", "{"]);
            let mut body = arg.tokens;
            body.reverse();
            expanded.extend(body);
            expanded.extend(tokens_from_texts(&["}", "}"]));
            expanded.reverse();
            new_me(expanded, 0)
        }
        Err(err) => report_macro_error(context, err.msg, err.loc),
    }
}

fn space_after_dots(next: &str) -> bool {
    matches!(
        next,
        ")" | "]"
            | "\\rbrack"
            | "\\}"
            | "\\rbrace"
            | "\\rangle"
            | "\\rceil"
            | "\\rfloor"
            | "\\rgroup"
            | "\\rmoustache"
            | "\\right"
            | "\\bigr"
            | "\\biggr"
            | "\\Bigr"
            | "\\Biggr"
            | "$"
            | ";"
            | "."
            | ","
    )
}

fn cdots_macro(context: &mut MacroExpander) -> MacroDefinition {
    let next = context.future().text;
    if space_after_dots(&next) {
        MacroDefinition::Str("\\@cdots\\,".to_string())
    } else {
        MacroDefinition::Str("\\@cdots".to_string())
    }
}

fn dotso_macro(context: &mut MacroExpander) -> MacroDefinition {
    let next = context.future().text;
    if space_after_dots(&next) {
        MacroDefinition::Str("\\ldots\\,".to_string())
    } else {
        MacroDefinition::Str("\\ldots".to_string())
    }
}

fn dotsc_macro(context: &mut MacroExpander) -> MacroDefinition {
    let next = context.future().text;
    if space_after_dots(&next) && next != "," {
        MacroDefinition::Str("\\ldots\\,".to_string())
    } else {
        MacroDefinition::Str("\\ldots".to_string())
    }
}

fn dots_macro(context: &mut MacroExpander) -> MacroDefinition {
    let next = context.expand_after_future().text;
    let dots = match next.as_str() {
        "," => "\\dotsc",
        "\\not" => "\\dotsb",
        "+" | "=" | "<" | ">" | "-" | "*" | ":" => "\\dotsb",
        "\\DOTSB" | "\\coprod" | "\\bigvee" | "\\bigwedge" | "\\biguplus" | "\\bigcap"
        | "\\bigcup" | "\\prod" | "\\sum" | "\\bigotimes" | "\\bigoplus"
        | "\\bigodot" | "\\bigsqcup" | "\\And" | "\\longrightarrow"
        | "\\Longrightarrow" | "\\longleftarrow" | "\\Longleftarrow"
        | "\\longleftrightarrow" | "\\Longleftrightarrow" | "\\mapsto"
        | "\\longmapsto" | "\\hookrightarrow" | "\\doteq" | "\\mathbin"
        | "\\mathrel" | "\\relbar" | "\\Relbar" | "\\xrightarrow"
        | "\\xleftarrow" => "\\dotsb",
        "\\DOTSI" | "\\int" | "\\oint" | "\\iint" | "\\iiint" | "\\iiiint"
        | "\\idotsint" => "\\dotsi",
        "\\DOTSX" => "\\dotsx",
        _ if next.starts_with("\\not") => "\\dotsb",
        _ if matches!(
            get_symbol(Mode::math, &next).map(|symbol| symbol.group),
            Some(Group::bin | Group::rel)
        ) => "\\dotsb",
        _ => "\\dotso",
    };
    MacroDefinition::Str(dots.to_string())
}

fn hspace_macro(context: &mut MacroExpander) -> MacroDefinition {
    if context.future().text == "*" {
        context.pop_token();
        MacroDefinition::Str("\\@hspacer{#1}".to_string())
    } else {
        MacroDefinition::Str("\\@hspace{#1}".to_string())
    }
}

fn digit_to_number(text: &str) -> Option<u32> {
    match text {
        "0" => Some(0),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        "a" | "A" => Some(10),
        "b" | "B" => Some(11),
        "c" | "C" => Some(12),
        "d" | "D" => Some(13),
        "e" | "E" => Some(14),
        "f" | "F" => Some(15),
        _ => None,
    }
}

fn char_macro(context: &mut MacroExpander) -> MacroDefinition {
    let mut token = context.pop_token();
    let mut base: Option<u32> = None;
    let number = if token.text == "'" {
        base = Some(8);
        token = context.pop_token();
        None
    } else if token.text == "\"" {
        base = Some(16);
        token = context.pop_token();
        None
    } else if token.text == "`" {
        token = context.pop_token();
        if token.text == "EOF" {
            return report_macro_error(
                context,
                "\\char` missing argument".to_string(),
                token.loc.clone(),
            );
        }
        let code = if token.text.starts_with('\\') {
            token.text.chars().nth(1).map(|ch| ch as u32)
        } else {
            token.text.chars().next().map(|ch| ch as u32)
        };
        match code {
            Some(code) => Some(code),
            None => {
                return report_macro_error(
                    context,
                    "\\char` missing argument".to_string(),
                    token.loc.clone(),
                );
            }
        }
    } else {
        base = Some(10);
        None
    };

    let number = if let Some(number) = number {
        number
    } else {
        let base = base.unwrap();
        let mut number = match digit_to_number(&token.text) {
            Some(number) if number < base => number,
            _ => {
                return report_macro_error(
                    context,
                    format!("Invalid base-{base} digit {}", token.text),
                    token.loc.clone(),
                );
            }
        };

        while let Some(digit) = digit_to_number(&context.future().text) {
            if digit >= base {
                break;
            }
            number *= base;
            number += digit;
            context.pop_token();
        }
        number
    };

    MacroDefinition::Str(format!("\\@char{{{number}}}"))
}

fn firstoftwo_macro(context: &mut MacroExpander) -> MacroDefinition {
    match context.consume_args(2, None) {
        Ok(args) => new_me(args[0].clone(), 0),
        Err(err) => report_macro_error(context, err.msg, err.loc),
    }
}

fn secondoftwo_macro(context: &mut MacroExpander) -> MacroDefinition {
    match context.consume_args(2, None) {
        Ok(args) => new_me(args[1].clone(), 0),
        Err(err) => report_macro_error(context, err.msg, err.loc),
    }
}

fn ifnextchar_macro(context: &mut MacroExpander) -> MacroDefinition {
    match context.consume_args(3, None) {
        Ok(args) => {
            context.consume_spaces();
            let next_token = context.future();
            if args[0].len() == 1 && args[0][0].text == next_token.text {
                new_me(args[1].clone(), 0)
            } else {
                new_me(args[2].clone(), 0)
            }
        }
        Err(err) => report_macro_error(context, err.msg, err.loc),
    }
}

fn report_macro_error(
    context: &mut MacroExpander,
    msg: String,
    loc: Option<crate::sourceLocation::SourceLocation>,
) -> MacroDefinition {
    context.report_parse_error(msg, loc);
    MacroDefinition::Str(String::new())
}

fn read_macro_definition_arg(context: &mut MacroExpander) -> Option<super::public::MacroArg> {
    match context.consume_arg(None) {
        Ok(arg) => Some(arg),
        Err(err) => {
            context.report_parse_error(err.msg, err.loc);
            None
        }
    }
}

fn newcommand_impl(
    context: &mut MacroExpander,
    exists_ok: bool,
    nonexists_ok: bool,
    skip_if_exists: bool,
) -> MacroDefinition {
    let Some(name_arg) = read_macro_definition_arg(context) else {
        return MacroDefinition::Str(String::new());
    };
    if name_arg.tokens.len() != 1 {
        return report_macro_error(
            context,
            "\\newcommand's first argument must be a macro name".to_string(),
            None,
        );
    }
    let name = name_arg.tokens[0].text.clone();
    let exists = context.is_defined(&name);
    if exists && !exists_ok {
        return report_macro_error(
            context,
            format!(
                "\\newcommand{{{}}} attempting to redefine {}; use \\renewcommand",
                name, name
            ),
            None,
        );
    }
    if !exists && !nonexists_ok {
        return report_macro_error(
            context,
            format!(
                "\\renewcommand{{{}}} when command {} does not yet exist; use \\newcommand",
                name, name
            ),
            None,
        );
    }

    let mut num_args = 0;
    let Some(mut body_arg) = read_macro_definition_arg(context) else {
        return MacroDefinition::Str(String::new());
    };
    if body_arg.tokens.len() == 1 && body_arg.tokens[0].text == "[" {
        let mut arg_text = String::new();
        loop {
            let token = context.expand_next_token();
            if token.text == "]" {
                break;
            }
            if token.text == "EOF" {
                return report_macro_error(
                    context,
                    "Unexpected end of input in a macro argument, expected ']'".to_string(),
                    token.loc.clone(),
                );
            }
            arg_text.push_str(token.text.as_str());
        }
        if !arg_text.chars().all(|ch| ch.is_ascii_whitespace() || ch.is_ascii_digit())
            || arg_text.trim().is_empty()
        {
            return report_macro_error(
                context,
                format!("Invalid number of arguments: {}", arg_text),
                None,
            );
        }
        num_args = arg_text.trim().parse::<i32>().unwrap_or(0);
        let Some(next_body_arg) = read_macro_definition_arg(context) else {
            return MacroDefinition::Str(String::new());
        };
        body_arg = next_body_arg;
    }

    if !(exists && skip_if_exists) {
        context
            .macros
            .set(&name, Some(new_me(body_arg.tokens, num_args)), false);
    }
    MacroDefinition::Str(String::new())
}

fn newcommand_macro(context: &mut MacroExpander) -> MacroDefinition {
    newcommand_impl(context, false, true, false)
}

fn renewcommand_macro(context: &mut MacroExpander) -> MacroDefinition {
    newcommand_impl(context, true, false, false)
}

fn providecommand_macro(context: &mut MacroExpander) -> MacroDefinition {
    newcommand_impl(context, true, true, true)
}

pub fn create_macro_map() -> crate::Namespace::Mapping<MacroDefinition> {
    let mut res = std::collections::HashMap::from([
        //////////////////////////////////////////////////////////////////////
        // array.js
        (
            "\\nonumber".to_string(),
            MacroDefinition::Str("\\gdef\\@eqnsw{0}".to_string()),
        ),
        (
            "\\notag".to_string(),
            MacroDefinition::Str("\\nonumber".to_string()),
        ),
        //////////////////////////////////////////////////////////////////////
        // macro tools
        (
            "\\noexpand".to_string(),
            MacroDefinition::MacroContext(|context| {
                // The expansion is the token itself; but that token is interpreted
                // as if its meaning were ‘\relax’ if it is a control sequence that
                // would ordinarily be expanded by TeX’s expansion rules.
                let mut t = context.pop_token();
                if (context.is_expandable(&t.text)) {
                    t.noexpand = true;
                    t.treatAsRelax = true;
                }
                return new_me(vec![t], 0);
            }),
        ),
        //                                                       defineMacro("\\expandafter", function(context) {
        //                                                           // TeX first reads the token that comes immediately after \expandafter,
        //                                                           // without expanding it; let’s call this token t. Then TeX reads the
        //                                                           // token that comes after t (and possibly more tokens, if that token
        //                                                           // has an argument), replacing it by its expansion. Finally TeX puts
        //                                                           // t back in front of that expansion.
        //                                                           const t = context.popToken();
        //                                                           context.expandOnce(true); // expand only an expandable token
        //                                                           return {tokens: [t], numArgs: 0};
        //                                                       });
        //
        (
            "\\@firstoftwo".to_string(),
            MacroDefinition::MacroContext(firstoftwo_macro),
        ),
        (
            "\\@secondoftwo".to_string(),
            MacroDefinition::MacroContext(secondoftwo_macro),
        ),
        (
            "\\@ifnextchar".to_string(),
            MacroDefinition::MacroContext(ifnextchar_macro),
        ),
        (
            "\\@ifstar".to_string(),
            MacroDefinition::Str("\\@ifnextchar *{\\@firstoftwo{#1}}".to_string()),
        ),
        //
        // LaTeX's \TextOrMath{#1}{#2} expands to #1 in text mode, #2 in math mode
        (
            "\\TextOrMath".to_string(),
            MacroDefinition::MacroContext(|context| {
                let args = context.consume_args(2, None).unwrap();
                return new_me(
                    args[if context.mode == Mode::text { 0 } else { 1 }].clone(),
                    0,
                );
            }),
        ),
        (
            "\\cdots".to_string(),
            MacroDefinition::MacroContext(cdots_macro),
        ),
        (
            "\\dotsb".to_string(),
            MacroDefinition::Str("\\cdots".to_string()),
        ),
        (
            "\\dotsm".to_string(),
            MacroDefinition::Str("\\cdots".to_string()),
        ),
        (
            "\\dotsi".to_string(),
            MacroDefinition::Str("\\!\\cdots".to_string()),
        ),
        (
            "\\dots".to_string(),
            MacroDefinition::MacroContext(dots_macro),
        ),
        (
            "\\dotso".to_string(),
            MacroDefinition::MacroContext(dotso_macro),
        ),
        (
            "\\dotsc".to_string(),
            MacroDefinition::MacroContext(dotsc_macro),
        ),
        (
            "\\dotsx".to_string(),
            MacroDefinition::Str("\\ldots\\,".to_string()),
        ),
        (
            "\\DOTSI".to_string(),
            MacroDefinition::Str("\\relax".to_string()),
        ),
        (
            "\\DOTSB".to_string(),
            MacroDefinition::Str("\\relax".to_string()),
        ),
        (
            "\\DOTSX".to_string(),
            MacroDefinition::Str("\\relax".to_string()),
        ),
        //
        //
        //     // Lookup table for parsing numbers in base 8 through 16
        //     const digitToNumber = {
        //         "0": 0, "1": 1, "2": 2, "3": 3, "4": 4, "5": 5, "6": 6, "7": 7, "8": 8,
        //         "9": 9, "a": 10, "A": 10, "b": 11, "B": 11, "c": 12, "C": 12,
        //         "d": 13, "D": 13, "e": 14, "E": 14, "f": 15, "F": 15,
        //     };
        //
        // // TeX \char makes a literal character (catcode 12) using the following forms:
        // // (see The TeXBook, p. 43)
        // //   \char123  -- decimal
        // //   \char'123 -- octal
        // //   \char"123 -- hex
        // //   \char`x   -- character that can be written (i.e. isn't active)
        // //   \char`\x  -- character that cannot be written (e.g. %)
        // // These all refer to characters from the font, so we turn them into special
        // // calls to a function \@char dealt with in the Parser.
        //     defineMacro("\\char", function(context) {
        //         let token = context.popToken();
        //         let base;
        //         let number = '';
        //         if (token.text === "'") {
        //             base = 8;
        //             token = context.popToken();
        //         } else if (token.text === '"') {
        //             base = 16;
        //             token = context.popToken();
        //         } else if (token.text === "`") {
        //             token = context.popToken();
        //             if (token.text[0] === "\\") {
        //                 number = token.text.charCodeAt(1);
        //             } else if (token.text === "EOF") {
        //                 throw new ParseError("\\char` missing argument");
        //             } else {
        //                 number = token.text.charCodeAt(0);
        //             }
        //         } else {
        //             base = 10;
        //         }
        //         if (base) {
        //             // Parse a number in the given base, starting with first `token`.
        //             number = digitToNumber[token.text];
        //             if (number == null || number >= base) {
        //                 throw new ParseError(`Invalid base-${base} digit ${token.text}`);
        //             }
        //             let digit;
        //             while ((digit = digitToNumber[context.future().text]) != null &&
        //                 digit < base) {
        //                 number *= base;
        //                 number += digit;
        //                 context.popToken();
        //             }
        //         }
        //         return `\\@char{${number}}`;
        //     });
        ("\\char".to_string(), MacroDefinition::MacroContext(char_macro)),
        //
        //     // \newcommand{\macro}[args]{definition}
        // // \renewcommand{\macro}[args]{definition}
        // // TODO: Optional arguments: \newcommand{\macro}[args][default]{definition}
        //     const newcommand = (context, existsOK: boolean, nonexistsOK: boolean) => {
        //         let arg = context.consumeArg().tokens;
        //         if (arg.length !== 1) {
        //             throw new ParseError(
        //                 "\\newcommand's first argument must be a macro name");
        //         }
        //         const name = arg[0].text;
        //
        //         const exists = context.isDefined(name);
        //         if (exists && !existsOK) {
        //             throw new ParseError(`\\newcommand{${name}} attempting to redefine ` +
        //             `${name}; use \\renewcommand`);
        //         }
        //         if (!exists && !nonexistsOK) {
        //             throw new ParseError(`\\renewcommand{${name}} when command ${name} ` +
        //                                  `does not yet exist; use \\newcommand`);
        //         }
        //
        //         let numArgs = 0;
        //         arg = context.consumeArg().tokens;
        //         if (arg.length === 1 && arg[0].text === "[") {
        //             let argText = '';
        //             let token = context.expandNextToken();
        //             while (token.text !== "]" && token.text !== "EOF") {
        //                 // TODO: Should properly expand arg, e.g., ignore {}s
        //                 argText += token.text;
        //                 token = context.expandNextToken();
        //             }
        //             if (!argText.match(/^\s*[0-9]+\s*$/)) {
        //             throw new ParseError(`Invalid number of arguments: ${argText}`);
        //             }
        //             numArgs = parseInt(argText);
        //             arg = context.consumeArg().tokens;
        //         }
        //
        //         // Final arg is the expansion of the macro
        //         context.macros.set(name, {
        //             tokens: arg,
        //             numArgs,
        //         });
        //         return '';
        //     };
        (
            "\\newcommand".to_string(),
            MacroDefinition::MacroContext(newcommand_macro),
        ),
        (
            "\\renewcommand".to_string(),
            MacroDefinition::MacroContext(renewcommand_macro),
        ),
        (
            "\\providecommand".to_string(),
            MacroDefinition::MacroContext(providecommand_macro),
        ),
        //
        // // terminal (console) tools
        //     defineMacro("\\message", (context) => {
        //         const arg = context.consumeArgs(1)[0];
        //         // eslint-disable-next-line no-console
        //         console.log(arg.reverse().map(token => token.text).join(""));
        //         return '';
        //     });
        //     defineMacro("\\errmessage", (context) => {
        //         const arg = context.consumeArgs(1)[0];
        //         // eslint-disable-next-line no-console
        //         console.error(arg.reverse().map(token => token.text).join(""));
        //         return '';
        //     });
        //     defineMacro("\\show", (context) => {
        //         const tok = context.popToken();
        //         const name = tok.text;
        //         // eslint-disable-next-line no-console
        //         console.log(tok, context.macros.get(name), functions[name],
        //                     symbols.math[name], symbols.text[name]);
        //         return '';
        //     });
        //
        // //////////////////////////////////////////////////////////////////////
        // // Grouping
        // // \let\bgroup={ \let\egroup=}
        //     defineMacro("\\bgroup", "{");
        //     defineMacro("\\egroup", "}");
        //
        // Symbols from latex.ltx:
        // \def~{\nobreakspace{}}
        (
            "~".to_string(),
            MacroDefinition::Str("\\nobreakspace".to_string()),
        ),
        // \def\lq{`}
        ("\\lq".to_string(), MacroDefinition::Str("`".to_string())),
        // \def\rq{'}
        ("\\rq".to_string(), MacroDefinition::Str("'".to_string())),
        ("\\aa".to_string(), MacroDefinition::Str("\\r a".to_string())),
        ("\\AA".to_string(), MacroDefinition::Str("\\r A".to_string())),
        //
        // // Copyright (C) and registered (R) symbols. Use raw symbol in MathML.
        // // \DeclareTextCommandDefault{\textcopyright}{\textcircled{c}}
        // // \DeclareTextCommandDefault{\textregistered}{\textcircled{%
        // //      \check@mathfonts\fontsize\sf@size\z@\math@fontsfalse\selectfont R}}
        // // \DeclareRobustCommand{\copyright}{%
        // //    \ifmmode{\nfss@text{\textcopyright}}\else\textcopyright\fi}
        //     defineMacro("\\textcopyright", "\\html@mathml{\\textcircled{c}}{\\char`©}");
        //     defineMacro("\\copyright",
        //                 "\\TextOrMath{\\textcopyright}{\\text{\\textcopyright}}");
        //     defineMacro("\\textregistered",
        //                 "\\html@mathml{\\textcircled{\\scriptsize R}}{\\char`®}");
        (
            "\\textcopyright".to_string(),
            MacroDefinition::Str("\\html@mathml{\\textcircled{c}}{\\char`©}".to_string()),
        ),
        (
            "\\copyright".to_string(),
            MacroDefinition::Str(
                "\\TextOrMath{\\textcopyright}{\\text{\\textcopyright}}".to_string(),
            ),
        ),
        (
            "\\textregistered".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\textcircled{\\scriptsize R}}{\\char`®}".to_string(),
            ),
        ),
        //
        // // Characters omitted from Unicode range 1D400–1D7FF
        //     defineMacro("\u212C", "\\mathscr{B}");  // script
        //     defineMacro("\u2130", "\\mathscr{E}");
        //     defineMacro("\u2131", "\\mathscr{F}");
        //     defineMacro("\u210B", "\\mathscr{H}");
        //     defineMacro("\u2110", "\\mathscr{I}");
        //     defineMacro("\u2112", "\\mathscr{L}");
        //     defineMacro("\u2133", "\\mathscr{M}");
        //     defineMacro("\u211B", "\\mathscr{R}");
        //     defineMacro("\u212D", "\\mathfrak{C}");  // Fraktur
        //     defineMacro("\u210C", "\\mathfrak{H}");
        //     defineMacro("\u2128", "\\mathfrak{Z}");
        ("ℬ".to_string(), MacroDefinition::Str("\\mathscr{B}".to_string())),
        ("ℰ".to_string(), MacroDefinition::Str("\\mathscr{E}".to_string())),
        ("ℱ".to_string(), MacroDefinition::Str("\\mathscr{F}".to_string())),
        ("ℋ".to_string(), MacroDefinition::Str("\\mathscr{H}".to_string())),
        ("ℐ".to_string(), MacroDefinition::Str("\\mathscr{I}".to_string())),
        ("ℒ".to_string(), MacroDefinition::Str("\\mathscr{L}".to_string())),
        ("ℳ".to_string(), MacroDefinition::Str("\\mathscr{M}".to_string())),
        ("ℛ".to_string(), MacroDefinition::Str("\\mathscr{R}".to_string())),
        ("ℭ".to_string(), MacroDefinition::Str("\\mathfrak{C}".to_string())),
        ("ℌ".to_string(), MacroDefinition::Str("\\mathfrak{H}".to_string())),
        ("ℨ".to_string(), MacroDefinition::Str("\\mathfrak{Z}".to_string())),
        //
        // // Define \Bbbk with a macro that works in both HTML and MathML.
        //     defineMacro("\\Bbbk", "\\Bbb{k}");
        ("\\Bbbk".to_string(), MacroDefinition::Str("\\Bbb{k}".to_string())),
        //
        // // Unicode middle dot
        // // The KaTeX fonts do not contain U+00B7. Instead, \cdotp displays
        // // the dot at U+22C5 and gives it punct spacing.
        //     defineMacro("\u00b7", "\\cdotp");
        ("·".to_string(), MacroDefinition::Str("\\cdotp".to_string())),
        //
        // // \llap and \rlap render their contents in text mode
        (
            "\\llap".to_string(),
            MacroDefinition::Str("\\mathllap{\\textrm{#1}}".to_string()),
        ),
        (
            "\\rlap".to_string(),
            MacroDefinition::Str("\\mathrlap{\\textrm{#1}}".to_string()),
        ),
        (
            "\\clap".to_string(),
            MacroDefinition::Str("\\mathclap{\\textrm{#1}}".to_string()),
        ),
        //
        // // \mathstrut from the TeXbook, p 360
        (
            "\\mathstrut".to_string(),
            MacroDefinition::Str("\\vphantom{(}".to_string()),
        ),
        //
        // \underbar from TeXbook p 353
        (
            "\\underbar".to_string(),
            MacroDefinition::Str("\\underline{\\text{#1}}".to_string()),
        ),
        //
        // // \not is defined by base/fontmath.ltx via
        // // \DeclareMathSymbol{\not}{\mathrel}{symbols}{"36}
        // // It's thus treated like a \mathrel, but defined by a symbol that has zero
        // // width but extends to the right.  We use \rlap to get that spacing.
        // // For MathML we write U+0338 here. buildMathML.js will then do the overlay.
        //     defineMacro("\\not", '\\html@mathml{\\mathrel{\\mathrlap\\@not}}{\\char"338}');
        (
            "\\not".to_string(),
            MacroDefinition::Str("\\mathrel{\\mathrlap\\@not}".to_string()),
        ),
        //
        // // Negated symbols from base/fontmath.ltx:
        // // \def\neq{\not=} \let\ne=\neq
        // // \DeclareRobustCommand
        // //   \notin{\mathrel{\m@th\mathpalette\c@ncel\in}}
        // // \def\c@ncel#1#2{\m@th\ooalign{$\hfil#1\mkern1mu/\hfil$\crcr$#1#2$}}
        // defineMacro("\\neq", "\\html@mathml{\\mathrel{\\not=}}{\\mathrel{\\char`≠}}");
        // defineMacro("\\ne", "\\neq");
        // defineMacro("\u2260", "\\neq");
        (
            "\\neq".to_string(),
            MacroDefinition::Str("\\mathrel{\\not=}".to_string()),
        ),
        (
            "\\ne".to_string(),
            MacroDefinition::Str("\\neq".to_string()),
        ),
        (
            "≠".to_string(),
            MacroDefinition::Str("\\neq".to_string()),
        ),
        (
            "\\notin".to_string(),
            MacroDefinition::Str("\\mathrel{{\\in}\\mathllap{/\\mskip1mu}}".to_string()),
        ),
        ("∉".to_string(), MacroDefinition::Str("\\notin".to_string())),
        //
        // // Unicode stacked relations
        // defineMacro("\u2258", "\\html@mathml{" +
        //     "\\mathrel{=\\kern{-1em}\\raisebox{0.4em}{$\\scriptsize\\frown$}}" +
        //     "}{\\mathrel{\\char`\u2258}}");
        // defineMacro("\u2259",
        //     "\\html@mathml{\\stackrel{\\tiny\\wedge}{=}}{\\mathrel{\\char`\u2258}}");
        // defineMacro("\u225A",
        //     "\\html@mathml{\\stackrel{\\tiny\\vee}{=}}{\\mathrel{\\char`\u225A}}");
        // defineMacro("\u225B",
        //     "\\html@mathml{\\stackrel{\\scriptsize\\star}{=}}" +
        //     "{\\mathrel{\\char`\u225B}}");
        // defineMacro("\u225D",
        //     "\\html@mathml{\\stackrel{\\tiny\\mathrm{def}}{=}}" +
        //     "{\\mathrel{\\char`\u225D}}");
        // defineMacro("\u225E",
        //     "\\html@mathml{\\stackrel{\\tiny\\mathrm{m}}{=}}" +
        //     "{\\mathrel{\\char`\u225E}}");
        // defineMacro("\u225F",
        //     "\\html@mathml{\\stackrel{\\tiny?}{=}}{\\mathrel{\\char`\u225F}}");
        //
        // // Misc Unicode
        // defineMacro("\u27C2", "\\perp");
        // defineMacro("\u203C", "\\mathclose{!\\mkern-0.8mu!}");
        // defineMacro("\u220C", "\\notni");
        // defineMacro("\u231C", "\\ulcorner");
        // defineMacro("\u231D", "\\urcorner");
        // defineMacro("\u231E", "\\llcorner");
        // defineMacro("\u231F", "\\lrcorner");
        // defineMacro("\u00A9", "\\copyright");
        // defineMacro("\u00AE", "\\textregistered");
        // defineMacro("\uFE0F", "\\textregistered");
        ("∌".to_string(), MacroDefinition::Str("\\notni".to_string())),
        ("⌜".to_string(), MacroDefinition::Str("\\ulcorner".to_string())),
        ("⌝".to_string(), MacroDefinition::Str("\\urcorner".to_string())),
        ("⌞".to_string(), MacroDefinition::Str("\\llcorner".to_string())),
        ("⌟".to_string(), MacroDefinition::Str("\\lrcorner".to_string())),
        ("©".to_string(), MacroDefinition::Str("\\copyright".to_string())),
        ("®".to_string(), MacroDefinition::Str("\\textregistered".to_string())),
        ("\u{fe0f}".to_string(), MacroDefinition::Str("\\textregistered".to_string())),
        //
        // // The KaTeX fonts have corners at codepoints that don't match Unicode.
        // // For MathML purposes, use the Unicode code point.
        // defineMacro("\\ulcorner", "\\html@mathml{\\@ulcorner}{\\mathop{\\char\"231c}}");
        //         defineMacro("\\urcorner", "\\html@mathml{\\@urcorner}{\\mathop{\\char\"231d}}");
        //         defineMacro("\\llcorner", "\\html@mathml{\\@llcorner}{\\mathop{\\char\"231e}}");
        //         defineMacro("\\lrcorner", "\\html@mathml{\\@lrcorner}{\\mathop{\\char\"231f}}");
        (
            "\\ulcorner".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@ulcorner}{\\mathop{\\char\"231c}}".to_string()),
        ),
        (
            "\\urcorner".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@urcorner}{\\mathop{\\char\"231d}}".to_string()),
        ),
        (
            "\\llcorner".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@llcorner}{\\mathop{\\char\"231e}}".to_string()),
        ),
        (
            "\\lrcorner".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@lrcorner}{\\mathop{\\char\"231f}}".to_string()),
        ),
        //
        // //////////////////////////////////////////////////////////////////////
        // // LaTeX_2ε
        //
        // // \vdots{\vbox{\baselineskip4\p@  \lineskiplimit\z@
        // // \kern6\p@\hbox{.}\hbox{.}\hbox{.}}}
        // // We'll call \varvdots, which gets a glyph from symbols.js.
        // // The zero-width rule gets us an equivalent to the vertical 6pt kern.
        (
            "\\vdots".to_string(),
            MacroDefinition::Str("{\\varvdots\\rule{0pt}{15pt}}".to_string()),
        ),
        (
            "\u{22ee}".to_string(),
            MacroDefinition::Str("\\vdots".to_string()),
        ),
        //
        // //////////////////////////////////////////////////////////////////////
        // // amsmath.sty
        // // http://mirrors.concertpass.com/tex-archive/macros/latex/required/amsmath/amsmath.pdf
        //
        // // Italic Greek capital letters.  AMS defines these with \DeclareMathSymbol,
        // // but they are equivalent to \mathit{\Letter}.
        //         defineMacro("\\varGamma", "\\mathit{\\Gamma}");
        //         defineMacro("\\varDelta", "\\mathit{\\Delta}");
        //         defineMacro("\\varTheta", "\\mathit{\\Theta}");
        //         defineMacro("\\varLambda", "\\mathit{\\Lambda}");
        //         defineMacro("\\varXi", "\\mathit{\\Xi}");
        //         defineMacro("\\varPi", "\\mathit{\\Pi}");
        //         defineMacro("\\varSigma", "\\mathit{\\Sigma}");
        //         defineMacro("\\varUpsilon", "\\mathit{\\Upsilon}");
        //         defineMacro("\\varPhi", "\\mathit{\\Phi}");
        //         defineMacro("\\varPsi", "\\mathit{\\Psi}");
        //         defineMacro("\\varOmega", "\\mathit{\\Omega}");
        ("\\varGamma".to_string(), MacroDefinition::Str("\\mathit{\\Gamma}".to_string())),
        ("\\varDelta".to_string(), MacroDefinition::Str("\\mathit{\\Delta}".to_string())),
        ("\\varTheta".to_string(), MacroDefinition::Str("\\mathit{\\Theta}".to_string())),
        ("\\varLambda".to_string(), MacroDefinition::Str("\\mathit{\\Lambda}".to_string())),
        ("\\varXi".to_string(), MacroDefinition::Str("\\mathit{\\Xi}".to_string())),
        ("\\varPi".to_string(), MacroDefinition::Str("\\mathit{\\Pi}".to_string())),
        ("\\varSigma".to_string(), MacroDefinition::Str("\\mathit{\\Sigma}".to_string())),
        ("\\varUpsilon".to_string(), MacroDefinition::Str("\\mathit{\\Upsilon}".to_string())),
        ("\\varPhi".to_string(), MacroDefinition::Str("\\mathit{\\Phi}".to_string())),
        ("\\varPsi".to_string(), MacroDefinition::Str("\\mathit{\\Psi}".to_string())),
        ("\\varOmega".to_string(), MacroDefinition::Str("\\mathit{\\Omega}".to_string())),
        //
        // //\newcommand{\substack}[1]{\subarray{c}#1\endsubarray}
        //         defineMacro("\\substack", "\\begin{subarray}{c}#1\\end{subarray}");
        (
            "\\substack".to_string(),
            MacroDefinition::Str("\\begin{subarray}{c}#1\\end{subarray}".to_string()),
        ),
        //
        // // \renewcommand{\colon}{\nobreak\mskip2mu\mathpunct{}\nonscript
        // // \mkern-\thinmuskip{:}\mskip6muplus1mu\relax}
        //         defineMacro("\\colon", "\\nobreak\\mskip2mu\\mathpunct{}" +
        //         "\\mathchoice{\\mkern-3mu}{\\mkern-3mu}{}{}{:}\\mskip6mu\\relax");
        (
            "\\colon".to_string(),
            MacroDefinition::Str(
                "\\nobreak\\mskip2mu\\mathpunct{}\\mathchoice{\\mkern-3mu}{\\mkern-3mu}{}{}{:}\\mskip6mu\\relax".to_string(),
            ),
        ),
        //
        // // \newcommand{\boxed}[1]{\fbox{\m@th$\displaystyle#1$}}
        //         defineMacro("\\boxed", "\\fbox{$\\displaystyle{#1}$}");
        (
            "\\boxed".to_string(),
            MacroDefinition::Str("\\fbox{$\\displaystyle{#1}$}".to_string()),
        ),
        (
            "\\bra".to_string(),
            MacroDefinition::Str("\\mathinner{\\langle{#1}|}".to_string()),
        ),
        (
            "\\ket".to_string(),
            MacroDefinition::Str("\\mathinner{|{#1}\\rangle}".to_string()),
        ),
        (
            "\\braket".to_string(),
            MacroDefinition::Str("\\mathinner{\\langle{#1}\\rangle}".to_string()),
        ),
        (
            "\\Bra".to_string(),
            MacroDefinition::Str("\\left\\langle#1\\right|".to_string()),
        ),
        (
            "\\Ket".to_string(),
            MacroDefinition::Str("\\left|#1\\right\\rangle".to_string()),
        ),
        (
            "\\Braket".to_string(),
            MacroDefinition::MacroContext(braket_macro),
        ),
        //
        // // \def\iff{\DOTSB\;\Longleftrightarrow\;}
        // // \def\implies{\DOTSB\;\Longrightarrow\;}
        // // \def\impliedby{\DOTSB\;\Longleftarrow\;}
        //         defineMacro("\\iff", "\\DOTSB\\;\\Longleftrightarrow\\;");
        //         defineMacro("\\implies", "\\DOTSB\\;\\Longrightarrow\\;");
        //         defineMacro("\\impliedby", "\\DOTSB\\;\\Longleftarrow\\;");
        (
            "\\iff".to_string(),
            MacroDefinition::Str("\\DOTSB\\;\\Longleftrightarrow\\;".to_string()),
        ),
        (
            "\\implies".to_string(),
            MacroDefinition::Str("\\DOTSB\\;\\Longrightarrow\\;".to_string()),
        ),
        (
            "\\impliedby".to_string(),
            MacroDefinition::Str("\\DOTSB\\;\\Longleftarrow\\;".to_string()),
        ),
        //
        // // AMSMath's automatic \dots, based on \mdots@@ macro.
        //         const dotsByToken = {
        //         ',': '\\dotsc',
        //         '\\not': '\\dotsb',
        //         // \keybin@ checks for the following:
        //         '+': '\\dotsb',
        //         '=': '\\dotsb',
        //         '<': '\\dotsb',
        //         '>': '\\dotsb',
        //         '-': '\\dotsb',
        //         '*': '\\dotsb',
        //         ':': '\\dotsb',
        //         // Symbols whose definition starts with \DOTSB:
        //         '\\DOTSB': '\\dotsb',
        //         '\\coprod': '\\dotsb',
        //         '\\bigvee': '\\dotsb',
        //         '\\bigwedge': '\\dotsb',
        //         '\\biguplus': '\\dotsb',
        //         '\\bigcap': '\\dotsb',
        //         '\\bigcup': '\\dotsb',
        //         '\\prod': '\\dotsb',
        //         '\\sum': '\\dotsb',
        //         '\\bigotimes': '\\dotsb',
        //         '\\bigoplus': '\\dotsb',
        //         '\\bigodot': '\\dotsb',
        //         '\\bigsqcup': '\\dotsb',
        //         '\\And': '\\dotsb',
        //         '\\longrightarrow': '\\dotsb',
        //         '\\Longrightarrow': '\\dotsb',
        //         '\\longleftarrow': '\\dotsb',
        //         '\\Longleftarrow': '\\dotsb',
        //         '\\longleftrightarrow': '\\dotsb',
        //         '\\Longleftrightarrow': '\\dotsb',
        //         '\\mapsto': '\\dotsb',
        //         '\\longmapsto': '\\dotsb',
        //         '\\hookrightarrow': '\\dotsb',
        //         '\\doteq': '\\dotsb',
        //         // Symbols whose definition starts with \mathbin:
        //         '\\mathbin': '\\dotsb',
        //         // Symbols whose definition starts with \mathrel:
        //         '\\mathrel': '\\dotsb',
        //         '\\relbar': '\\dotsb',
        //         '\\Relbar': '\\dotsb',
        //         '\\xrightarrow': '\\dotsb',
        //         '\\xleftarrow': '\\dotsb',
        //         // Symbols whose definition starts with \DOTSI:
        //         '\\DOTSI': '\\dotsi',
        //         '\\int': '\\dotsi',
        //         '\\oint': '\\dotsi',
        //         '\\iint': '\\dotsi',
        //         '\\iiint': '\\dotsi',
        //         '\\iiiint': '\\dotsi',
        //         '\\idotsint': '\\dotsi',
        //         // Symbols whose definition starts with \DOTSX:
        //         '\\DOTSX': '\\dotsx',
        //         };
        //
        //         defineMacro("\\dots", function(context) {
        //         // TODO: If used in text mode, should expand to \textellipsis.
        //         // However, in KaTeX, \textellipsis and \ldots behave the same
        //         // (in text mode), and it's unlikely we'd see any of the math commands
        //         // that affect the behavior of \dots when in text mode.  So fine for now
        //         // (until we support \ifmmode ... \else ... \fi).
        //         let thedots = '\\dotso';
        //         const next = context.expandAfterFuture().text;
        //         if (next in dotsByToken) {
        //         thedots = dotsByToken[next];
        //         } else if (next.slice(0, 4) === '\\not') {
        //         thedots = '\\dotsb';
        //         } else if (next in symbols.math) {
        //         if (utils.contains(['bin', 'rel'], symbols.math[next].group)) {
        //         thedots = '\\dotsb';
        //         }
        //         }
        //         return thedots;
        //         });
        //
        //         const spaceAfterDots = {
        //         // \rightdelim@ checks for the following:
        //         ')': true,
        //         ']': true,
        //         '\\rbrack': true,
        //         '\\}': true,
        //         '\\rbrace': true,
        //         '\\rangle': true,
        //         '\\rceil': true,
        //         '\\rfloor': true,
        //         '\\rgroup': true,
        //         '\\rmoustache': true,
        //         '\\right': true,
        //         '\\bigr': true,
        //         '\\biggr': true,
        //         '\\Bigr': true,
        //         '\\Biggr': true,
        //         // \extra@ also tests for the following:
        //         '$': true,
        //         // \extrap@ checks for the following:
        //         ';': true,
        //         '.': true,
        //         ',': true,
        //     };
        //
        //         defineMacro("\\dotso", function(context) {
        //             const next = context.future().text;
        //             if (next in spaceAfterDots) {
        //                 return "\\ldots\\,";
        //             } else {
        //                 return "\\ldots";
        //             }
        //         });
        //
        //         defineMacro("\\dotsc", function(context) {
        //             const next = context.future().text;
        //             // \dotsc uses \extra@ but not \extrap@, instead specially checking for
        //             // ';' and '.', but doesn't check for ','.
        //             if (next in spaceAfterDots && next !== ',') {
        //                 return "\\ldots\\,";
        //             } else {
        //                 return "\\ldots";
        //             }
        //         });
        //
        //         defineMacro("\\cdots", function(context) {
        //             const next = context.future().text;
        //             if (next in spaceAfterDots) {
        //                 return "\\@cdots\\,";
        //             } else {
        //                 return "\\@cdots";
        //             }
        //         });
        //
        //         defineMacro("\\dotsb", "\\cdots");
        //         defineMacro("\\dotsm", "\\cdots");
        //         defineMacro("\\dotsi", "\\!\\cdots");
        // // amsmath doesn't actually define \dotsx, but \dots followed by a macro
        // // starting with \DOTSX implies \dotso, and then \extra@ detects this case
        // // and forces the added `\,`.
        //         defineMacro("\\dotsx", "\\ldots\\,");
        //
        // // \let\DOTSI\relax
        // // \let\DOTSB\relax
        // // \let\DOTSX\relax
        //         defineMacro("\\DOTSI", "\\relax");
        //         defineMacro("\\DOTSB", "\\relax");
        //         defineMacro("\\DOTSX", "\\relax");
        //
        // Spacing, based on amsmath.sty's override of LaTeX defaults
        // \DeclareRobustCommand{\tmspace}[3]{%
        //   \ifmmode\mskip#1#2\else\kern#1#3\fi\relax}
        (
            "\\tmspace".to_string(),
            MacroDefinition::Str("\\TextOrMath{\\kern#1#3}{\\mskip#1#2}\\relax".to_string()),
        ),
        // \renewcommand{\,}{\tmspace+\thinmuskip{.1667em}}
        // TODO: math mode should use \thinmuskip
        (
            "\\,".to_string(),
            MacroDefinition::Str("\\tmspace+{3mu}{.1667em}".to_string()),
        ),
        // \let\thinspace\,
        (
            "\\thinspace".to_string(),
            MacroDefinition::Str("\\,".to_string()),
        ),
        // \def\>{\mskip\medmuskip}
        // \renewcommand{\:}{\tmspace+\medmuskip{.2222em}}
        // TODO: \> and math mode of \: should use \medmuskip = 4mu plus 2mu minus 4mu
        (
            "\\>".to_string(),
            MacroDefinition::Str("\\mskip{4mu}".to_string()),
        ),
        (
            "\\:".to_string(),
            MacroDefinition::Str("\\tmspace+{4mu}{.2222em}".to_string()),
        ),
        // \let\medspace\:
        (
            "\\medspace".to_string(),
            MacroDefinition::Str("\\:".to_string()),
        ),
        // \renewcommand{\;}{\tmspace+\thickmuskip{.2777em}}
        // TODO: math mode should use \thickmuskip = 5mu plus 5mu
        (
            "\\;".to_string(),
            MacroDefinition::Str("\\tmspace+{5mu}{.2777em}".to_string()),
        ),
        // \let\thickspace\;
        (
            "\\thickspace".to_string(),
            MacroDefinition::Str("\\;".to_string()),
        ),
        // \renewcommand{\!}{\tmspace-\thinmuskip{.1667em}}
        // TODO: math mode should use \thinmuskip
        (
            "\\!".to_string(),
            MacroDefinition::Str("\\tmspace-{3mu}{.1667em}".to_string()),
        ),
        // \let\negthinspace\!
        (
            "\\negthinspace".to_string(),
            MacroDefinition::Str("\\!".to_string()),
        ),
        // \newcommand{\negmedspace}{\tmspace-\medmuskip{.2222em}}
        (
            "\\negmedspace".to_string(),
            MacroDefinition::Str("\\tmspace-{4mu}{.2222em}".to_string()),
        ),
        // \newcommand{\negthickspace}{\tmspace-\thickmuskip{.2777em}}
        (
            "\\negthickspace".to_string(),
            MacroDefinition::Str("\\tmspace-{5mu}{.277em}".to_string()),
        ),
        // \def\enspace{\kern.5em }
        (
            "\\enspace".to_string(),
            MacroDefinition::Str("\\kern.5em ".to_string()),
        ),
        // \def\enskip{\hskip.5em\relax}
        (
            "\\enskip".to_string(),
            MacroDefinition::Str("\\hskip.5em\\relax".to_string()),
        ),
        // \def\quad{\hskip1em\relax}
        (
            "\\quad".to_string(),
            MacroDefinition::Str("\\hskip1em\\relax".to_string()),
        ),
        // \def\qquad{\hskip2em\relax}
        (
            "\\qquad".to_string(),
            MacroDefinition::Str("\\hskip2em\\relax".to_string()),
        ),
        ("\\tag".to_string(), MacroDefinition::Str("\\@ifstar\\tag@literal\\tag@paren".to_string())),
        ("\\tag@paren".to_string(), MacroDefinition::Str("\\tag@literal{({#1})}".to_string())),
        ("\\tag@literal".to_string(), MacroDefinition::MacroContext(tag_literal_macro)),
        //
        // // \renewcommand{\bmod}{\nonscript\mskip-\medmuskip\mkern5mu\mathbin
        // //   {\operator@font mod}\penalty900
        // //   \mkern5mu\nonscript\mskip-\medmuskip}
        // // \newcommand{\pod}[1]{\allowbreak
        // //   \if@display\mkern18mu\else\mkern8mu\fi(#1)}
        // // \renewcommand{\pmod}[1]{\pod{{\operator@font mod}\mkern6mu#1}}
        // // \newcommand{\mod}[1]{\allowbreak\if@display\mkern18mu
        // //   \else\mkern12mu\fi{\operator@font mod}\,\,#1}
        // // TODO: math mode should use \medmuskip = 4mu plus 2mu minus 4mu
        (
            "\\bmod".to_string(),
            MacroDefinition::Str(
                "\\mathchoice{\\mskip1mu}{\\mskip1mu}{\\mskip5mu}{\\mskip5mu}\
\\mathbin{\\rm mod}\
\\mathchoice{\\mskip1mu}{\\mskip1mu}{\\mskip5mu}{\\mskip5mu}"
                    .to_string(),
            ),
        ),
        (
            "\\pod".to_string(),
            MacroDefinition::Str(
                "\\allowbreak\
\\mathchoice{\\mkern18mu}{\\mkern8mu}{\\mkern8mu}{\\mkern8mu}(#1)"
                    .to_string(),
            ),
        ),
        (
            "\\pmod".to_string(),
            MacroDefinition::Str("\\pod{{\\rm mod}\\mkern6mu#1}".to_string()),
        ),
        (
            "\\mod".to_string(),
            MacroDefinition::Str(
                "\\allowbreak\
\\mathchoice{\\mkern18mu}{\\mkern12mu}{\\mkern12mu}{\\mkern12mu}\
{\\rm mod}\\,\\,#1"
                    .to_string(),
            ),
        ),
        //
        // //////////////////////////////////////////////////////////////////////
        // // LaTeX source2e
        //
        // // \expandafter\let\expandafter\@normalcr
        // //     \csname\expandafter\@gobble\string\\ \endcsname
        // // \DeclareRobustCommand\newline{\@normalcr\relax}
        //         defineMacro("\\newline", "\\\\\\relax");
        (
            "\\newline".to_string(),
            MacroDefinition::Str("\\\\\\relax".to_string()),
        ),
        //
        // // \def\TeX{T\kern-.1667em\lower.5ex\hbox{E}\kern-.125emX\@}
        // // TODO: Doesn't normally work in math mode because \@ fails.  KaTeX doesn't
        // // support \@ yet, so that's omitted, and we add \text so that the result
        // // doesn't look funny in math mode.
        //         defineMacro("\\TeX", "\\textrm{\\html@mathml{" +
        //             "T\\kern-.1667em\\raisebox{-.5ex}{E}\\kern-.125emX" +
        //             "}{TeX}}");
        //
        //         // \DeclareRobustCommand{\LaTeX}{L\kern-.36em%
        // //         {\sbox\z@ T%
        // //          \vbox to\ht\z@{\hbox{\check@mathfonts
        // //                               \fontsize\sf@size\z@
        // //                               \math@fontsfalse\selectfont
        // //                               A}%
        // //                         \vss}%
        // //         }%
        // //         \kern-.15em%
        // //         \TeX}
        // // This code aligns the top of the A with the T (from the perspective of TeX's
        // // boxes, though visually the A appears to extend above slightly).
        // // We compute the corresponding \raisebox when A is rendered in \normalsize
        // // \scriptstyle, which has a scale factor of 0.7 (see Options.js).
        //         const latexRaiseA = makeEm(fontMetricsData['Main-Regular']["T".charCodeAt(0)][1] -
        //             0.7 * fontMetricsData['Main-Regular']["A".charCodeAt(0)][1]);
        //         defineMacro("\\LaTeX", "\\textrm{\\html@mathml{" +
        //                     `L\\kern-.36em\\raisebox{${latexRaiseA}}{\\scriptstyle A}` +
        //                     "\\kern-.15em\\TeX}{LaTeX}}");
        //
        // // New KaTeX logo based on tweaking LaTeX logo
        //         defineMacro("\\KaTeX", "\\textrm{\\html@mathml{" +
        //                     `K\\kern-.17em\\raisebox{${latexRaiseA}}{\\scriptstyle A}` +
        //                     "\\kern-.15em\\TeX}{KaTeX}}");
        //
        (
            "\\TeX".to_string(),
            MacroDefinition::Str(
                "\\textrm{\\html@mathml{T\\kern-.1667em\\raisebox{-.5ex}{E}\\kern-.125emX}{TeX}}"
                    .to_string(),
            ),
        ),
        (
            "\\LaTeX".to_string(),
            MacroDefinition::Str(
                "\\textrm{\\html@mathml{L\\kern-.36em\\raisebox{0.205em}{\\scriptstyle A}\\kern-.15em\\TeX}{LaTeX}}"
                    .to_string(),
            ),
        ),
        (
            "\\KaTeX".to_string(),
            MacroDefinition::Str(
                "\\textrm{\\html@mathml{K\\kern-.17em\\raisebox{0.205em}{\\scriptstyle A}\\kern-.15em\\TeX}{KaTeX}}"
                    .to_string(),
            ),
        ),
        //
        // \DeclareRobustCommand\hspace{\@ifstar\@hspacer\@hspace}
        // \def\@hspace#1{\hskip  #1\relax}
        // \def\@hspacer#1{\vrule \@width\z@\nobreak
        //                 \hskip #1\hskip \z@skip}
        (
            "\\hspace".to_string(),
            MacroDefinition::MacroContext(hspace_macro),
        ),
        (
            "\\operatorname".to_string(),
            MacroDefinition::Str("\\@ifstar\\operatornamewithlimits\\operatorname@".to_string()),
        ),
        (
            "\\@hspace".to_string(),
            MacroDefinition::Str("\\hskip #1\\relax".to_string()),
        ),
        (
            "\\@hspacer".to_string(),
            MacroDefinition::Str("\\rule{0pt}{0pt}\\hskip #1\\relax".to_string()),
        ),
        //
        // //////////////////////////////////////////////////////////////////////
        // // mathtools.sty
        //
        // //\providecommand\ordinarycolon{:}
        //         defineMacro("\\ordinarycolon", ":");
        // //\def\vcentcolon{\mathrel{\mathop\ordinarycolon}}
        // //TODO(edemaine): Not yet centered. Fix via \raisebox or #726
        //         defineMacro("\\vcentcolon", "\\mathrel{\\mathop\\ordinarycolon}");
        // // \providecommand*\dblcolon{\vcentcolon\mathrel{\mkern-.9mu}\vcentcolon}
        //         defineMacro("\\dblcolon", "\\html@mathml{" +
        //             "\\mathrel{\\vcentcolon\\mathrel{\\mkern-.9mu}\\vcentcolon}}" +
        //             "{\\mathop{\\char\"2237}}");
        // // \providecommand*\coloneqq{\vcentcolon\mathrel{\mkern-1.2mu}=}
        //         defineMacro("\\coloneqq", "\\html@mathml{" +
        //             "\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}=}}" +
        //             "{\\mathop{\\char\"2254}}"); // ≔
        // // \providecommand*\Coloneqq{\dblcolon\mathrel{\mkern-1.2mu}=}
        //         defineMacro("\\Coloneqq", "\\html@mathml{" +
        //             "\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}=}}" +
        //             "{\\mathop{\\char\"2237\\char\"3d}}");
        // // \providecommand*\coloneq{\vcentcolon\mathrel{\mkern-1.2mu}\mathrel{-}}
        //         defineMacro("\\coloneq", "\\html@mathml{" +
        //             "\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}\\mathrel{-}}}" +
        //             "{\\mathop{\\char\"3a\\char\"2212}}");
        // // \providecommand*\Coloneq{\dblcolon\mathrel{\mkern-1.2mu}\mathrel{-}}
        //         defineMacro("\\Coloneq", "\\html@mathml{" +
        //             "\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}\\mathrel{-}}}" +
        //             "{\\mathop{\\char\"2237\\char\"2212}}");
        // // \providecommand*\eqqcolon{=\mathrel{\mkern-1.2mu}\vcentcolon}
        //         defineMacro("\\eqqcolon", "\\html@mathml{" +
        //             "\\mathrel{=\\mathrel{\\mkern-1.2mu}\\vcentcolon}}" +
        //             "{\\mathop{\\char\"2255}}"); // ≕
        // // \providecommand*\Eqqcolon{=\mathrel{\mkern-1.2mu}\dblcolon}
        //         defineMacro("\\Eqqcolon", "\\html@mathml{" +
        //             "\\mathrel{=\\mathrel{\\mkern-1.2mu}\\dblcolon}}" +
        //             "{\\mathop{\\char\"3d\\char\"2237}}");
        // // \providecommand*\eqcolon{\mathrel{-}\mathrel{\mkern-1.2mu}\vcentcolon}
        //         defineMacro("\\eqcolon", "\\html@mathml{" +
        //             "\\mathrel{\\mathrel{-}\\mathrel{\\mkern-1.2mu}\\vcentcolon}}" +
        //             "{\\mathop{\\char\"2239}}");
        // // \providecommand*\Eqcolon{\mathrel{-}\mathrel{\mkern-1.2mu}\dblcolon}
        //         defineMacro("\\Eqcolon", "\\html@mathml{" +
        //             "\\mathrel{\\mathrel{-}\\mathrel{\\mkern-1.2mu}\\dblcolon}}" +
        //             "{\\mathop{\\char\"2212\\char\"2237}}");
        // // \providecommand*\colonapprox{\vcentcolon\mathrel{\mkern-1.2mu}\approx}
        //         defineMacro("\\colonapprox", "\\html@mathml{" +
        //             "\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}\\approx}}" +
        //             "{\\mathop{\\char\"3a\\char\"2248}}");
        // // \providecommand*\Colonapprox{\dblcolon\mathrel{\mkern-1.2mu}\approx}
        //         defineMacro("\\Colonapprox", "\\html@mathml{" +
        //             "\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}\\approx}}" +
        //             "{\\mathop{\\char\"2237\\char\"2248}}");
        // // \providecommand*\colonsim{\vcentcolon\mathrel{\mkern-1.2mu}\sim}
        //         defineMacro("\\colonsim", "\\html@mathml{" +
        //             "\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}\\sim}}" +
        //             "{\\mathop{\\char\"3a\\char\"223c}}");
        // // \providecommand*\Colonsim{\dblcolon\mathrel{\mkern-1.2mu}\sim}
        //         defineMacro("\\Colonsim", "\\html@mathml{" +
        //             "\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}\\sim}}" +
        //             "{\\mathop{\\char\"2237\\char\"223c}}");
        //
        // // Some Unicode characters are implemented with macros to mathtools functions.
        //         defineMacro("\u2237", "\\dblcolon");  // ::
        //         defineMacro("\u2239", "\\eqcolon");  // -:
        //         defineMacro("\u2254", "\\coloneqq");  // :=
        //         defineMacro("\u2255", "\\eqqcolon");  // =:
        //         defineMacro("\u2A74", "\\Coloneqq");  // ::=
        ("\\ordinarycolon".to_string(), MacroDefinition::Str(":".to_string())),
        (
            "\\vcentcolon".to_string(),
            MacroDefinition::Str("\\mathrel{\\mathop\\ordinarycolon}".to_string()),
        ),
        (
            "\\dblcolon".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{\\vcentcolon\\mathrel{\\mkern-.9mu}\\vcentcolon}}{\\mathop{\\char\"2237}}"
                    .to_string(),
            ),
        ),
        (
            "\\coloneqq".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}=}}{\\mathop{\\char\"2254}}"
                    .to_string(),
            ),
        ),
        (
            "\\Coloneqq".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}=}}{\\mathop{\\char\"2237\\char\"3d}}"
                    .to_string(),
            ),
        ),
        (
            "\\coloneq".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}\\mathrel{-}}}{\\mathop{\\char\"3a\\char\"2212}}"
                    .to_string(),
            ),
        ),
        (
            "\\Coloneq".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}\\mathrel{-}}}{\\mathop{\\char\"2237\\char\"2212}}"
                    .to_string(),
            ),
        ),
        (
            "\\eqqcolon".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{=\\mathrel{\\mkern-1.2mu}\\vcentcolon}}{\\mathop{\\char\"2255}}"
                    .to_string(),
            ),
        ),
        (
            "\\Eqqcolon".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{=\\mathrel{\\mkern-1.2mu}\\dblcolon}}{\\mathop{\\char\"3d\\char\"2237}}"
                    .to_string(),
            ),
        ),
        (
            "\\eqcolon".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{\\mathrel{-}\\mathrel{\\mkern-1.2mu}\\vcentcolon}}{\\mathop{\\char\"2239}}"
                    .to_string(),
            ),
        ),
        (
            "\\Eqcolon".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{\\mathrel{-}\\mathrel{\\mkern-1.2mu}\\dblcolon}}{\\mathop{\\char\"2212\\char\"2237}}"
                    .to_string(),
            ),
        ),
        (
            "\\colonapprox".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}\\approx}}{\\mathop{\\char\"3a\\char\"2248}}"
                    .to_string(),
            ),
        ),
        (
            "\\Colonapprox".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}\\approx}}{\\mathop{\\char\"2237\\char\"2248}}"
                    .to_string(),
            ),
        ),
        (
            "\\colonsim".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}\\sim}}{\\mathop{\\char\"3a\\char\"223c}}"
                    .to_string(),
            ),
        ),
        (
            "\\Colonsim".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}\\sim}}{\\mathop{\\char\"2237\\char\"223c}}"
                    .to_string(),
            ),
        ),
        ("∷".to_string(), MacroDefinition::Str("\\dblcolon".to_string())),
        ("∹".to_string(), MacroDefinition::Str("\\eqcolon".to_string())),
        ("≔".to_string(), MacroDefinition::Str("\\coloneqq".to_string())),
        ("≕".to_string(), MacroDefinition::Str("\\eqqcolon".to_string())),
        ("⩴".to_string(), MacroDefinition::Str("\\Coloneqq".to_string())),
        //
        // //////////////////////////////////////////////////////////////////////
        // // colonequals.sty
        //
        // // Alternate names for mathtools's macros:
        //         defineMacro("\\ratio", "\\vcentcolon");
        //         defineMacro("\\coloncolon", "\\dblcolon");
        //         defineMacro("\\colonequals", "\\coloneqq");
        //         defineMacro("\\coloncolonequals", "\\Coloneqq");
        //         defineMacro("\\equalscolon", "\\eqqcolon");
        //         defineMacro("\\equalscoloncolon", "\\Eqqcolon");
        //         defineMacro("\\colonminus", "\\coloneq");
        //         defineMacro("\\coloncolonminus", "\\Coloneq");
        //         defineMacro("\\minuscolon", "\\eqcolon");
        //         defineMacro("\\minuscoloncolon", "\\Eqcolon");
        // // \colonapprox name is same in mathtools and colonequals.
        //         defineMacro("\\coloncolonapprox", "\\Colonapprox");
        // // \colonsim name is same in mathtools and colonequals.
        //         defineMacro("\\coloncolonsim", "\\Colonsim");
        //
        // // Additional macros, implemented by analogy with mathtools definitions:
        //         defineMacro("\\simcolon",
        //                     "\\mathrel{\\sim\\mathrel{\\mkern-1.2mu}\\vcentcolon}");
        //         defineMacro("\\simcoloncolon",
        //                     "\\mathrel{\\sim\\mathrel{\\mkern-1.2mu}\\dblcolon}");
        //         defineMacro("\\approxcolon",
        //                     "\\mathrel{\\approx\\mathrel{\\mkern-1.2mu}\\vcentcolon}");
        //         defineMacro("\\approxcoloncolon",
        //                     "\\mathrel{\\approx\\mathrel{\\mkern-1.2mu}\\dblcolon}");
        //
        // // Present in newtxmath, pxfonts and txfonts
        //         defineMacro("\\notni", "\\html@mathml{\\not\\ni}{\\mathrel{\\char`\u220C}}");
        ("\\ratio".to_string(), MacroDefinition::Str("\\vcentcolon".to_string())),
        ("\\coloncolon".to_string(), MacroDefinition::Str("\\dblcolon".to_string())),
        (
            "\\colonequals".to_string(),
            MacroDefinition::Str("\\coloneqq".to_string()),
        ),
        (
            "\\coloncolonequals".to_string(),
            MacroDefinition::Str("\\Coloneqq".to_string()),
        ),
        (
            "\\equalscolon".to_string(),
            MacroDefinition::Str("\\eqqcolon".to_string()),
        ),
        (
            "\\equalscoloncolon".to_string(),
            MacroDefinition::Str("\\Eqqcolon".to_string()),
        ),
        (
            "\\colonminus".to_string(),
            MacroDefinition::Str("\\coloneq".to_string()),
        ),
        (
            "\\coloncolonminus".to_string(),
            MacroDefinition::Str("\\Coloneq".to_string()),
        ),
        (
            "\\minuscolon".to_string(),
            MacroDefinition::Str("\\eqcolon".to_string()),
        ),
        (
            "\\minuscoloncolon".to_string(),
            MacroDefinition::Str("\\Eqcolon".to_string()),
        ),
        (
            "\\coloncolonapprox".to_string(),
            MacroDefinition::Str("\\Colonapprox".to_string()),
        ),
        (
            "\\coloncolonsim".to_string(),
            MacroDefinition::Str("\\Colonsim".to_string()),
        ),
        (
            "\\simcolon".to_string(),
            MacroDefinition::Str(
                "\\mathrel{\\sim\\mathrel{\\mkern-1.2mu}\\vcentcolon}".to_string(),
            ),
        ),
        (
            "\\simcoloncolon".to_string(),
            MacroDefinition::Str(
                "\\mathrel{\\sim\\mathrel{\\mkern-1.2mu}\\dblcolon}".to_string(),
            ),
        ),
        (
            "\\approxcolon".to_string(),
            MacroDefinition::Str(
                "\\mathrel{\\approx\\mathrel{\\mkern-1.2mu}\\vcentcolon}".to_string(),
            ),
        ),
        (
            "\\approxcoloncolon".to_string(),
            MacroDefinition::Str(
                "\\mathrel{\\approx\\mathrel{\\mkern-1.2mu}\\dblcolon}".to_string(),
            ),
        ),
        (
            "\\notni".to_string(),
            MacroDefinition::Str("\\html@mathml{\\not\\ni}{\\mathrel{\\char`∌}}".to_string()),
        ),
        (
            "\\stackrel".to_string(),
            MacroDefinition::Str("\\mathrel{\\mathop{#2}\\limits^{#1}}".to_string()),
        ),
        (
            "\\limsup".to_string(),
            MacroDefinition::Str("\\DOTSB\\operatorname*{lim\\,sup}".to_string()),
        ),
        (
            "\\liminf".to_string(),
            MacroDefinition::Str("\\DOTSB\\operatorname*{lim\\,inf}".to_string()),
        ),
        (
            "\\injlim".to_string(),
            MacroDefinition::Str("\\DOTSB\\operatorname*{inj\\,lim}".to_string()),
        ),
        (
            "\\projlim".to_string(),
            MacroDefinition::Str("\\DOTSB\\operatorname*{proj\\,lim}".to_string()),
        ),
        (
            "\\varlimsup".to_string(),
            MacroDefinition::Str("\\DOTSB\\operatorname*{\\overline{lim}}".to_string()),
        ),
        (
            "\\varliminf".to_string(),
            MacroDefinition::Str("\\DOTSB\\operatorname*{\\underline{lim}}".to_string()),
        ),
        (
            "\\varinjlim".to_string(),
            MacroDefinition::Str("\\DOTSB\\operatorname*{\\underrightarrow{lim}}".to_string()),
        ),
        (
            "\\varprojlim".to_string(),
            MacroDefinition::Str("\\DOTSB\\operatorname*{\\underleftarrow{lim}}".to_string()),
        ),
        //
        // //////////////////////////////////////////////////////////////////////
        // // From amsopn.sty
        //         defineMacro("\\injlim", "\\DOTSB\\operatorname*{inj\\,lim}");
        //         defineMacro("\\projlim", "\\DOTSB\\operatorname*{proj\\,lim}");
        //         defineMacro("\\varlimsup", "\\DOTSB\\operatorname*{\\overline{lim}}");
        //         defineMacro("\\varliminf", "\\DOTSB\\operatorname*{\\underline{lim}}");
        //         defineMacro("\\varinjlim", "\\DOTSB\\operatorname*{\\underrightarrow{lim}}");
        //         defineMacro("\\varprojlim", "\\DOTSB\\operatorname*{\\underleftarrow{lim}}");
        //
        // //////////////////////////////////////////////////////////////////////
        // // MathML alternates for KaTeX glyphs in the Unicode private area
        //         defineMacro("\\gvertneqq", "\\html@mathml{\\@gvertneqq}{\u2269}");
        //         defineMacro("\\lvertneqq", "\\html@mathml{\\@lvertneqq}{\u2268}");
        //         defineMacro("\\ngeqq", "\\html@mathml{\\@ngeqq}{\u2271}");
        //         defineMacro("\\ngeqslant", "\\html@mathml{\\@ngeqslant}{\u2271}");
        //         defineMacro("\\nleqq", "\\html@mathml{\\@nleqq}{\u2270}");
        //         defineMacro("\\nleqslant", "\\html@mathml{\\@nleqslant}{\u2270}");
        //         defineMacro("\\nshortmid", "\\html@mathml{\\@nshortmid}{∤}");
        //         defineMacro("\\nshortparallel", "\\html@mathml{\\@nshortparallel}{∦}");
        //         defineMacro("\\nsubseteqq", "\\html@mathml{\\@nsubseteqq}{\u2288}");
        //         defineMacro("\\nsupseteqq", "\\html@mathml{\\@nsupseteqq}{\u2289}");
        //         defineMacro("\\varsubsetneq", "\\html@mathml{\\@varsubsetneq}{⊊}");
        //         defineMacro("\\varsubsetneqq", "\\html@mathml{\\@varsubsetneqq}{⫋}");
        //         defineMacro("\\varsupsetneq", "\\html@mathml{\\@varsupsetneq}{⊋}");
        //         defineMacro("\\varsupsetneqq", "\\html@mathml{\\@varsupsetneqq}{⫌}");
        (
            "\\gvertneqq".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@gvertneqq}{≩}".to_string()),
        ),
        (
            "\\lvertneqq".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@lvertneqq}{≨}".to_string()),
        ),
        (
            "\\ngeqq".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@ngeqq}{≱}".to_string()),
        ),
        (
            "\\ngeqslant".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@ngeqslant}{≱}".to_string()),
        ),
        (
            "\\nleqq".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@nleqq}{≰}".to_string()),
        ),
        (
            "\\nleqslant".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@nleqslant}{≰}".to_string()),
        ),
        (
            "\\nshortmid".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@nshortmid}{∤}".to_string()),
        ),
        (
            "\\nshortparallel".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@nshortparallel}{∦}".to_string()),
        ),
        (
            "\\nsubseteqq".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@nsubseteqq}{⊈}".to_string()),
        ),
        (
            "\\nsupseteqq".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@nsupseteqq}{⊉}".to_string()),
        ),
        (
            "\\varsubsetneq".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@varsubsetneq}{⊊}".to_string()),
        ),
        (
            "\\varsubsetneqq".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@varsubsetneqq}{⫋}".to_string()),
        ),
        (
            "\\varsupsetneq".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@varsupsetneq}{⊋}".to_string()),
        ),
        (
            "\\varsupsetneqq".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@varsupsetneqq}{⫌}".to_string()),
        ),
        (
            "\\argmin".to_string(),
            MacroDefinition::Str("\\DOTSB\\operatorname*{arg\\,min}".to_string()),
        ),
        (
            "\\argmax".to_string(),
            MacroDefinition::Str("\\DOTSB\\operatorname*{arg\\,max}".to_string()),
        ),
        (
            "\\plim".to_string(),
            MacroDefinition::Str("\\DOTSB\\mathop{\\operatorname{plim}}\\limits".to_string()),
        ),
        (
            "\\imath".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@imath}{\u{0131}}".to_string()),
        ),
        (
            "\\jmath".to_string(),
            MacroDefinition::Str("\\html@mathml{\\@jmath}{\u{0237}}".to_string()),
        ),
        ("\\angln".to_string(), MacroDefinition::Str("{\\angl n}".to_string())),
        ("\\blue".to_string(), MacroDefinition::Str("\\textcolor{##6495ed}{#1}".to_string())),
        ("\\orange".to_string(), MacroDefinition::Str("\\textcolor{##ffa500}{#1}".to_string())),
        ("\\pink".to_string(), MacroDefinition::Str("\\textcolor{##ff00af}{#1}".to_string())),
        ("\\red".to_string(), MacroDefinition::Str("\\textcolor{##df0030}{#1}".to_string())),
        ("\\green".to_string(), MacroDefinition::Str("\\textcolor{##28ae7b}{#1}".to_string())),
        ("\\gray".to_string(), MacroDefinition::Str("\\textcolor{gray}{#1}".to_string())),
        ("\\purple".to_string(), MacroDefinition::Str("\\textcolor{##9d38bd}{#1}".to_string())),
        ("\\blueA".to_string(), MacroDefinition::Str("\\textcolor{##ccfaff}{#1}".to_string())),
        ("\\blueB".to_string(), MacroDefinition::Str("\\textcolor{##80f6ff}{#1}".to_string())),
        ("\\blueC".to_string(), MacroDefinition::Str("\\textcolor{##63d9ea}{#1}".to_string())),
        ("\\blueD".to_string(), MacroDefinition::Str("\\textcolor{##11accd}{#1}".to_string())),
        ("\\blueE".to_string(), MacroDefinition::Str("\\textcolor{##0c7f99}{#1}".to_string())),
        ("\\tealA".to_string(), MacroDefinition::Str("\\textcolor{##94fff5}{#1}".to_string())),
        ("\\tealB".to_string(), MacroDefinition::Str("\\textcolor{##26edd5}{#1}".to_string())),
        ("\\tealC".to_string(), MacroDefinition::Str("\\textcolor{##01d1c1}{#1}".to_string())),
        ("\\tealD".to_string(), MacroDefinition::Str("\\textcolor{##01a995}{#1}".to_string())),
        ("\\tealE".to_string(), MacroDefinition::Str("\\textcolor{##208170}{#1}".to_string())),
        ("\\greenA".to_string(), MacroDefinition::Str("\\textcolor{##b6ffb0}{#1}".to_string())),
        ("\\greenB".to_string(), MacroDefinition::Str("\\textcolor{##8af281}{#1}".to_string())),
        ("\\greenC".to_string(), MacroDefinition::Str("\\textcolor{##74cf70}{#1}".to_string())),
        ("\\greenD".to_string(), MacroDefinition::Str("\\textcolor{##1fab54}{#1}".to_string())),
        ("\\greenE".to_string(), MacroDefinition::Str("\\textcolor{##0d923f}{#1}".to_string())),
        ("\\goldA".to_string(), MacroDefinition::Str("\\textcolor{##ffd0a9}{#1}".to_string())),
        ("\\goldB".to_string(), MacroDefinition::Str("\\textcolor{##ffbb71}{#1}".to_string())),
        ("\\goldC".to_string(), MacroDefinition::Str("\\textcolor{##ff9c39}{#1}".to_string())),
        ("\\goldD".to_string(), MacroDefinition::Str("\\textcolor{##e07d10}{#1}".to_string())),
        ("\\goldE".to_string(), MacroDefinition::Str("\\textcolor{##a75a05}{#1}".to_string())),
        ("\\redA".to_string(), MacroDefinition::Str("\\textcolor{##fca9a9}{#1}".to_string())),
        ("\\redB".to_string(), MacroDefinition::Str("\\textcolor{##ff8482}{#1}".to_string())),
        ("\\redC".to_string(), MacroDefinition::Str("\\textcolor{##f9685d}{#1}".to_string())),
        ("\\redD".to_string(), MacroDefinition::Str("\\textcolor{##e84d39}{#1}".to_string())),
        ("\\redE".to_string(), MacroDefinition::Str("\\textcolor{##bc2612}{#1}".to_string())),
        ("\\maroonA".to_string(), MacroDefinition::Str("\\textcolor{##ffbde0}{#1}".to_string())),
        ("\\maroonB".to_string(), MacroDefinition::Str("\\textcolor{##ff92c6}{#1}".to_string())),
        ("\\maroonC".to_string(), MacroDefinition::Str("\\textcolor{##ed5fa6}{#1}".to_string())),
        ("\\maroonD".to_string(), MacroDefinition::Str("\\textcolor{##ca337c}{#1}".to_string())),
        ("\\maroonE".to_string(), MacroDefinition::Str("\\textcolor{##9e034e}{#1}".to_string())),
        ("\\purpleA".to_string(), MacroDefinition::Str("\\textcolor{##ddd7ff}{#1}".to_string())),
        ("\\purpleB".to_string(), MacroDefinition::Str("\\textcolor{##c6b9fc}{#1}".to_string())),
        ("\\purpleC".to_string(), MacroDefinition::Str("\\textcolor{##aa87ff}{#1}".to_string())),
        ("\\purpleD".to_string(), MacroDefinition::Str("\\textcolor{##7854ab}{#1}".to_string())),
        ("\\purpleE".to_string(), MacroDefinition::Str("\\textcolor{##543b78}{#1}".to_string())),
        ("\\mintA".to_string(), MacroDefinition::Str("\\textcolor{##f5f9e8}{#1}".to_string())),
        ("\\mintB".to_string(), MacroDefinition::Str("\\textcolor{##edf2df}{#1}".to_string())),
        ("\\mintC".to_string(), MacroDefinition::Str("\\textcolor{##e0e5cc}{#1}".to_string())),
        ("\\grayA".to_string(), MacroDefinition::Str("\\textcolor{##f6f7f7}{#1}".to_string())),
        ("\\grayB".to_string(), MacroDefinition::Str("\\textcolor{##f0f1f2}{#1}".to_string())),
        ("\\grayC".to_string(), MacroDefinition::Str("\\textcolor{##e3e5e6}{#1}".to_string())),
        ("\\grayD".to_string(), MacroDefinition::Str("\\textcolor{##d6d8da}{#1}".to_string())),
        ("\\grayE".to_string(), MacroDefinition::Str("\\textcolor{##babec2}{#1}".to_string())),
        ("\\grayF".to_string(), MacroDefinition::Str("\\textcolor{##888d93}{#1}".to_string())),
        ("\\grayG".to_string(), MacroDefinition::Str("\\textcolor{##626569}{#1}".to_string())),
        ("\\grayH".to_string(), MacroDefinition::Str("\\textcolor{##3b3e40}{#1}".to_string())),
        ("\\grayI".to_string(), MacroDefinition::Str("\\textcolor{##21242c}{#1}".to_string())),
        ("\\kaBlue".to_string(), MacroDefinition::Str("\\textcolor{##314453}{#1}".to_string())),
        ("\\kaGreen".to_string(), MacroDefinition::Str("\\textcolor{##71B307}{#1}".to_string())),
        //
        // //////////////////////////////////////////////////////////////////////
        // // stmaryrd and semantic
        //
        // // The stmaryrd and semantic packages render the next four items by calling a
        // // glyph. Those glyphs do not exist in the KaTeX fonts. Hence the macros.
        //
        //         defineMacro("\\llbracket", "\\html@mathml{" +
        //             "\\mathopen{[\\mkern-3.2mu[}}" +
        //             "{\\mathopen{\\char`\u27e6}}");
        //         defineMacro("\\rrbracket", "\\html@mathml{" +
        //             "\\mathclose{]\\mkern-3.2mu]}}" +
        //             "{\\mathclose{\\char`\u27e7}}");
        //
        //         defineMacro("\u27e6", "\\llbracket"); // blackboard bold [
        //         defineMacro("\u27e7", "\\rrbracket"); // blackboard bold ]
        //
        //         defineMacro("\\lBrace", "\\html@mathml{" +
        //             "\\mathopen{\\{\\mkern-3.2mu[}}" +
        //             "{\\mathopen{\\char`\u2983}}");
        //         defineMacro("\\rBrace", "\\html@mathml{" +
        //             "\\mathclose{]\\mkern-3.2mu\\}}}" +
        //             "{\\mathclose{\\char`\u2984}}");
        //
        //         defineMacro("\u2983", "\\lBrace"); // blackboard bold {
        //         defineMacro("\u2984", "\\rBrace"); // blackboard bold }
        //
        // // TODO: Create variable sized versions of the last two items. I believe that
        // // will require new font glyphs.
        //
        // // The stmaryrd function `\minuso` provides a "Plimsoll" symbol that
        // // superimposes the characters \circ and \mathminus. Used in chemistry.
        //         defineMacro("\\minuso", "\\mathbin{\\html@mathml{" +
        //             "{\\mathrlap{\\mathchoice{\\kern{0.145em}}{\\kern{0.145em}}" +
        //             "{\\kern{0.1015em}}{\\kern{0.0725em}}\\circ}{-}}}" +
        //             "{\\char`⦵}}");
        //         defineMacro("⦵", "\\minuso");
        (
            "\\llbracket".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathopen{[\\mkern-3.2mu[}}{\\mathopen{\\char`⟦}}".to_string(),
            ),
        ),
        (
            "\\rrbracket".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathclose{]\\mkern-3.2mu]}}{\\mathclose{\\char`⟧}}".to_string(),
            ),
        ),
        ("⟦".to_string(), MacroDefinition::Str("\\llbracket".to_string())),
        ("⟧".to_string(), MacroDefinition::Str("\\rrbracket".to_string())),
        (
            "\\lBrace".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathopen{\\{\\mkern-3.2mu[}}{\\mathopen{\\char`⦃}}".to_string(),
            ),
        ),
        (
            "\\rBrace".to_string(),
            MacroDefinition::Str(
                "\\html@mathml{\\mathclose{]\\mkern-3.2mu\\}}}{\\mathclose{\\char`⦄}}".to_string(),
            ),
        ),
        ("⦃".to_string(), MacroDefinition::Str("\\lBrace".to_string())),
        ("⦄".to_string(), MacroDefinition::Str("\\rBrace".to_string())),
        (
            "\\minuso".to_string(),
            MacroDefinition::Str(
                "\\mathbin{\\html@mathml{{\\mathrlap{\\mathchoice{\\kern{0.145em}}{\\kern{0.145em}}{\\kern{0.1015em}}{\\kern{0.0725em}}\\circ}{-}}}{\\char`⦵}}"
                    .to_string(),
            ),
        ),
        ("⦵".to_string(), MacroDefinition::Str("\\minuso".to_string())),
        //
        // //////////////////////////////////////////////////////////////////////
        // // texvc.sty
        //
        // // The texvc package contains macros available in mediawiki pages.
        // // We omit the functions deprecated at
        // // https://en.wikipedia.org/wiki/Help:Displaying_a_formula#Deprecated_syntax
        //
        // // We also omit texvc's \O, which conflicts with \text{\O}
        //
        //         defineMacro("\\darr", "\\downarrow");
        //         defineMacro("\\dArr", "\\Downarrow");
        //         defineMacro("\\Darr", "\\Downarrow");
        //         defineMacro("\\lang", "\\langle");
        //         defineMacro("\\rang", "\\rangle");
        //         defineMacro("\\uarr", "\\uparrow");
        //         defineMacro("\\uArr", "\\Uparrow");
        //         defineMacro("\\Uarr", "\\Uparrow");
        //         defineMacro("\\N", "\\mathbb{N}");
        //         defineMacro("\\R", "\\mathbb{R}");
        //         defineMacro("\\Z", "\\mathbb{Z}");
        //         defineMacro("\\alef", "\\aleph");
        //         defineMacro("\\alefsym", "\\aleph");
        //         defineMacro("\\Alpha", "\\mathrm{A}");
        //         defineMacro("\\Beta", "\\mathrm{B}");
        //         defineMacro("\\bull", "\\bullet");
        //         defineMacro("\\Chi", "\\mathrm{X}");
        //         defineMacro("\\clubs", "\\clubsuit");
        //         defineMacro("\\cnums", "\\mathbb{C}");
        //         defineMacro("\\Complex", "\\mathbb{C}");
        //         defineMacro("\\Dagger", "\\ddagger");
        //         defineMacro("\\diamonds", "\\diamondsuit");
        //         defineMacro("\\empty", "\\emptyset");
        //         defineMacro("\\Epsilon", "\\mathrm{E}");
        //         defineMacro("\\Eta", "\\mathrm{H}");
        //         defineMacro("\\exist", "\\exists");
        //         defineMacro("\\harr", "\\leftrightarrow");
        //         defineMacro("\\hArr", "\\Leftrightarrow");
        //         defineMacro("\\Harr", "\\Leftrightarrow");
        //         defineMacro("\\hearts", "\\heartsuit");
        //         defineMacro("\\image", "\\Im");
        //         defineMacro("\\infin", "\\infty");
        //         defineMacro("\\Iota", "\\mathrm{I}");
        //         defineMacro("\\isin", "\\in");
        //         defineMacro("\\Kappa", "\\mathrm{K}");
        //         defineMacro("\\larr", "\\leftarrow");
        //         defineMacro("\\lArr", "\\Leftarrow");
        //         defineMacro("\\Larr", "\\Leftarrow");
        //         defineMacro("\\lrarr", "\\leftrightarrow");
        //         defineMacro("\\lrArr", "\\Leftrightarrow");
        //         defineMacro("\\Lrarr", "\\Leftrightarrow");
        //         defineMacro("\\Mu", "\\mathrm{M}");
        //         defineMacro("\\natnums", "\\mathbb{N}");
        //         defineMacro("\\Nu", "\\mathrm{N}");
        //         defineMacro("\\Omicron", "\\mathrm{O}");
        //         defineMacro("\\plusmn", "\\pm");
        //         defineMacro("\\rarr", "\\rightarrow");
        //         defineMacro("\\rArr", "\\Rightarrow");
        //         defineMacro("\\Rarr", "\\Rightarrow");
        //         defineMacro("\\real", "\\Re");
        //         defineMacro("\\reals", "\\mathbb{R}");
        //         defineMacro("\\Reals", "\\mathbb{R}");
        //         defineMacro("\\Rho", "\\mathrm{P}");
        //         defineMacro("\\sdot", "\\cdot");
        //         defineMacro("\\sect", "\\S");
        //         defineMacro("\\spades", "\\spadesuit");
        //         defineMacro("\\sub", "\\subset");
        //         defineMacro("\\sube", "\\subseteq");
        //         defineMacro("\\supe", "\\supseteq");
        //         defineMacro("\\Tau", "\\mathrm{T}");
        //         defineMacro("\\thetasym", "\\vartheta");
        // // TODO: defineMacro("\\varcoppa", "\\\mbox{\\coppa}");
        //         defineMacro("\\weierp", "\\wp");
        //         defineMacro("\\Zeta", "\\mathrm{Z}");
        ("\\darr".to_string(), MacroDefinition::Str("\\downarrow".to_string())),
        ("\\dArr".to_string(), MacroDefinition::Str("\\Downarrow".to_string())),
        ("\\Darr".to_string(), MacroDefinition::Str("\\Downarrow".to_string())),
        ("\\lang".to_string(), MacroDefinition::Str("\\langle".to_string())),
        ("\\rang".to_string(), MacroDefinition::Str("\\rangle".to_string())),
        ("\\uarr".to_string(), MacroDefinition::Str("\\uparrow".to_string())),
        ("\\uArr".to_string(), MacroDefinition::Str("\\Uparrow".to_string())),
        ("\\Uarr".to_string(), MacroDefinition::Str("\\Uparrow".to_string())),
        ("\\N".to_string(), MacroDefinition::Str("\\mathbb{N}".to_string())),
        ("\\R".to_string(), MacroDefinition::Str("\\mathbb{R}".to_string())),
        ("\\Z".to_string(), MacroDefinition::Str("\\mathbb{Z}".to_string())),
        ("\\alef".to_string(), MacroDefinition::Str("\\aleph".to_string())),
        ("\\alefsym".to_string(), MacroDefinition::Str("\\aleph".to_string())),
        ("\\Alpha".to_string(), MacroDefinition::Str("\\mathrm{A}".to_string())),
        ("\\Beta".to_string(), MacroDefinition::Str("\\mathrm{B}".to_string())),
        ("\\bull".to_string(), MacroDefinition::Str("\\bullet".to_string())),
        ("\\Chi".to_string(), MacroDefinition::Str("\\mathrm{X}".to_string())),
        ("\\clubs".to_string(), MacroDefinition::Str("\\clubsuit".to_string())),
        ("\\cnums".to_string(), MacroDefinition::Str("\\mathbb{C}".to_string())),
        ("\\Complex".to_string(), MacroDefinition::Str("\\mathbb{C}".to_string())),
        ("\\Dagger".to_string(), MacroDefinition::Str("\\ddagger".to_string())),
        ("\\diamonds".to_string(), MacroDefinition::Str("\\diamondsuit".to_string())),
        ("\\empty".to_string(), MacroDefinition::Str("\\emptyset".to_string())),
        ("\\Epsilon".to_string(), MacroDefinition::Str("\\mathrm{E}".to_string())),
        ("\\Eta".to_string(), MacroDefinition::Str("\\mathrm{H}".to_string())),
        ("\\exist".to_string(), MacroDefinition::Str("\\exists".to_string())),
        ("\\harr".to_string(), MacroDefinition::Str("\\leftrightarrow".to_string())),
        ("\\hArr".to_string(), MacroDefinition::Str("\\Leftrightarrow".to_string())),
        ("\\Harr".to_string(), MacroDefinition::Str("\\Leftrightarrow".to_string())),
        ("\\hearts".to_string(), MacroDefinition::Str("\\heartsuit".to_string())),
        ("\\image".to_string(), MacroDefinition::Str("\\Im".to_string())),
        ("\\infin".to_string(), MacroDefinition::Str("\\infty".to_string())),
        ("\\Iota".to_string(), MacroDefinition::Str("\\mathrm{I}".to_string())),
        ("\\isin".to_string(), MacroDefinition::Str("\\in".to_string())),
        ("\\Kappa".to_string(), MacroDefinition::Str("\\mathrm{K}".to_string())),
        ("\\larr".to_string(), MacroDefinition::Str("\\leftarrow".to_string())),
        ("\\lArr".to_string(), MacroDefinition::Str("\\Leftarrow".to_string())),
        ("\\Larr".to_string(), MacroDefinition::Str("\\Leftarrow".to_string())),
        ("\\lrarr".to_string(), MacroDefinition::Str("\\leftrightarrow".to_string())),
        ("\\lrArr".to_string(), MacroDefinition::Str("\\Leftrightarrow".to_string())),
        ("\\Lrarr".to_string(), MacroDefinition::Str("\\Leftrightarrow".to_string())),
        ("\\Mu".to_string(), MacroDefinition::Str("\\mathrm{M}".to_string())),
        ("\\natnums".to_string(), MacroDefinition::Str("\\mathbb{N}".to_string())),
        ("\\Nu".to_string(), MacroDefinition::Str("\\mathrm{N}".to_string())),
        ("\\Omicron".to_string(), MacroDefinition::Str("\\mathrm{O}".to_string())),
        ("\\plusmn".to_string(), MacroDefinition::Str("\\pm".to_string())),
        ("\\rarr".to_string(), MacroDefinition::Str("\\rightarrow".to_string())),
        ("\\rArr".to_string(), MacroDefinition::Str("\\Rightarrow".to_string())),
        ("\\Rarr".to_string(), MacroDefinition::Str("\\Rightarrow".to_string())),
        ("\\real".to_string(), MacroDefinition::Str("\\Re".to_string())),
        ("\\reals".to_string(), MacroDefinition::Str("\\mathbb{R}".to_string())),
        ("\\Reals".to_string(), MacroDefinition::Str("\\mathbb{R}".to_string())),
        ("\\Rho".to_string(), MacroDefinition::Str("\\mathrm{P}".to_string())),
        ("\\sdot".to_string(), MacroDefinition::Str("\\cdot".to_string())),
        ("\\sect".to_string(), MacroDefinition::Str("\\S".to_string())),
        ("\\spades".to_string(), MacroDefinition::Str("\\spadesuit".to_string())),
        ("\\sub".to_string(), MacroDefinition::Str("\\subset".to_string())),
        ("\\sube".to_string(), MacroDefinition::Str("\\subseteq".to_string())),
        ("\\supe".to_string(), MacroDefinition::Str("\\supseteq".to_string())),
        ("\\Tau".to_string(), MacroDefinition::Str("\\mathrm{T}".to_string())),
        ("\\thetasym".to_string(), MacroDefinition::Str("\\vartheta".to_string())),
        ("\\weierp".to_string(), MacroDefinition::Str("\\wp".to_string())),
        ("\\Zeta".to_string(), MacroDefinition::Str("\\mathrm{Z}".to_string())),
        //
        // //////////////////////////////////////////////////////////////////////
        // // statmath.sty
        // // https://ctan.math.illinois.edu/macros/latex/contrib/statmath/statmath.pdf
        //
        //         defineMacro("\\argmin", "\\DOTSB\\operatorname*{arg\\,min}");
        //         defineMacro("\\argmax", "\\DOTSB\\operatorname*{arg\\,max}");
        //         defineMacro("\\plim", "\\DOTSB\\mathop{\\operatorname{plim}}\\limits");
        (
            "\\dddot".to_string(),
            MacroDefinition::Str(
                "{\\overset{\\raisebox{-0.1ex}{\\normalsize ...}}{#1}}".to_string(),
            ),
        ),
        (
            "\\ddddot".to_string(),
            MacroDefinition::Str(
                "{\\overset{\\raisebox{-0.1ex}{\\normalsize ....}}{#1}}".to_string(),
            ),
        ),
        //
        // //////////////////////////////////////////////////////////////////////
        // // braket.sty
        // // http://ctan.math.washington.edu/tex-archive/macros/latex/contrib/braket/braket.pdf
        //
        //         defineMacro("\\bra", "\\mathinner{\\langle{#1}|}");
        //         defineMacro("\\ket", "\\mathinner{|{#1}\\rangle}");
        //         defineMacro("\\braket", "\\mathinner{\\langle{#1}\\rangle}");
        //         defineMacro("\\Bra", "\\left\\langle#1\\right|");
        //         defineMacro("\\Ket", "\\left|#1\\right\\rangle");
        //         const braketHelper = (one) => (context) => {
        //             const left = context.consumeArg().tokens;
        //             const middle = context.consumeArg().tokens;
        //             const middleDouble = context.consumeArg().tokens;
        //             const right = context.consumeArg().tokens;
        //             const oldMiddle = context.macros.get("|");
        //             const oldMiddleDouble = context.macros.get("\\|");
        //             context.macros.beginGroup();
        //             const midMacro = (double) => (context) => {
        //                 if (one) {
        //                     // Only modify the first instance of | or \|
        //                     context.macros.set("|", oldMiddle);
        //                     if (middleDouble.length) {
        //                         context.macros.set("\\|", oldMiddleDouble);
        //                     }
        //                 }
        //                 let doubled = double;
        //                 if (!double && middleDouble.length) {
        //                     // Mimic \@ifnextchar
        //                     const nextToken = context.future();
        //                     if (nextToken.text === "|") {
        //                         context.popToken();
        //                         doubled = true;
        //                     }
        //                 }
        //                 return {
        //                     tokens: doubled ? middleDouble : middle,
        //                     numArgs: 0,
        //                 };
        //             };
        //             context.macros.set("|", midMacro(false));
        //             if (middleDouble.length) {
        //                 context.macros.set("\\|", midMacro(true));
        //             }
        //             const arg = context.consumeArg().tokens;
        //             const expanded = context.expandTokens([
        //                 ...right, ...arg, ...left,  // reversed
        //             ]);
        //             context.macros.endGroup();
        //             return {
        //                 tokens: expanded.reverse(),
        //                 numArgs: 0,
        //             };
        //         };
        //         defineMacro("\\bra@ket", braketHelper(false));
        //         defineMacro("\\bra@set", braketHelper(true));
        //         defineMacro("\\Braket", "\\bra@ket{\\left\\langle}" +
        //             "{\\,\\middle\\vert\\,}{\\,\\middle\\vert\\,}{\\right\\rangle}");
        //         defineMacro("\\Set", "\\bra@set{\\left\\{\\:}" +
        //             "{\\;\\middle\\vert\\;}{\\;\\middle\\Vert\\;}{\\:\\right\\}}");
        ("\\Set".to_string(), MacroDefinition::MacroContext(set_macro)),
        ("\\set".to_string(), MacroDefinition::MacroContext(set_small_macro)),
        //         defineMacro("\\set", "\\bra@set{\\{\\,}{\\mid}{}{\\,\\}}");
        //         // has no support for special || or \|
        //
        // //////////////////////////////////////////////////////////////////////
        // // actuarialangle.dtx
        //         defineMacro("\\angln", "{\\angl n}");
        //
        // // Custom Khan Academy colors, should be moved to an optional package
        //         defineMacro("\\blue", "\\textcolor{##6495ed}{#1}");
        //         defineMacro("\\orange", "\\textcolor{##ffa500}{#1}");
        //         defineMacro("\\pink", "\\textcolor{##ff00af}{#1}");
        //         defineMacro("\\red", "\\textcolor{##df0030}{#1}");
        //         defineMacro("\\green", "\\textcolor{##28ae7b}{#1}");
        //         defineMacro("\\gray", "\\textcolor{gray}{#1}");
        //         defineMacro("\\purple", "\\textcolor{##9d38bd}{#1}");
        //         defineMacro("\\blueA", "\\textcolor{##ccfaff}{#1}");
        //         defineMacro("\\blueB", "\\textcolor{##80f6ff}{#1}");
        //         defineMacro("\\blueC", "\\textcolor{##63d9ea}{#1}");
        //         defineMacro("\\blueD", "\\textcolor{##11accd}{#1}");
        //         defineMacro("\\blueE", "\\textcolor{##0c7f99}{#1}");
        //         defineMacro("\\tealA", "\\textcolor{##94fff5}{#1}");
        //         defineMacro("\\tealB", "\\textcolor{##26edd5}{#1}");
        //         defineMacro("\\tealC", "\\textcolor{##01d1c1}{#1}");
        //         defineMacro("\\tealD", "\\textcolor{##01a995}{#1}");
        //         defineMacro("\\tealE", "\\textcolor{##208170}{#1}");
        //         defineMacro("\\greenA", "\\textcolor{##b6ffb0}{#1}");
        //         defineMacro("\\greenB", "\\textcolor{##8af281}{#1}");
        //         defineMacro("\\greenC", "\\textcolor{##74cf70}{#1}");
        //         defineMacro("\\greenD", "\\textcolor{##1fab54}{#1}");
        //         defineMacro("\\greenE", "\\textcolor{##0d923f}{#1}");
        //         defineMacro("\\goldA", "\\textcolor{##ffd0a9}{#1}");
        //         defineMacro("\\goldB", "\\textcolor{##ffbb71}{#1}");
        //         defineMacro("\\goldC", "\\textcolor{##ff9c39}{#1}");
        //         defineMacro("\\goldD", "\\textcolor{##e07d10}{#1}");
        //         defineMacro("\\goldE", "\\textcolor{##a75a05}{#1}");
        //         defineMacro("\\redA", "\\textcolor{##fca9a9}{#1}");
        //         defineMacro("\\redB", "\\textcolor{##ff8482}{#1}");
        //         defineMacro("\\redC", "\\textcolor{##f9685d}{#1}");
        //         defineMacro("\\redD", "\\textcolor{##e84d39}{#1}");
        //         defineMacro("\\redE", "\\textcolor{##bc2612}{#1}");
        //         defineMacro("\\maroonA", "\\textcolor{##ffbde0}{#1}");
        //         defineMacro("\\maroonB", "\\textcolor{##ff92c6}{#1}");
        //         defineMacro("\\maroonC", "\\textcolor{##ed5fa6}{#1}");
        //         defineMacro("\\maroonD", "\\textcolor{##ca337c}{#1}");
        //         defineMacro("\\maroonE", "\\textcolor{##9e034e}{#1}");
        //         defineMacro("\\purpleA", "\\textcolor{##ddd7ff}{#1}");
        //         defineMacro("\\purpleB", "\\textcolor{##c6b9fc}{#1}");
        //         defineMacro("\\purpleC", "\\textcolor{##aa87ff}{#1}");
        //         defineMacro("\\purpleD", "\\textcolor{##7854ab}{#1}");
        //         defineMacro("\\purpleE", "\\textcolor{##543b78}{#1}");
        //         defineMacro("\\mintA", "\\textcolor{##f5f9e8}{#1}");
        //         defineMacro("\\mintB", "\\textcolor{##edf2df}{#1}");
        //         defineMacro("\\mintC", "\\textcolor{##e0e5cc}{#1}");
        //         defineMacro("\\grayA", "\\textcolor{##f6f7f7}{#1}");
        //         defineMacro("\\grayB", "\\textcolor{##f0f1f2}{#1}");
        //         defineMacro("\\grayC", "\\textcolor{##e3e5e6}{#1}");
        //         defineMacro("\\grayD", "\\textcolor{##d6d8da}{#1}");
        //         defineMacro("\\grayE", "\\textcolor{##babec2}{#1}");
        //         defineMacro("\\grayF", "\\textcolor{##888d93}{#1}");
        //         defineMacro("\\grayG", "\\textcolor{##626569}{#1}");
        //         defineMacro("\\grayH", "\\textcolor{##3b3e40}{#1}");
        //         defineMacro("\\grayI", "\\textcolor{##21242c}{#1}");
        //         defineMacro("\\kaBlue", "\\textcolor{##314453}{#1}");
        //         defineMacro("\\kaGreen", "\\textcolor{##71B307}{#1}");
    ]);

    res
}
