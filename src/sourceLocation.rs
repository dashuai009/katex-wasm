use wasm_bindgen::prelude::*;

use super::token::LexerInterface;

/**
 * Lexing or parsing positional information for error reporting.
 * This object is immutable.
 */
#[wasm_bindgen]
pub struct SourceLocation {
    // The + prefix indicates that these fields aren't writeable
    lexer: LexerInterface,
    // Lexer holding the input string.
    start: f64,
    // Start offset, zero-based inclusive.
    end: f64,              // End offset, zero-based exclusive.
}

// #[wasm_bindgen]
// impl SourceLocation {
    // #[wasm_bindgen(constructor)]
    // pub fn new(_lexer: LexerInterface, _start: f64, _end: f64) -> SourceLocation {
    //     SourceLocation {
    //         lexer: _lexer,
    //         start: _start,
    //         end: _end,
    //     }
    // }

    // /**
    //  * Merges two `SourceLocation`s from location providers, given they are
    //  * provided in order of appearance.
    //  * - Returns the first one's location if only the first is provided.
    //  * - Returns a merged range of the first and the last if both are provided
    //  *   and their lexers match.
    //  * - Otherwise, returns null.
    //  */
    // pub fn range(first: &JsValue, second: &JsValue) -> JsValue {
    //     if !second {
    //         if !JsValue::is_null(first) {
    //             return js_sys::Reflect::get(first, &JsValue::from_str("loc"))
    //                 .expect("yrkisfaksdj  ");
    //         } else {
    //             return JsValue::NULL;
    //         }
    //     } else if (!first || !first.loc || !second.loc || first.loc.lexer != second.loc.lexer) {
    //         return None;
    //     } else {
    //         return SourceLocation::new(first.loc.lexer, first.loc.start, second.loc.end);
    //     }
    // }
// }
    
