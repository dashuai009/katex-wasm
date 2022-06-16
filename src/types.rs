/**
 * This file consists only of basic flow types used in multiple places.
 * For types with javascript, create separate files by themselves.
 */
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    math,
    text,
}
impl FromStr for Mode {
    type Err = ();

    fn from_str(input: &str) -> Result<Mode, Self::Err> {
        match input {
            "math" => Ok(Mode::math),
            "text" => Ok(Mode::text),
            _ => Err(()),
        }
    }
}
impl Mode {
    fn as_str(&self) -> &'static str {
        match self {
            Mode::math => "math",
            Mode::text => "text",
        }
    }
}
// LaTeX argument type.
//   - "size": A size-like thing, such as "1em" or "5ex"
//   - "color": An html color, like "#abc" or "blue"
//   - "url": An url string, in which "\" will be ignored
//   -        if it precedes [#$%&~_^\{}]
//   - "raw": A string, allowing single character, percent sign,
//            and nested braces
//   - "original": The same type as the environment that the
//                 function being parsed is in (e.g. used for the
//                 bodies of functions like \textcolor where the
//                 first argument is special and the second
//                 argument is parsed normally)
//   - Mode: Node group parsed in given mode.
pub enum ArgType {
    color,
    size,
    url,
    raw,
    original,
    hbox,
    primitive,
    math, // | Mode
    text,
}

// LaTeX display style.
pub enum StyleStr {
    text,
    display,
    script,
    scriptscript,
}

// Allowable token text for "break" arguments in parser.
pub enum BreakToken {
    rightBracket,     // "]"
    rightBrace,       // "}"
    endgroup,         // "\\endgroup"
    dollar,           // "$"
    rightParentheses, // "\\)"
    doubleSlash,      // "\\\\"
    end,              // "\\end"
    EOF,              // "EOF"
}

// Math font variants.
pub enum FontVariant {
    bold,
    bold_italic,     // "bold-italic"
    bold_sans_serif, //
    double_struck,   //
    fraktur,         //
    italic,
    monospace,
    normal,
    sans_serif,
    sans_serif_bold_italic,
    sans_serif_italic,
    script,
}
