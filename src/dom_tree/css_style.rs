use std::fmt::Formatter;
use struct_format::format;
use wasm_bindgen::prelude::*;

/**
 * This node represents an image embed (<img>) element.
 */
#[derive(Clone, format, Default, PartialEq)]
#[wasm_bindgen(getter_with_clone)]
pub struct CssStyle {
    pub background_color: Option<String>,
    pub border_bottom_width: Option<String>,
    pub border_color: Option<String>,
    pub border_right_style: Option<String>,
    pub border_right_width: Option<String>,
    pub border_top_width: Option<String>,
    pub border_style: Option<String>,
    pub border_width: Option<String>,
    pub bottom: Option<String>,
    pub color: Option<String>,
    pub height: Option<String>,
    pub left: Option<String>,
    pub margin: Option<String>,
    pub margin_left: Option<String>,
    pub margin_right: Option<String>,
    pub margin_top: Option<String>,
    pub min_width: Option<String>,
    pub padding_left: Option<String>,
    pub position: Option<String>,
    pub top: Option<String>,
    pub width: Option<String>,
    pub vertical_align: Option<String>,
}

#[wasm_bindgen]
impl CssStyle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CssStyle {
        CssStyle::default()
    }
}

impl std::fmt::Debug for CssStyle{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CssStyle {}",self.to_css_str())
    }
}


#[cfg(test)]
mod tests {
    use crate::dom_tree::css_style::CssStyle;

    #[test]
    fn test_css_style_format() {
        let default_css_style = CssStyle::new();
        println!("default_css_style = {}", default_css_style.to_css_str());
        let mut test_css_style = CssStyle::new();
        test_css_style.border_style = Some("aaa".to_string());
        println!("test_css_style = {}", test_css_style.to_css_str());
        test_css_style.min_width = Some("aaa".to_string());
        println!("test_css_style = {}", test_css_style.to_css_str());
    }
}
