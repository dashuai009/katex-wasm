use crate::utils::{console_log, log};
use regex::Regex;
use wasm_bindgen::prelude::*;

/**
 * Interface required to break circular dependency between Token, Lexer, and
 * ParseError.
 */
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct LexerInterface {
    input: String,
    // pub tokenRegx: regex::Regex,
    token_regex: Regex,
    last_index: usize,
}

impl std::fmt::Display for LexerInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f , "input:{} last_index:{}" , self.input, self.last_index)
    }
}

impl LexerInterface {
    pub fn new(input: String, token_regex: Regex) -> LexerInterface {
        // console_log!("input {}",input);
        return LexerInterface {
            input,
            token_regex,
            last_index: 0,
        };
    }

    pub fn get_input(&self)->&String{
        &self.input
    }

    pub fn get_last_index(&self) -> usize {
        self.last_index
    }
    pub fn set_last_index(&mut self, index: usize) {
        self.last_index = index;
    }

    pub fn captures(&mut self)->std::option::Option<regex::Captures<'_>>{
        if self.last_index >= self.input.len(){
            return None;
        }
        if let Some(res) = self.token_regex.captures(&self.input[self.last_index..]){
            self.last_index = res.get(0).unwrap().end() + self.last_index;
            return Some(res);
        }else{
            self.last_index = self.input.len();
            return None;
        }
    }
}
//impl Copy for LexerInterface {}

/**
 * Lexing or parsing positional information for error reporting.
 * This object is immutable.
 */
#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct SourceLocation {
    // The + prefix indicates that these fields aren't writeable
    pub lexer: LexerInterface,
    // Lexer holding the input string.
    pub start: i32,
    // Start offset, zero-based inclusive.
    pub end: i32, // End offset, zero-based exclusive.
}

#[wasm_bindgen]
impl SourceLocation {
    #[wasm_bindgen(constructor)]
    pub fn new(lexer: &LexerInterface, start: f64, end: f32) -> SourceLocation {
        SourceLocation {
            lexer: lexer.clone(),
            start: start as i32,
            end: end as i32,
        }
    }
    /**
     * Merges two `SourceLocation`s from location providers, given they are
     * provided in order of appearance.
     * - Returns the first one's location if only the first is provided.
     * - Returns a merged range of the first and the last if both are provided
     *   and their lexers match.
     * - Otherwise, returns null.
     */
    pub fn range(first: &SourceLocation, second: &SourceLocation) -> SourceLocation {
        return SourceLocation {
            lexer: first.lexer.clone(),
            start: first.start,
            end: second.end,
        };
    }
}
