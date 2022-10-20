use crate::StyleInterface;
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
    fn as_arg_type(&self) -> ArgType {
        match self {
            Mode::math => ArgType::math,
            Mode::text => ArgType::text,
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Debug)]
pub enum StyleStr {
    text,
    display,
    script,
    scriptscript,
}

impl StyleStr {
    pub fn as_style(&self) -> StyleInterface {
        match self {
            StyleStr::text => {
                let res = crate::Style::TEXT.lock().unwrap();
                res.clone()
            }
            StyleStr::display => {
                let res = crate::Style::DISPLAY.lock().unwrap();
                res.clone()
            }
            StyleStr::script => {
                let res = crate::Style::SCRIPT.lock().unwrap();
                res.clone()
            }
            StyleStr::scriptscript => {
                let res = crate::Style::SCRIPTSCRIPT.lock().unwrap();
                res.clone()
            }
        }
    }
}

// Allowable token text for "break" arguments in parser.
#[derive(Clone, PartialEq)]
pub enum BreakToken {
    RightBracket,     // "]"
    RightBrace,       // "}"
    Endgroup,         // "\\endgroup"
    Dollar,           // "$"
    RightParentheses, // "\\)"
    DoubleSlash,      // "\\\\"
    End,              // "\\end"
    Eof,              // "EOF"
}
impl FromStr for BreakToken {
    type Err = ();

    fn from_str(input: &str) -> Result<BreakToken, Self::Err> {
        match input {
            "]" => Ok(BreakToken::RightBracket),
            "}" => Ok(BreakToken::RightBrace),
            "\\endgroup" => Ok(BreakToken::Endgroup),
            "$" => Ok(BreakToken::Dollar),
            "\\)" => Ok(BreakToken::RightParentheses),
            "\\\\" => Ok(BreakToken::DoubleSlash),
            "\\end" => Ok(BreakToken::End),
            "EOF" => Ok(BreakToken::Eof),
            _ => Err(()),
        }
    }
}
impl BreakToken {
    pub fn as_str(&self) -> &'static str {
        match self {
            BreakToken::RightBracket => "]",
            BreakToken::RightBrace => "}",
            BreakToken::Endgroup => "\\endgroup",
            BreakToken::Dollar => "$",
            BreakToken::RightParentheses => "\\)",
            BreakToken::DoubleSlash => "\\\\",
            BreakToken::End => "\\end",
            BreakToken::Eof => "EOF",
        }
    }
}

// Math font variants.
#[derive(Copy, Clone, PartialEq)]
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

impl FontVariant {
    pub fn as_str(&self) -> &str {
        match self {
            FontVariant::bold => "bold",
            FontVariant::bold_italic => "bold-italic",
            FontVariant::bold_sans_serif => "bold-sans-serif",
            FontVariant::double_struck => "double-struck",
            FontVariant::fraktur => "fraktur",
            FontVariant::italic => "italic",
            FontVariant::monospace => "monospace",
            FontVariant::normal => "normal",
            FontVariant::sans_serif => "sans-serif",
            FontVariant::sans_serif_bold_italic => "sans-serif-bold-italic",
            FontVariant::sans_serif_italic => "sans-serif-italic",
            FontVariant::script => "script",
        }
    }
}
