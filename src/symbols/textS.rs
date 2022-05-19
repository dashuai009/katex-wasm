use std::collections::HashMap;

use crate::symbols::public::*;

pub fn define_all_text_symbols() -> HashMap<String, Symbol> {
    let mut text = HashMap::new();
    use Font::*;
    use Group::*;

    defineSymbolM!(text, main, textord, "\u{0023}", "\\#");

    defineSymbolM!(text, main, textord, "\u{0026}", "\\&");

    defineSymbolM!(text, main, textord, "\u{00a7}", "\\S");

    defineSymbolM!(text, main, textord, "\u{00b6}", "\\P");

    // Math and Text

    defineSymbolM!(text, main, textord, "\u{2020}", "\\dag");
    defineSymbolM!(text, main, textord, "\u{2020}", "\\textdagger");

    defineSymbolM!(text, main, textord, "\u{2021}", "\\ddag");
    defineSymbolM!(text, main, textord, "\u{2021}", "\\textdaggerdbl");

    defineSymbolM!(text, ams, textord, "\u{00ae}", "\\circledR");

    defineSymbolM!(text, main, textord, "\u{00f0}", "\u{00f0}");

    defineSymbolM!(text, ams, textord, "\u{00a5}", "\\yen", true);

    defineSymbolM!(text, ams, textord, "\u{2713}", "\\checkmark");

    defineSymbolM!(text, main, textord, "$", "\\$");
    defineSymbolM!(text, main, textord, "$", "\\textdollar");

    defineSymbolM!(text, main, textord, "%", "\\%");

    defineSymbolM!(text, main, textord, "_", "\\_");
    defineSymbolM!(text, main, textord, "_", "\\textunderscore");

    defineSymbolM!(text, main, spacing, "\u{00a0}", "\\ ");
    defineSymbolM!(text, main, spacing, "\u{00a0}", " ");
    defineSymbolM!(text, main, spacing, "\u{00a0}", "\\space");
    defineSymbolM!(text, main, spacing, "\u{00a0}", "\\nobreakspace");

    defineSymbolM!(text, main, textord, "{", "\\{");
    defineSymbolM!(text, main, textord, "{", "\\textbraceleft");

    defineSymbolM!(text, main, textord, "}", "\\}");
    defineSymbolM!(text, main, textord, "}", "\\textbraceright");

    defineSymbolM!(text, main, textord, "[", "\\lbrack", true);

    defineSymbolM!(text, main, textord, "]", "\\rbrack", true);

    defineSymbolM!(text, main, textord, "<", "\\textless", true); // in T1 fontenc
    defineSymbolM!(text, main, textord, ">", "\\textgreater", true); // in T1 fontenc

    defineSymbolM!(text, main, textord, "|", "\\textbar", true); // in T1 fontenc

    defineSymbolM!(text, main, textord, "\u{2225}", "\\textbardbl");
    defineSymbolM!(text, main, textord, "~", "\\textasciitilde");
    defineSymbolM!(text, main, textord, "\\", "\\textbackslash");
    defineSymbolM!(text, main, textord, "^", "\\textasciicircum");

    defineSymbolM!(text, main, inner, "\u{2026}", "\\textellipsis");

    defineSymbolM!(text, main, inner, "\u{2026}", "\\ldots}", true);

    defineSymbolM!(text, main, textord, "\u{0131}", "\\i", true);
    defineSymbolM!(text, main, textord, "\u{0237}", "\\j", true);
    defineSymbolM!(text, main, textord, "\u{00df}", "\\ss", true);
    defineSymbolM!(text, main, textord, "\u{00e6}", "\\ae", true);
    defineSymbolM!(text, main, textord, "\u{0153}", "\\oe", true);
    defineSymbolM!(text, main, textord, "\u{00f8}", "\\o", true);
    defineSymbolM!(text, main, textord, "\u{00c6}", "\\AE", true);
    defineSymbolM!(text, main, textord, "\u{0152}", "\\OE", true);
    defineSymbolM!(text, main, textord, "\u{00d8}", "\\O", true);
    defineSymbolM!(text, main, accent, "\u{02ca}", "\\'"); // acute
    defineSymbolM!(text, main, accent, "\u{02cb}", "\\`"); // grave
    defineSymbolM!(text, main, accent, "\u{02c6}", "\\^"); // circumflex
    defineSymbolM!(text, main, accent, "\u{02dc}", "\\~"); // tilde
    defineSymbolM!(text, main, accent, "\u{02c9}", "\\="); // macron
    defineSymbolM!(text, main, accent, "\u{02d8}", "\\u"); // breve
    defineSymbolM!(text, main, accent, "\u{02d9}", "\\."); // dot above
    defineSymbolM!(text, main, accent, "\u{00b8}", "\\c"); // cedilla
    defineSymbolM!(text, main, accent, "\u{02da}", "\\r"); // ring above
    defineSymbolM!(text, main, accent, "\u{02c7}", "\\v"); // caron
    defineSymbolM!(text, main, accent, "\u{00a8}", "\\\""); // diaresis
    defineSymbolM!(text, main, accent, "\u{02dd}", "\\H"); // double acute
    defineSymbolM!(text, main, accent, "\u{25ef}", "\\textcircled"); // \bigcirc glyph

    defineSymbolM!(text, main, textord, "\u{2013}", "--", true);
    defineSymbolM!(text, main, textord, "\u{2013}", "\\textendash");
    defineSymbolM!(text, main, textord, "\u{2014}", "---", true);
    defineSymbolM!(text, main, textord, "\u{2014}", "\\textemdash");
    defineSymbolM!(text, main, textord, "\u{2018}", "`", true);
    defineSymbolM!(text, main, textord, "\u{2018}", "\\textquoteleft");
    defineSymbolM!(text, main, textord, "\u{2019}", "'", true);
    defineSymbolM!(text, main, textord, "\u{2019}", "\\textquoteright");
    defineSymbolM!(text, main, textord, "\u{201c}", "``", true);
    defineSymbolM!(text, main, textord, "\u{201c}", "\\textquotedblleft");
    defineSymbolM!(text, main, textord, "\u{201d}", "''", true);
    defineSymbolM!(text, main, textord, "\u{201d}", "\\textquotedblright");
    //  \degree from gensymb package

    defineSymbolM!(text, main, textord, "\u{00b0}", "\\degree");
    // \textdegree from inputenc package
    defineSymbolM!(text, main, textord, "\u{00b0}", "\\textdegree", true);
    // TODO: In LaTeX, \pounds can generate a different character in text and math
    // mode, but among our fonts, only Main-Regular defines this character "163".

    defineSymbolM!(text, main, textord, "\u{00a3}", "\\pounds");
    defineSymbolM!(text, main, textord, "\u{00a3}", "\\textsterling", true);

    defineSymbolM!(text, ams, textord, "\u{2720}", "\\maltese");

    for ch in String::from("0123456789!@*()-=+\";:?/.,").chars() {
        defineSymbolM!(text, main, textord, ch, ch);
    }

    // All of these are textords in text mode, and mathords in math mode
    let letters: String = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
    for ch in letters.chars() {
        defineSymbolM!(text, main, textord, ch, ch);
    }

    // Blackboard bold and script letters in Unicode range
    // blackboard bold
    defineSymbolM!(text, ams, textord, "C", "\u{2102}");

    defineSymbolM!(text, ams, textord, "H", "\u{210D}");

    defineSymbolM!(text, ams, textord, "N", "\u{2115}");

    defineSymbolM!(text, ams, textord, "P", "\u{2119}");

    defineSymbolM!(text, ams, textord, "Q", "\u{211A}");

    defineSymbolM!(text, ams, textord, "R", "\u{211D}");

    defineSymbolM!(text, ams, textord, "Z", "\u{2124}");
    // italic h, Planck constant
    defineSymbolM!(text, main, mathord, "h", "\u{210E}");

    // The next loop loads wide (surrogate pair) characters.
    // We support some letters in the Unicode range U+1D400 to U+1D7FF,
    // Mathematical Alphanumeric Symbols.
    // Some editors do not deal well with wide characters. So don't write the
    // string into this file. Instead, create the string from the surrogate pair.
    let mut i = 0;
    for ch in letters.chars() {
        // The hex numbers in the next line are a surrogate pair.
        // 0xD835 is the high surrogate for all letters in the range we support.
        // 0xDC00 is the low surrogate for bold A.
        let mut wide_char = code_to_str(0xD835, 0xDC00 + i); // A-Z a-z bold;

        defineSymbolM!(text, main, textord, ch, wide_char);

        wide_char = code_to_str(0xD835, 0xDC34 + i); // A-Z a-z italic

        defineSymbolM!(text, main, textord, ch, wide_char);

        wide_char = code_to_str(0xD835, 0xDC68 + i); // A-Z a-z bold italic

        defineSymbolM!(text, main, textord, ch, wide_char);

        wide_char = code_to_str(0xD835, 0xDD04 + i); // A-Z a-z Fractur

        defineSymbolM!(text, main, textord, ch, wide_char);

        wide_char = code_to_str(0xD835, 0xDDA0 + i); // A-Z a-z sans-serif

        defineSymbolM!(text, main, textord, ch, wide_char);

        wide_char = code_to_str(0xD835, 0xDDD4 + i); // A-Z a-z sans bold

        defineSymbolM!(text, main, textord, ch, wide_char);

        wide_char = code_to_str(0xD835, 0xDE08 + i); // A-Z a-z sans italic

        defineSymbolM!(text, main, textord, ch, wide_char);

        wide_char = code_to_str(0xD835, 0xDE70 + i); // A-Z a-z monospace

        defineSymbolM!(text, main, textord, ch, wide_char);

        if i < 26 {
            // KaTeX fonts have only capital letters for blackboard bold and script.
            // See exception for k below.
            wide_char = code_to_str(0xD835, 0xDD38 + i); // A-Z double struck

            defineSymbolM!(text, main, textord, ch, wide_char);

            wide_char = code_to_str(0xD835, 0xDC9C + i); // A-Z script

            defineSymbolM!(text, main, textord, ch, wide_char);
        }
        i += 1;

        // TODO: Add bold script when it is supported by a KaTeX font.
    }
    // "k" is the only double struck lower case letter in the KaTeX fonts.
    // k double struck

    defineSymbolM!(text, main, textord, "k", code_to_str(0xD835, 0xDD5C));

    // Next, some wide character numerals
    i=0;
    for ch in '0'..'9' {
        let mut wide_char = code_to_str(0xD835, 0xDFCE + i); // 0-9 bold

        defineSymbolM!(text, main, textord, ch, wide_char);

        wide_char = code_to_str(0xD835, 0xDFE2 + i); // 0-9 sans serif

        defineSymbolM!(text, main, textord, ch, wide_char);

        wide_char = code_to_str(0xD835, 0xDFEC + i); // 0-9 bold sans

        defineSymbolM!(text, main, textord, ch, wide_char);

        wide_char = code_to_str(0xD835, 0xDFF6 + i); // 0-9 monospace

        defineSymbolM!(text, main, textord, ch, wide_char);
        i+=1;
    }

    // We add these Latin-1 letters as symbols for backwards-compatibility,
    // but they are not actually in the font, nor are they supported by the
    // Unicode accent mechanism, so they fall back to Times font and look ugly.
    // TODO(edemaine): Fix this.
    for ch in String::from("\u{00d0}\u{00de}\u{00fe}").chars() {
        defineSymbolM!(text, main, textord, ch, ch);
    }

    text
}
