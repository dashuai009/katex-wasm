//! This file provides support for Unicode range U+1D400 to U+1D7FF,
//! Mathematical Alphanumeric Symbols.
//! Function wideCharacterFont takes a wide character as input and returns
//! the font information necessary to render it properly.

use crate::types::Mode;
use std::str::FromStr;

///
/// Data below is from https://www.unicode.org/charts/PDF/U1D400.pdf
///    That document sorts characters into groups by font type, say bold or italic.
///
/// In the arrays below, each subarray consists three elements:
///    - The CSS class of that group when in math mode.
///    - The CSS class of that group when in text mode.
///    - The font name, so that KaTeX can get font metrics.
///

const WIDE_LATIN_LETTER_DATA: [[&'static str; 3]; 26] = [
    ["mathbf", "textbf", "Main-Bold"],               // A-Z bold upright
    ["mathbf", "textbf", "Main-Bold"],               // a-z bold upright
    ["mathnormal", "textit", "Math-Italic"],         // A-Z italic
    ["mathnormal", "textit", "Math-Italic"],         // a-z italic
    ["boldsymbol", "boldsymbol", "Main-BoldItalic"], // A-Z bold italic
    ["boldsymbol", "boldsymbol", "Main-BoldItalic"], // a-z bold italic
    // Map fancy A-Z letters to script, not calligraphic.
    // This aligns with unicode-math and math fonts (except Cambria Math).
    ["mathscr", "textscr", "Script-Regular"], // A-Z script
    ["", "", ""],                             // a-z script.  No font
    ["", "", ""],                             // A-Z bold script. No font
    ["", "", ""],                             // a-z bold script. No font
    ["mathfrak", "textfrak", "Fraktur-Regular"], // A-Z Fraktur
    ["mathfrak", "textfrak", "Fraktur-Regular"], // a-z Fraktur
    ["mathbb", "textbb", "AMS-Regular"],      // A-Z double-struck
    ["mathbb", "textbb", "AMS-Regular"],      // k double-struck
    ["", "", ""],                             // A-Z bold Fraktur No font metrics
    ["", "", ""],                             // a-z bold Fraktur.   No font.
    ["mathsf", "textsf", "SansSerif-Regular"], // A-Z sans-serif
    ["mathsf", "textsf", "SansSerif-Regular"], // a-z sans-serif
    ["mathboldsf", "textboldsf", "SansSerif-Bold"], // A-Z bold sans-serif
    ["mathboldsf", "textboldsf", "SansSerif-Bold"], // a-z bold sans-serif
    ["mathitsf", "textitsf", "SansSerif-Italic"], // A-Z italic sans-serif
    ["mathitsf", "textitsf", "SansSerif-Italic"], // a-z italic sans-serif
    ["", "", ""],                             // A-Z bold italic sans. No font
    ["", "", ""],                             // a-z bold italic sans. No font
    ["mathtt", "texttt", "Typewriter-Regular"], // A-Z monospace
    ["mathtt", "texttt", "Typewriter-Regular"], // a-z monospace
];

const WIDE_NUMERAL_DATA: [[&'static str; 3]; 5] = [
    ["mathbf", "textbf", "Main-Bold"],              // 0-9 bold
    ["", "", ""],                                   // 0-9 double-struck. No KaTeX font.
    ["mathsf", "textsf", "SansSerif-Regular"],      // 0-9 sans-serif
    ["mathboldsf", "textboldsf", "SansSerif-Bold"], // 0-9 bold sans-serif
    ["mathtt", "texttt", "Typewriter-Regular"],     // 0-9 monospace
];

pub fn wide_character_font(wide_char: &String, mode: Mode) -> Result<[&'static str; 2], String> {
    // IE doesn't support codePointAt(). So work with the surrogate pair.
    // let H = wideChar[0];    // high surrogate
    // let L = wideChar.charCodeAt(1);    // low surrogate
    // let codePoint = ((H - 0xD800) * 0x400) + (L - 0xDC00) + 0x10000;
    let code_point = wide_char.chars().nth(0).unwrap() as usize;

    let j = if mode == Mode::math { 0 } else { 1 }; // column index for CSS class.

    return if 0x1D400 <= code_point && code_point < 0x1D6A4 {
        // WIDE_LATIN_LETTER_DATA contains exactly 26 chars on each row.
        // So we can calculate the relevant row. No traverse necessary.
        let i = (code_point - 0x1D400) / 26;
        Ok([WIDE_LATIN_LETTER_DATA[i][2], WIDE_LATIN_LETTER_DATA[i][j]])
    } else if 0x1D7CE <= code_point && code_point <= 0x1D7FF {
        // Numerals, ten per row.
        let i = (code_point - 0x1D7CE) / 10;
        Ok([WIDE_NUMERAL_DATA[i][2], WIDE_NUMERAL_DATA[i][j]])
    } else if code_point == 0x1D6A5 || code_point == 0x1D6A6 {
        // dotless i or j
        Ok([WIDE_LATIN_LETTER_DATA[0][2], WIDE_LATIN_LETTER_DATA[0][j]])
    } else if 0x1D6A6 < code_point && code_point < 0x1D7CE {
        // Greek letters. Not supported, yet.
        Ok(["", ""])
    } else {
        // We don't support any wide characters outside 1D400–1D7FF.
        let err = format!("{}{}", "Unsupported character", wide_char);
        Err(err)
        //throw new ParseError();
    };
}
