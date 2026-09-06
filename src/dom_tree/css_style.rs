use wasm_bindgen::prelude::*;

/// Sparse style properties in canonical order. Most nodes have zero or one
/// property; keep that case inline instead of reserving 23 optional Strings.
#[derive(Clone, Default)]
#[wasm_bindgen]
pub struct CssStyle {
    values: smallvec::SmallVec<[(u8, Option<String>); 1]>,
}
pub type StyleStorage = CssStyle;

impl PartialEq for CssStyle {
    fn eq(&self, other: &Self) -> bool {
        self.values
            .iter()
            .filter_map(|(key, value)| value.as_ref().map(|value| (key, value)))
            .eq(other
                .values
                .iter()
                .filter_map(|(key, value)| value.as_ref().map(|value| (key, value))))
    }
}

impl CssStyle {
    fn value(&self, key: u8) -> &Option<String> {
        static NONE: Option<String> = None;
        self.values
            .iter()
            .find(|entry| entry.0 == key)
            .map(|entry| &entry.1)
            .unwrap_or(&NONE)
    }
    fn value_mut(&mut self, key: u8) -> &mut Option<String> {
        let index = match self.values.binary_search_by_key(&key, |entry| entry.0) {
            Ok(index) => index,
            Err(index) => {
                self.values.insert(index, (key, None));
                index
            }
        };
        &mut self.values[index].1
    }
    pub fn is_empty(&self) -> bool {
        self.values.iter().all(|entry| entry.1.is_none())
    }
    pub fn to_css_str(&self) -> String {
        let mut output = String::new();
        for (key, value) in &self.values {
            if let Some(value) = value {
                output.push_str(NAMES[*key as usize]);
                output.push_str(value);
                output.push(';');
            }
        }
        output
    }
    pub fn write_css_escaped(&self, output: &mut String) {
        for (key, value) in &self.values {
            if let Some(value) = value {
                output.push_str(NAMES[*key as usize]);
                crate::utils::escape_to(output, value);
                output.push(';');
            }
        }
    }
    pub fn background_color_mut(&mut self) -> &mut Option<String> {
        self.value_mut(0)
    }
    pub fn border_bottom_width_mut(&mut self) -> &mut Option<String> {
        self.value_mut(1)
    }
    pub fn border_color_mut(&mut self) -> &mut Option<String> {
        self.value_mut(2)
    }
    pub fn border_right_style_mut(&mut self) -> &mut Option<String> {
        self.value_mut(3)
    }
    pub fn border_right_width_mut(&mut self) -> &mut Option<String> {
        self.value_mut(4)
    }
    pub fn border_top_width_mut(&mut self) -> &mut Option<String> {
        self.value_mut(5)
    }
    pub fn border_style_mut(&mut self) -> &mut Option<String> {
        self.value_mut(6)
    }
    pub fn border_width_mut(&mut self) -> &mut Option<String> {
        self.value_mut(7)
    }
    pub fn bottom_mut(&mut self) -> &mut Option<String> {
        self.value_mut(8)
    }
    pub fn color_mut(&mut self) -> &mut Option<String> {
        self.value_mut(9)
    }
    pub fn height_mut(&mut self) -> &mut Option<String> {
        self.value_mut(10)
    }
    pub fn left_mut(&mut self) -> &mut Option<String> {
        self.value_mut(11)
    }
    pub fn margin_mut(&mut self) -> &mut Option<String> {
        self.value_mut(12)
    }
    pub fn margin_left_mut(&mut self) -> &mut Option<String> {
        self.value_mut(13)
    }
    pub fn margin_right_mut(&mut self) -> &mut Option<String> {
        self.value_mut(14)
    }
    pub fn margin_top_mut(&mut self) -> &mut Option<String> {
        self.value_mut(15)
    }
    pub fn min_width_mut(&mut self) -> &mut Option<String> {
        self.value_mut(16)
    }
    pub fn padding_left_mut(&mut self) -> &mut Option<String> {
        self.value_mut(17)
    }
    pub fn position_mut(&mut self) -> &mut Option<String> {
        self.value_mut(18)
    }
    pub fn text_shadow_mut(&mut self) -> &mut Option<String> {
        self.value_mut(19)
    }
    pub fn top_mut(&mut self) -> &mut Option<String> {
        self.value_mut(20)
    }
    pub fn width_mut(&mut self) -> &mut Option<String> {
        self.value_mut(21)
    }
    pub fn vertical_align_mut(&mut self) -> &mut Option<String> {
        self.value_mut(22)
    }
}

