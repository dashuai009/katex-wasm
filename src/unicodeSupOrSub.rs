use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    pub static ref U_SUBS_AND_SUPS: Mutex<HashMap<&'static str, &'static str>> = Mutex::new({
        let res = HashMap::from([
            ("₊", "+"),
            ("₋", "-"),
            ("₌", "="),
            ("₍", "("),
            ("₎", ")"),
            ("₀", "0"),
            ("₁", "1"),
            ("₂", "2"),
            ("₃", "3"),
            ("₄", "4"),
            ("₅", "5"),
            ("₆", "6"),
            ("₇", "7"),
            ("₈", "8"),
            ("₉", "9"),
            ("\u{2090}", "a"),
            ("\u{2091}", "e"),
            ("\u{2095}", "h"),
            ("\u{1D62}", "i"),
            ("\u{2C7C}", "j"),
            ("\u{2096}", "k"),
            ("\u{2097}", "l"),
            ("\u{2098}", "m"),
            ("\u{2099}", "n"),
            ("\u{2092}", "o"),
            ("\u{209A}", "p"),
            ("\u{1D63}", "r"),
            ("\u{209B}", "s"),
            ("\u{209C}", "t"),
            ("\u{1D64}", "u"),
            ("\u{1D65}", "v"),
            ("\u{2093}", "x"),
            ("\u{1D66}", "β"),
            ("\u{1D67}", "γ"),
            ("\u{1D68}", "ρ"),
            ("\u{1D69}", "\u{03d5}"),
            ("\u{1D6A}", "χ"),
            ("⁺", "+"),
            ("⁻", "-"),
            ("⁼", "="),
            ("⁽", "("),
            ("⁾", ")"),
            ("⁰", "0"),
            ("¹", "1"),
            ("²", "2"),
            ("³", "3"),
            ("⁴", "4"),
            ("⁵", "5"),
            ("⁶", "6"),
            ("⁷", "7"),
            ("⁸", "8"),
            ("⁹", "9"),
            ("\u{1D2C}", "A"),
            ("\u{1D2E}", "B"),
            ("\u{1D30}", "D"),
            ("\u{1D31}", "E"),
            ("\u{1D33}", "G"),
            ("\u{1D34}", "H"),
            ("\u{1D35}", "I"),
            ("\u{1D36}", "J"),
            ("\u{1D37}", "K"),
            ("\u{1D38}", "L"),
            ("\u{1D39}", "M"),
            ("\u{1D3A}", "N"),
            ("\u{1D3C}", "O"),
            ("\u{1D3E}", "P"),
            ("\u{1D3F}", "R"),
            ("\u{1D40}", "T"),
            ("\u{1D41}", "U"),
            ("\u{2C7D}", "V"),
            ("\u{1D42}", "W"),
            ("\u{1D43}", "a"),
            ("\u{1D47}", "b"),
            ("\u{1D9C}", "c"),
            ("\u{1D48}", "d"),
            ("\u{1D49}", "e"),
            ("\u{1DA0}", "f"),
            ("\u{1D4D}", "g"),
            ("\u{02B0}", "h"),
            ("\u{2071}", "i"),
            ("\u{02B2}", "j"),
            ("\u{1D4F}", "k"),
            ("\u{02E1}", "l"),
            ("\u{1D50}", "m"),
            ("\u{207F}", "n"),
            ("\u{1D52}", "o"),
            ("\u{1D56}", "p"),
            ("\u{02B3}", "r"),
            ("\u{02E2}", "s"),
            ("\u{1D57}", "t"),
            ("\u{1D58}", "u"),
            ("\u{1D5B}", "v"),
            ("\u{02B7}", "w"),
            ("\u{02E3}", "x"),
            ("\u{02B8}", "y"),
            ("\u{1DBB}", "z"),
            ("\u{1D5D}", "β"),
            ("\u{1D5E}", "γ"),
            ("\u{1D5F}", "δ"),
            ("\u{1D60}", "\u{03d5}"),
            ("\u{1D61}", "χ"),
            ("\u{1DBF}", "θ"),
        ]);
        res
    });
}
