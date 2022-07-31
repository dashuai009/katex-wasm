use struct_format::parse_node_type;
use wasm_bindgen::prelude::*;

use crate::{sourceLocation::SourceLocation, symbols::public::Mode};

pub(crate) trait AnyParseNode {}

pub(crate) trait GetMode {
    fn get_mode(&self) -> Mode;
}
pub(crate) trait GetText {
    fn get_text(&self) -> String;
}
#[derive(parse_node_type)]
#[wasm_bindgen]
pub struct raw {
    mode: Mode,
    loc: Option<SourceLocation>,
    string: String,
}

impl GetMode for raw {
    fn get_mode(&self) -> Mode {
        return self.mode;
    }
}
// impl GetText for raw{
//     fn get_text(&self)->String{
//         return self.text;
//     }
// }
