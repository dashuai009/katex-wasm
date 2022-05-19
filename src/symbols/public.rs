pub use std::{collections::HashMap, str::FromStr};


pub use crate::types::Mode;


#[derive(Debug, PartialEq, Clone)]
pub enum Font {
    main,
    ams,
}

impl FromStr for Font {
    type Err = ();

    fn from_str(input: &str) -> Result<Font, Self::Err> {
        match input {
            "main" => Ok(Font::main),
            "ams" => Ok(Font::ams),
            _ => Err(()),
        }
    }
}
impl Font {
    fn as_str(&self) -> &'static str {
        match self {
            Font::main => "main",
            Font::ams => "ams",
        }
    }
}

// groups:
// const accent = "accent-token";
// const bin = "bin";
// const close = "close";
// const inner = "inner";
// const mathord = "mathord";
// const op = "op-token";
// const open = "open";
// const punct = "punct";
// const rel = "rel";
// const spacing = "spacing";
// const textord = "textord";
#[derive(Debug, Clone)]
pub enum Group {
    accent,
    bin,
    close,
    inner,
    mathord,
    op,
    open,
    punct,
    rel,
    spacing,
    textord,
}

impl Group {
    fn as_str(&self) -> &'static str {
        match self {
            Group::accent => "accent-token",
            Group::bin => "bin",
            Group::close => "close",
            Group::inner => "inner",
            Group::mathord => "mathord",
            Group::op => "op-token",
            Group::open => "open",
            Group::punct => "punct",
            Group::rel => "rel",
            Group::spacing => "spacing",
            Group::textord => "textord",
        }
    }
}
#[derive(Debug, Clone)]
pub struct Symbol {
    pub font: Font,
    pub group: Group,
    pub replace: Option<String>,
}

impl Symbol {
    pub fn to_js_object(&self) -> js_sys::Object {
        let mut m = js_sys::Object::new();
        js_sys::Reflect::set(
            &m,
            &js_sys::JsString::from("font"),
            &js_sys::JsString::from(self.font.as_str()),
        );
        js_sys::Reflect::set(
            &m,
            &js_sys::JsString::from("group"),
            &js_sys::JsString::from(self.group.as_str()),
        );
        if self.replace.is_some() {
            js_sys::Reflect::set(
                &m,
                &js_sys::JsString::from("replace"),
                &js_sys::JsString::from(self.replace.as_ref().unwrap().clone()),
            );
        }
        m
    }
}



pub fn defineSymbol2(
    map: &mut HashMap<String, Symbol>,
    font: Font,
    group: Group,
    replace: Option<String>,
    name: String,
    acceptUnicodeChar: bool,
) {
    let s = Symbol {
        font,
        group,
         replace,
    };
    let t = s.clone();

    if acceptUnicodeChar {
        match s.replace {
            Some(r) => {
                map.insert(
                    name,
                    Symbol {
                        font: s.font.clone(),
                        group: s.group.clone(),
                        replace: Some(r.clone()),
                    },
                );
                map.insert(r, t);
            }
            None => {}
        }
    } else {
        map.insert(name, s.clone());
    }
}

macro_rules! defineSymbolM {
    ($_map:ident,$_font:ident, $_group:ident, None, $_name:expr, $_accecpt:expr) => {
        defineSymbol2(
            &mut $_map,
            $_font,
            $_group,
            None,
            $_name.to_string(),
            $_accecpt
        )
    };
    ($_map:ident,$_font:ident, $_group:ident, $_replace:expr, $_name:expr, $_accecpt:expr) => {
        defineSymbol2(
            &mut $_map,
            $_font,
            $_group,
            Some(($_replace).to_string()),
            $_name.to_string(),
            $_accecpt
        )
    };
    ($_map:ident,$_font:ident, $_group:ident, $_replace:expr, $_name:expr) => {
        defineSymbol2(
            &mut $_map,
            $_font,
            $_group,
            Some(($_replace).to_string()),
            $_name.to_string(),
            false
        )
    };
}

pub(crate) use defineSymbolM;

pub fn code_to_str(a: u16, b: u16) -> String {
    return String::from_utf16(&[a,b]).unwrap();
}

