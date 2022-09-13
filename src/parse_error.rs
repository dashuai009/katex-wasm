use std::fmt::{self, Error};

/**
 * This is the ParseError class, which is the main error thrown by KaTeX
 * functions when something has gone wrong. This is used to distinguish internal
 * errors from errors in the expression that the user provided.
 *
 * If possible, a caller should provide a Token or ParseNode with information
 * about where in the source string the problem occurred.
 */
#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
    pub loc: Option<crate::sourceLocation::SourceLocation>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut error: String = format!("KaTeX parse error: {}", self.msg);
        let mut start: usize = 0;
        if let Some(loc) = &self.loc {
            if loc.start <= loc.end {
                // If we have the input and a position, make the error a bit fancier

                // Get the input
                let input = loc.lexer.get_input();

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
                let right = if end + 15 < input.len() {
                    format!("{} ...", &input[end..end + 15])
                } else {
                    String::from(&input[end..])
                };
                error.push_str(&format!("{}{}{}", left, underlined, right));
            }
        } else {
            error.push_str("ha ha ha loc is none");
        }
        return write!(f, "{}", error);
    }
}

// impl std::error::Error for ParseError {
//     fn description(&self) -> &str {
//     }

//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         None
//     }

//     fn type_id(&self, _: private::Internal) -> std::any::TypeId
//     where
//         Self: 'static,
//     {
//         std::any::TypeId::of::<Self>()
//     }

//     fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
//         None
//     }

//     fn cause(&self) -> Option<&dyn std::error::Error> {
//         self.source()
//     }
// }
