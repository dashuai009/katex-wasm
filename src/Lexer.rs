use crate::parse_error::ParseError;
use crate::settings::Settings;
use crate::utils::{console_log, log};
use crate::{LexerInterface, SourceLocation, Token};
use regex::Regex;
/**
 * The Lexer class handles tokenizing the input in various ways. Since our
 * parser expects us to be able to backtrack, the lexer allows lexing from any
 * given starting point.
 *
 * Its main exposed function is the `lex` function, which takes a position to
 * lex from and a type of token to lex. It defers to the appropriate `_innerLex`
 * function.
 *
 * The various `_innerLex` functions perform the actual lexing of different
 * kinds.
 */
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
/* The following tokenRegex
 * - matches typical whitespace (but not NBSP etc.) using its first group
 * - does not match any control character \x00-\x1f except whitespace
 * - does not match a bare backslash
 * - matches any ASCII character except those just mentioned
 * - does not match the BMP private use area \uE000-\uF8FF
 * - does not match bare surrogate code units
 * - matches any BMP character except for those just described
 * - matches any valid Unicode surrogate pair
 * - matches a backslash followed by one or more whitespace characters
 * - matches a backslash followed by one or more letters then whitespace
 * - matches a backslash followed by any BMP character
 * Capturing groups:
 *   [1] regular whitespace
 *   [2] backslash followed by whitespace
 *   [3] anything else, which may include:
 *     [4] left character of \verb*
 *     [5] left character of \verb
 *     [6] backslash followed by word, excluding any trailing whitespace
 * Just because the Lexer matches something doesn't mean it's valid input:
 * If there is no matching function or symbol definition, the Parser will
 * still reject the input.
 */

const spaceRegexString: &'static str = "[ \r\n\t]";
const controlWordRegexString: &'static str = "\\\\[a-zA-Z@]+";
const controlWordWhitespaceRegexString: &'static str = "(\\\\[a-zA-Z@]+)[ \r\n\t]*";
const controlSpaceRegexString: &'static str = "\\\\(\n|[ \r\t]+\n?)[ \r\t]*";
const combiningDiacriticalMarkString: &'static str = "[\u{0300}-\u{036f}]";
lazy_static! {
// static ref controlSymbolRegexString: &str = "\\\\[^\u{d800}-\u{DFFF}]";
// static ref controlWordWhitespaceRegexString:String = format!("(${controlWordRegexString})${spaceRegexString}*");

// const token_regex_string :&'static str = "([ \r\n\t]+)|\\\\(\n|[ \r\t]+\n?)[ \r\t]*|([!-\\[\\]-\u2027\u202A-\uD7FF\uF900-\uFFFF][\u0300-\u036f]*[\uD800-\uDBFF][\uDC00-\uDFFF][\u0300-\u036f]*|\\\\verb\\*([^]).*?\\4|\\\\verb([^*a-zA-Z]).*?\\5|(\\\\[a-zA-Z@]+)[ \r\n\t]*|\\\\[^\uD800-\uDFFF]";
// static ref  combiningDiacriticalMarksEndRegex: RegExp =    new RegExp(`${combiningDiacriticalMarkString}+$`);
static ref TOKEN_REGEX: Regex = {
        let mut res = format!("({spaceRegexString}+)|");  // whitespace
        res.push_str(format!("{controlSpaceRegexString}|").as_str()) ;                  // \whitespace
        res.push_str("([!-\\[\\]-\u{2027}\u{202A}-\u{D7FF}\u{F900}-\u{FFFF}]");  // single codepoint
        res.push_str(format!("{combiningDiacriticalMarkString}*").as_str())    ;        // ...plus accents
        // .push_str("|[\u{D800}-\u{DBFF}][\u{DC00}-\uDFFF]")               // surrogate pair
        // res.push_str(format!("{combiningDiacriticalMarkString}*").as_str())  ;          // ...plus accents
        res.push_str("|\\\\verb\\*([^.]).*?\u{4}")        ;               // \verb*
        res.push_str("|\\\\verb([^*a-zA-Z]).*?\u{5}")    ;               // \verb unstarred
        res.push_str(format!("|{controlWordWhitespaceRegexString}").as_str())   ;      // \macroName + spaces
        res.push_str("|\\\\.)");// res.push_str(format!("|{controlSymbolRegexString})"));                  // \\, \', etc.
        Regex::new(&res).expect("Failed to compile TOKEN_REGEX")
        };
}

