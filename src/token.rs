use unicode_normalization::IsNormalized::No;
use wasm_bindgen::prelude::*;

/**
 * The resulting token returned from `lex`.
 *
 * It consists of the token text plus some position information.
 * The position information is essentially a range in an input string,
 * but instead of referencing the bare input string, we refer to the lexer.
 * That way it is possible to attach extra metadata to the input string,
 * like for example a file name or similar.
 *
 * The position information is optional, so it is OK to construct synthetic
 * tokens if appropriate. Not providing available position information may
 * lead to degraded error reporting, though.
 */
use super::sourceLocation::SourceLocation;

#[wasm_bindgen(getter_with_clone)]
pub struct Token {
    pub text: String,
    pub loc: Option<SourceLocation>,
    pub noexpand: Option<bool>,     // don't expand the token
    pub treatAsRelax: Option<bool>, // used in \noexpand
}

#[wasm_bindgen]
impl Token {

    #[wasm_bindgen(constructor)]
    pub fn new(
        text: String,           // the text of this token
        loc: Option<SourceLocation>,
    )->Token {
        Token{
            text,
            loc,
            noexpand:None,
            treatAsRelax:None
        }
    }
    /**
     * Given a pair of tokens (this and endToken), compute a `Token` encompassing
     * the whole input range enclosed by these two.
     */
    pub fn range(
        &self,
        endToken: &Token, // last token of the range, inclusive
        text: String,    // the text of the newly constructed token
    ) -> Token {
        if self.loc.is_some() && endToken.loc.is_some() {
            return Token {
                text,
                loc: Some(SourceLocation::range(self.loc.as_ref().unwrap(), endToken.loc.as_ref().unwrap())),
                noexpand: None,
                treatAsRelax: None,
            };
        } else {
            return Token {
                text,
                loc: self.loc.clone(),
                noexpand: None,
                treatAsRelax: None,
            };
        }
    }
}
