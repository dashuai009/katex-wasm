
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct unicodeAccentsTextAndMath {
    text: &'static str,
    math: &'static str,
}

impl unicodeAccentsTextAndMath{
    pub fn get(&self, mode:Mode)->&'static str{
        match mode{
            Mode::math => self.math,
            Mode::text => self.text,
        }
    }
}
use std::collections::HashMap;

use crate::symbols::public::Mode;
lazy_static! {
    /**
     * Unicode block data for the families of scripts we support in \text{}.
     * Scripts only need to appear here if they do not have font metrics.
    */
    pub static ref unicodeAccents: HashMap<char, unicodeAccentsTextAndMath> = {
        let mut m = HashMap::new();
        m.insert(
            '\u{0301}',
            unicodeAccentsTextAndMath {
                text: "\\'",
                math: "\\acute",
            },
        );
        m.insert(
            '\u{0300}',
            unicodeAccentsTextAndMath {
                text: "\\`",
                math: "\\grave",
            },
        );
        m.insert(
            '\u{0308}',
            unicodeAccentsTextAndMath {
                text: "\\",
                math: "\\ddot",
            },
        );
        m.insert(
            '\u{0303}',
            unicodeAccentsTextAndMath {
                text: "\\~",
                math: "\\tilde",
            },
        );
        m.insert(
            '\u{0304}',
            unicodeAccentsTextAndMath {
                text: "\\=",
                math: "\\bar",
            },
        );
        m.insert(
            '\u{0306}',
            unicodeAccentsTextAndMath {
                text: "\\u",
                math: "\\breve",
            },
        );
        m.insert(
            '\u{030c}',
            unicodeAccentsTextAndMath {
                text: "\\v",
                math: "\\check",
            },
        );
        m.insert(
            '\u{0302}',
            unicodeAccentsTextAndMath {
                text: "\\^",
                math: "\\hat",
            },
        );
        m.insert(
            '\u{0307}',
            unicodeAccentsTextAndMath {
                text: "\\.",
                math: "\\dot",
            },
        );
        m.insert(
            '\u{030a}',
            unicodeAccentsTextAndMath {
                text: "\\r",
                math: "\\mathring",
            },
        );
        m.insert(
            '\u{030b}',
            unicodeAccentsTextAndMath {
                text: "\\H",
                math: "",
            },
        );
        m.insert(
            '\u{0327}',
            unicodeAccentsTextAndMath {
                text: "\\c",
                math: "",
            },
        );
        m
    };
}

// #[wasm_bindgen]
// pub fn __unicodeAccents() -> js_sys::Object {
//     let mut res = js_sys::Object::new();
//     for item in unicodeAccents.iter() {
//         let mut j = js_sys::Object::new();
//         js_sys::Reflect::set(
//             &j,
//             &JsValue::from_str("text"),
//             &JsValue::from_str(&item.1.text),
//         );
//         js_sys::Reflect::set(
//             &j,
//             &JsValue::from_str("math"),
//             &JsValue::from_str(&item.1.math),
//         );
//         js_sys::Reflect::set(&res, &JsValue::from_str(&item.0), &j);
//     }
//     res
// }