#[wasm_bindgen]
impl CssStyle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CssStyle {
        Self::default()
    }
    #[wasm_bindgen(getter = backgroundColor)]
    pub fn background_color(&self) -> Option<String> {
        self.value(0).clone()
    }
    #[wasm_bindgen(setter = backgroundColor)]
    pub fn set_background_color(&mut self, value: Option<String>) {
        *self.value_mut(0) = value;
    }
    #[wasm_bindgen(getter = background_color)]
    pub fn get_background_color_alias(&self) -> Option<String> {
        self.background_color()
    }
    #[wasm_bindgen(setter = background_color)]
    pub fn set_background_color_alias(&mut self, value: Option<String>) {
        self.set_background_color(value);
    }
    #[wasm_bindgen(getter = borderBottomWidth)]
    pub fn border_bottom_width(&self) -> Option<String> {
        self.value(1).clone()
    }
    #[wasm_bindgen(setter = borderBottomWidth)]
    pub fn set_border_bottom_width(&mut self, value: Option<String>) {
        *self.value_mut(1) = value;
    }
    #[wasm_bindgen(getter = border_bottom_width)]
    pub fn get_border_bottom_width_alias(&self) -> Option<String> {
        self.border_bottom_width()
    }
    #[wasm_bindgen(setter = border_bottom_width)]
    pub fn set_border_bottom_width_alias(&mut self, value: Option<String>) {
        self.set_border_bottom_width(value);
    }
    #[wasm_bindgen(getter = borderColor)]
    pub fn border_color(&self) -> Option<String> {
        self.value(2).clone()
    }
    #[wasm_bindgen(setter = borderColor)]
    pub fn set_border_color(&mut self, value: Option<String>) {
        *self.value_mut(2) = value;
    }
    #[wasm_bindgen(getter = border_color)]
    pub fn get_border_color_alias(&self) -> Option<String> {
        self.border_color()
    }
    #[wasm_bindgen(setter = border_color)]
    pub fn set_border_color_alias(&mut self, value: Option<String>) {
        self.set_border_color(value);
    }
    #[wasm_bindgen(getter = borderRightStyle)]
    pub fn border_right_style(&self) -> Option<String> {
        self.value(3).clone()
    }
    #[wasm_bindgen(setter = borderRightStyle)]
    pub fn set_border_right_style(&mut self, value: Option<String>) {
        *self.value_mut(3) = value;
    }
    #[wasm_bindgen(getter = border_right_style)]
    pub fn get_border_right_style_alias(&self) -> Option<String> {
        self.border_right_style()
    }
    #[wasm_bindgen(setter = border_right_style)]
    pub fn set_border_right_style_alias(&mut self, value: Option<String>) {
        self.set_border_right_style(value);
    }
    #[wasm_bindgen(getter = borderRightWidth)]
    pub fn border_right_width(&self) -> Option<String> {
        self.value(4).clone()
    }
    #[wasm_bindgen(setter = borderRightWidth)]
    pub fn set_border_right_width(&mut self, value: Option<String>) {
        *self.value_mut(4) = value;
    }
    #[wasm_bindgen(getter = border_right_width)]
    pub fn get_border_right_width_alias(&self) -> Option<String> {
        self.border_right_width()
    }
    #[wasm_bindgen(setter = border_right_width)]
    pub fn set_border_right_width_alias(&mut self, value: Option<String>) {
        self.set_border_right_width(value);
    }
    #[wasm_bindgen(getter = borderTopWidth)]
    pub fn border_top_width(&self) -> Option<String> {
        self.value(5).clone()
    }
    #[wasm_bindgen(setter = borderTopWidth)]
    pub fn set_border_top_width(&mut self, value: Option<String>) {
        *self.value_mut(5) = value;
    }
    #[wasm_bindgen(getter = border_top_width)]
    pub fn get_border_top_width_alias(&self) -> Option<String> {
        self.border_top_width()
    }
    #[wasm_bindgen(setter = border_top_width)]
    pub fn set_border_top_width_alias(&mut self, value: Option<String>) {
        self.set_border_top_width(value);
    }
    #[wasm_bindgen(getter = borderStyle)]
    pub fn border_style(&self) -> Option<String> {
        self.value(6).clone()
    }
    #[wasm_bindgen(setter = borderStyle)]
    pub fn set_border_style(&mut self, value: Option<String>) {
        *self.value_mut(6) = value;
    }
    #[wasm_bindgen(getter = border_style)]
    pub fn get_border_style_alias(&self) -> Option<String> {
        self.border_style()
    }
    #[wasm_bindgen(setter = border_style)]
    pub fn set_border_style_alias(&mut self, value: Option<String>) {
        self.set_border_style(value);
    }
    #[wasm_bindgen(getter = borderWidth)]
    pub fn border_width(&self) -> Option<String> {
        self.value(7).clone()
    }
    #[wasm_bindgen(setter = borderWidth)]
    pub fn set_border_width(&mut self, value: Option<String>) {
        *self.value_mut(7) = value;
    }
    #[wasm_bindgen(getter = border_width)]
    pub fn get_border_width_alias(&self) -> Option<String> {
        self.border_width()
    }
    #[wasm_bindgen(setter = border_width)]
    pub fn set_border_width_alias(&mut self, value: Option<String>) {
        self.set_border_width(value);
    }
    #[wasm_bindgen(getter = bottom)]
    pub fn bottom(&self) -> Option<String> {
        self.value(8).clone()
    }
    #[wasm_bindgen(setter = bottom)]
    pub fn set_bottom(&mut self, value: Option<String>) {
        *self.value_mut(8) = value;
    }
    #[wasm_bindgen(getter = color)]
    pub fn color(&self) -> Option<String> {
        self.value(9).clone()
    }
    #[wasm_bindgen(setter = color)]
    pub fn set_color(&mut self, value: Option<String>) {
        *self.value_mut(9) = value;
    }
    #[wasm_bindgen(getter = height)]
    pub fn height(&self) -> Option<String> {
        self.value(10).clone()
    }
    #[wasm_bindgen(setter = height)]
    pub fn set_height(&mut self, value: Option<String>) {
        *self.value_mut(10) = value;
    }
    #[wasm_bindgen(getter = left)]
    pub fn left(&self) -> Option<String> {
        self.value(11).clone()
    }
    #[wasm_bindgen(setter = left)]
    pub fn set_left(&mut self, value: Option<String>) {
        *self.value_mut(11) = value;
    }
    #[wasm_bindgen(getter = margin)]
    pub fn margin(&self) -> Option<String> {
        self.value(12).clone()
    }
    #[wasm_bindgen(setter = margin)]
    pub fn set_margin(&mut self, value: Option<String>) {
        *self.value_mut(12) = value;
    }
    #[wasm_bindgen(getter = marginLeft)]
    pub fn margin_left(&self) -> Option<String> {
        self.value(13).clone()
    }
    #[wasm_bindgen(setter = marginLeft)]
    pub fn set_margin_left(&mut self, value: Option<String>) {
        *self.value_mut(13) = value;
    }
    #[wasm_bindgen(getter = margin_left)]
    pub fn get_margin_left_alias(&self) -> Option<String> {
        self.margin_left()
    }
    #[wasm_bindgen(setter = margin_left)]
    pub fn set_margin_left_alias(&mut self, value: Option<String>) {
        self.set_margin_left(value);
    }
    #[wasm_bindgen(getter = marginRight)]
    pub fn margin_right(&self) -> Option<String> {
        self.value(14).clone()
    }
    #[wasm_bindgen(setter = marginRight)]
    pub fn set_margin_right(&mut self, value: Option<String>) {
        *self.value_mut(14) = value;
    }
    #[wasm_bindgen(getter = margin_right)]
    pub fn get_margin_right_alias(&self) -> Option<String> {
        self.margin_right()
    }
    #[wasm_bindgen(setter = margin_right)]
    pub fn set_margin_right_alias(&mut self, value: Option<String>) {
        self.set_margin_right(value);
    }
    #[wasm_bindgen(getter = marginTop)]
    pub fn margin_top(&self) -> Option<String> {
        self.value(15).clone()
    }
    #[wasm_bindgen(setter = marginTop)]
    pub fn set_margin_top(&mut self, value: Option<String>) {
        *self.value_mut(15) = value;
    }
    #[wasm_bindgen(getter = margin_top)]
    pub fn get_margin_top_alias(&self) -> Option<String> {
        self.margin_top()
    }
    #[wasm_bindgen(setter = margin_top)]
    pub fn set_margin_top_alias(&mut self, value: Option<String>) {
        self.set_margin_top(value);
    }
    #[wasm_bindgen(getter = minWidth)]
    pub fn min_width(&self) -> Option<String> {
        self.value(16).clone()
    }
    #[wasm_bindgen(setter = minWidth)]
    pub fn set_min_width(&mut self, value: Option<String>) {
        *self.value_mut(16) = value;
    }
    #[wasm_bindgen(getter = min_width)]
    pub fn get_min_width_alias(&self) -> Option<String> {
        self.min_width()
    }
    #[wasm_bindgen(setter = min_width)]
    pub fn set_min_width_alias(&mut self, value: Option<String>) {
        self.set_min_width(value);
    }
    #[wasm_bindgen(getter = paddingLeft)]
    pub fn padding_left(&self) -> Option<String> {
        self.value(17).clone()
    }
    #[wasm_bindgen(setter = paddingLeft)]
    pub fn set_padding_left(&mut self, value: Option<String>) {
        *self.value_mut(17) = value;
    }
    #[wasm_bindgen(getter = padding_left)]
    pub fn get_padding_left_alias(&self) -> Option<String> {
        self.padding_left()
    }
    #[wasm_bindgen(setter = padding_left)]
    pub fn set_padding_left_alias(&mut self, value: Option<String>) {
        self.set_padding_left(value);
    }
    #[wasm_bindgen(getter = position)]
    pub fn position(&self) -> Option<String> {
        self.value(18).clone()
    }
    #[wasm_bindgen(setter = position)]
    pub fn set_position(&mut self, value: Option<String>) {
        *self.value_mut(18) = value;
    }
    #[wasm_bindgen(getter = textShadow)]
    pub fn text_shadow(&self) -> Option<String> {
        self.value(19).clone()
    }
    #[wasm_bindgen(setter = textShadow)]
    pub fn set_text_shadow(&mut self, value: Option<String>) {
        *self.value_mut(19) = value;
    }
    #[wasm_bindgen(getter = text_shadow)]
    pub fn get_text_shadow_alias(&self) -> Option<String> {
        self.text_shadow()
    }
    #[wasm_bindgen(setter = text_shadow)]
    pub fn set_text_shadow_alias(&mut self, value: Option<String>) {
        self.set_text_shadow(value);
    }
    #[wasm_bindgen(getter = top)]
    pub fn top(&self) -> Option<String> {
        self.value(20).clone()
    }
    #[wasm_bindgen(setter = top)]
    pub fn set_top(&mut self, value: Option<String>) {
        *self.value_mut(20) = value;
    }
    #[wasm_bindgen(getter = width)]
    pub fn width(&self) -> Option<String> {
        self.value(21).clone()
    }
    #[wasm_bindgen(setter = width)]
    pub fn set_width(&mut self, value: Option<String>) {
        *self.value_mut(21) = value;
    }
    #[wasm_bindgen(getter = verticalAlign)]
    pub fn vertical_align(&self) -> Option<String> {
        self.value(22).clone()
    }
    #[wasm_bindgen(setter = verticalAlign)]
    pub fn set_vertical_align(&mut self, value: Option<String>) {
        *self.value_mut(22) = value;
    }
    #[wasm_bindgen(getter = vertical_align)]
    pub fn get_vertical_align_alias(&self) -> Option<String> {
        self.vertical_align()
    }
    #[wasm_bindgen(setter = vertical_align)]
    pub fn set_vertical_align_alias(&mut self, value: Option<String>) {
        self.set_vertical_align(value);
    }
}
impl std::fmt::Debug for CssStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CssStyle {}", self.to_css_str())
    }
}
const NAMES: [&str; 23] = [
    "background-color:",
    "border-bottom-width:",
    "border-color:",
    "border-right-style:",
    "border-right-width:",
    "border-top-width:",
    "border-style:",
    "border-width:",
    "bottom:",
    "color:",
    "height:",
    "left:",
    "margin:",
    "margin-left:",
    "margin-right:",
    "margin-top:",
    "min-width:",
    "padding-left:",
    "position:",
    "text-shadow:",
    "top:",
    "width:",
    "vertical-align:",
];

#[cfg(test)]
mod regression_tests {
    use super::*;
    #[test]
    fn sparse_styles_keep_order_escaping_and_independent_clones() {
        let mut a = CssStyle::new();
        assert!(a.is_empty());
        *a.color_mut() = None;
        assert_eq!(a, CssStyle::new());
        *a.width_mut() = Some("2em".to_owned());
        *a.height_mut() = Some("1em".to_owned());
        assert_eq!(a.to_css_str(), "height:1em;width:2em;");
        let mut b = a.clone();
        *b.height_mut() = None;
        *b.color_mut() = Some("<&\"'".to_owned());
        let mut escaped = String::new();
        b.write_css_escaped(&mut escaped);
        assert_eq!(escaped, "color:&lt;&amp;&quot;&#x27;;width:2em;");
        assert_eq!(a.to_css_str(), "height:1em;width:2em;");
    }
}
