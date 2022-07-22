use struct_format::format;
use wasm_bindgen::prelude::*;

/**
 * This node represents an image embed (<img>) element.
 */
#[derive(Debug, Clone, format, Default, PartialEq)]
#[wasm_bindgen(getter_with_clone)]
pub struct CssStyle {
    background_color: Option<String>,
    border_bottom_width: Option<String>,
    border_color: Option<String>,
    border_right_style: Option<String>,
    border_right_width: Option<String>,
    border_top_width: Option<String>,
    border_style: Option<String>,
    border_width: Option<String>,
    pub bottom: Option<String>,
    pub color: Option<String>,
    pub height: Option<String>,
    pub left: Option<String>,
    pub margin: Option<String>,
    margin_left: Option<String>,
    margin_right: Option<String>,
    margin_top: Option<String>,
    min_width: Option<String>,
    padding_left: Option<String>,
    pub position: Option<String>,
    pub top: Option<String>,
    pub width: Option<String>,
    vertical_align: Option<String>,
}

#[wasm_bindgen]
impl CssStyle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CssStyle {
        CssStyle::default()
    }
}
