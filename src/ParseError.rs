use crate::token::Token;

use crate::sourceLocation::SourceLocation;

/**
 * This is the ParseError class, which is the main error thrown by KaTeX
 * functions when something has gone wrong. This is used to distinguish internal
 * errors from errors in the expression that the user provided.
 *
 * If possible, a caller should provide a Token or ParseNode with information
 * about where in the source string the problem occurred.
 */
// struct ParseError {
//     position: f64,
// }

pub fn parse_error(message: String, loc: SourceLocation) -> js_sys::Error {
    let mut error: String = format!("KaTeX parse error: {}", message);
    let mut start: usize = 0;
    if loc.start <= loc.end {
        // If we have the input and a position, make the error a bit fancier

        // Get the input
        let input = loc.lexer.input;

        // Prepend some information
        start = loc.start as usize;
        let end: usize = loc.end as usize;
        if start == input.len() as usize {
            error += " at end of input: ";
        } else {
            error.push_str(&format!(" at position {}: ", start + 1));
        }

        // Underline token in question using combining underscores
        let mut underlined = (&input[start..end]).clone();
        regex::Regex::new("[^]")
            .unwrap()
            .replace_all(underlined, "$&\u{0332}");

        // Extract some context from the input and add it to the error
        let left;
        if start > 15 {
            left = format!("… {}", &input[start - 15..start]);
        } else {
            left = String::from(&input[0..start]);
        }
        let right;
        if end + 15 < input.len() {
            right = format!("{} ...", &input[end..end + 15]);
        } else {
            right = String::from(&input[end..]);
        }
        error.push_str(&format!("{}{}{}", left, underlined, right));
    }
    let res = js_sys::Error::new(&error);
    res.set_name("ParseError");
    return res;
}