/** Main Lexer class */
#[wasm_bindgen]
pub struct Lexer {
    lexer_i: LexerInterface,
    settings: Settings,
    //     Category codes. The lexer only supports comment characters (14) for now.
    // MacroExpander additionally distinguishes active (13).
    catcodes: HashMap<String, i32>,
}

//
// get input(): string {
// return this.lexerI.input;
// }
// set input(s: string) {
// this.lexerI.input = s;
// }
//
// get tokenRegex(): RegExp {
// return this.lexerI.tokenRegex;
// }
//
// set tokenRegex(t: RegExp) {
// this.lexerI.tokenRegex = t;
// }
#[wasm_bindgen]
impl Lexer {
    #[wasm_bindgen(constructor)]
    pub fn new(input: String, settings: &Settings) -> Lexer {
        // Separate accents from characters
        Lexer {
            lexer_i: LexerInterface::new(
                input,
                Regex::new(&TOKEN_REGEX.to_string()).unwrap(),
            ),
            settings: settings.clone(),
            catcodes: HashMap::from([
                ("%".to_string(), 14), // comment character
                ("~".to_string(), 13), // active character
            ]),
        }
    }

    #[wasm_bindgen(js_name = setCatcode)]
    pub fn set_catcode(&mut self, char: String, code: i32) {
        self.catcodes.insert(char, code);
    }

    /**
     * This function lexes a single token.
     */
    pub fn lex(&mut self) -> Token {
        use web_sys::console;
        // let input = self.lexer_i.get_input();
        let pos = { self.lexer_i.get_last_index().clone() };
        // println!("pos = {}; input = {};", pos, input);
        if pos == self.lexer_i.get_input().len() {
            return Token {
                text: "EOF".to_string(),
                loc: Some(SourceLocation::new(&self.lexer_i, pos as f64, pos as f32)),
                noexpand: false,
                treatAsRelax: false,
            };
        }

        let _match = self.lexer_i.captures();
        if _match.is_none()
        /*|| _match.unwrap() != pos*/
        {
            panic!(
                "Unexpected character: ,new Token(input[pos], new SourceLocation(this, pos, pos + 1)"
            );
        }
        let _match_u = _match.unwrap();
        let text: String = if let Some(m6) = _match_u.get(6) {
            m6.as_str()
        } else if let Some(m3) = _match_u.get(3) {
            m3.as_str()
        } else if let Some(m2) = _match_u.get(2) {
            "\\ "
        } else {
            " "
        }
        .clone()
        .to_string();
        // console_log!("text = {}", text);

        if self.catcodes.get(&text) == Some(&14) {
            // comment character
            let nl_index = self.lexer_i.get_input()[self.lexer_i.get_last_index()..].find('\n');
            if nl_index.is_none() {
                self.lexer_i.set_last_index(self.lexer_i.get_input().len()); // EOF
                self.settings.report_nonstrict(
                    "commentAtEnd" ,
                    "% comment has no terminating newline; LaTeX would fail because of commenting the end of math mode (e.g. $)",
                    None,
                );
            } else {
                self.lexer_i.set_last_index(nl_index.unwrap() + 1);
            }
            return self.lex();
        }

        return Token {
            text: text,
            loc: Some(SourceLocation {
                lexer: self.lexer_i.clone(),
                start: pos as i32,
                end: self.lexer_i.get_last_index() as i32,
            }),
            noexpand: false,

            treatAsRelax: false,
        };
    }
}

impl Lexer {
    pub fn catcodes_get(&self, name: &String) -> Option<&i32> {
        return self.catcodes.get(name);
    }
}