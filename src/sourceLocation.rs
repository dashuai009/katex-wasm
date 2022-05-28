use wasm_bindgen::prelude::*;
use crate::utils::{console_log,log};

/**
 * Interface required to break circular dependency between Token, Lexer, and
 * ParseError.
 */
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug,Clone)]
pub struct LexerInterface {
     pub input: String,
    // pub tokenRegx: regex::Regex,
     pub tokenRegex: js_sys::RegExp
}

#[wasm_bindgen]
impl LexerInterface{
    #[wasm_bindgen(constructor)]
    pub fn new(input: String,tokenRegex: js_sys::RegExp) -> LexerInterface {
        // console_log!("input {}",input);
        return LexerInterface { input:input,tokenRegex: tokenRegex};
    }

    // #[wasm_bindgen(setter)]
    // pub fn set_input(&mut self,input:String){
    //     self.input = input;
    // }
    //
    //
    // #[wasm_bindgen(setter)]
    // pub fn set_tokenRegex(&mut self,tokenRegex:js_sys::RegExp){
    //     self.tokenRegex = tokenRegex;
    // }
}
//impl Copy for LexerInterface {}

/**
 * Lexing or parsing positional information for error reporting.
 * This object is immutable.
 */
#[derive(Debug,Clone)]
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
    pub fn new(lexer: &LexerInterface, start: f64, end: f32)->SourceLocation{
        SourceLocation{
             lexer:lexer.clone(),
             start: start as i32,
             end: end as i32
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
    pub fn range(first:&SourceLocation , second: &SourceLocation) -> SourceLocation {
        return SourceLocation{
            lexer:first.lexer.clone(),
            start:first.start,
            end:second.end
        };
    }
}
