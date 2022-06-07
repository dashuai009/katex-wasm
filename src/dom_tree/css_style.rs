use wasm_bindgen::prelude::*;
use struct_format::format;

macro_rules! css_to_string {
    ($($key:ident : $t:ty),*)=>{
        let mut res = String::new();
        $(
            res.push(format!("{}:{};",stringify!($key),self.$key));
        )*
        return res;
    }
}
/**
 * This node represents an image embed (<img>) element.
 */
#[derive(Debug, Clone)]
// #[derive(format)]
#[wasm_bindgen]
pub struct CssStyle {
    backgroundColor: String,
    borderBottomWidth: String,
    borderColor: String,
    borderRightStyle: String,
    borderRightWidth: String,
    borderTopWidth: String,
    borderStyle: String,
    borderWidth: String,
    bottom: String,
    color: String,
    height: String,
    left: String,
    margin: String,
    marginLeft: String,
    marginRight: String,
    marginTop: String,
    minWidth: String,
    paddingLeft: String,
    position: String,
    top: String,
    width: String,
    verticalAlign: String,
}

impl CssStyle {
    pub fn to_string(&self) -> String {
        String::new()
    }
}
