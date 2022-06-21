use crate::dom_tree::css_style::CssStyle;
use crate::utils::{escape, make_em};
use crate::{scriptFromCodepoint, VirturalNode};
use js_sys::Array;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
lazy_static! {
    static ref iCombinations:Mutex<HashMap<&'static str,&'static str> >  = Mutex::new({
    HashMap::from([
            ("î", "\u{0131}\u{0302}"),
            ("ï", "\u{0131}\u{0308}"),
            ("í", "\u{0131}\u{0301}"),
            // 'ī': '\u0131\u0304', // enable when we add Extended Latin
            ("ì", "\u{0131}\u{0300}")
        ])
    });
}

/**
 * A symbol node contains information about a single symbol. It either renders
 * to a single text node, or a span with a single text node in it, depending on
 * whether it has CSS classes, styles, or needs italic correction.
 */
#[wasm_bindgen(getter_with_clone)]
pub struct SymbolNode {
    text: String,
    pub height: f64,
    pub depth: f64,
    pub italic: f64,
    pub skew: f64,
    pub width: f64,
    pub maxFontSize: f64,
    classes: Vec<String>,
    style: CssStyle,
}

impl SymbolNode {
    pub fn set_style_color(&mut self, c: Option<String>) {
        self.style.color = c;
    }
}
#[wasm_bindgen]
impl SymbolNode {
    #[wasm_bindgen(getter)]
    pub fn style(&self) -> CssStyle {
        self.style.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_style(&mut self, style: &CssStyle) {
        self.style = style.clone()
    }
}
#[wasm_bindgen]
impl SymbolNode {
    #[wasm_bindgen(constructor)]
    pub fn new(text: String) -> SymbolNode {
        let mut res = SymbolNode {
            text,
            height: 0.0,
            depth: 0.0,
            italic: 0.0,
            skew: 0.0,
            width: 0.0,
            maxFontSize: 0.0,
            classes: vec![],
            style: CssStyle::default(),
        };

        // Mark text from non-Latin scripts with specific classes so that we
        // can specify which fonts to use.  This allows us to render these
        // characters with a serif font in situations where the browser would
        // either default to a sans serif or render a placeholder character.
        // We use CSS class names like cjk_fallback, hangul_fallback and
        // brahmic_fallback. See ./unicodeScripts.js for the set of possible
        // script names
        if let Some(script) = scriptFromCodepoint(res.text.chars().next().unwrap() as u32 as f64) {
            res.classes.push(script + "_fallback");
        }

        let i_comb = iCombinations.lock().unwrap();
        if i_comb.contains_key(res.text.as_str()) {
            // add ī when we add Extended Latin
            res.text = i_comb.get(&*res.text).unwrap().to_string();
        }
        res
    }

    pub fn hasClass(&self, class_name: String) -> bool {
        self.classes.contains(&class_name)
    }

    /**
     * Creates a text node or span from a symbol node. Note that a span is only
     * created if it is needed.
     */
    pub fn toNode(&self) -> web_sys::Node {
        let document = web_sys::window().expect("").document().expect("");
        let node = document.create_text_node(&self.text);
        let st = self.style.to_css_str();
        if self.italic > 0.0 || self.classes.len() > 0 || st != "" {
            let mut span_node = document.create_element("span").expect("");
            if self.italic > 0.0 {
                web_sys::Element::set_attribute(
                    &span_node,
                    "style",
                    format!("marginRight:{};", make_em(self.italic)).as_str(),
                );
            }
            if self.classes.len() > 0 {
                web_sys::Element::set_attribute(
                    &span_node,
                    "className",
                    self.classes.join(" ").as_str(),
                );
            }
            if st != "" {
                web_sys::Element::set_attribute(&span_node, "style", st.as_str());
            }
            span_node.append_child(&node);
            return web_sys::Node::from(span_node);
        } else {
            return web_sys::Node::from(node);
        }
    }

    /**
     * Creates markup for a symbol node.
     */
    pub fn toMarkup(&self) -> String {
        // TODO(alpert): More duplication than I'd like from
        // span.prototype.toMarkup and symbolNode.prototype.toNode...
        let mut needsSpan = false;

        let mut markup = "<span".to_string();

        if self.classes.len() > 0 {
            needsSpan = true;
            markup.push_str(&format!(
                " class=\"{}\"",
                escape(&self.classes.join(" ")).as_str()
            ));
        }

        let mut styles = String::new();

        if self.italic > 0.0 {
            styles.push_str(&format!("margin-right:{}em", self.italic).as_str());
        }

        styles.push_str(&self.style.to_css_str());

        let escaped_text = escape(&self.text);
        return if styles != "" {
            markup.push_str(&format!(" style=\"{}\"", escape(&styles.to_string())).as_str());

            markup.push_str(&format!(">{escaped_text}</span>").as_str());
            markup.to_string()
        } else {
            escaped_text
        };
    }
}

#[wasm_bindgen]
impl SymbolNode {
    #[wasm_bindgen(getter)]
    pub fn classes(&self) -> Array {
        let arr = Array::new_with_length(self.classes.len() as u32);
        for (i, s) in self.classes.iter().enumerate() {
            arr.set(i as u32, JsValue::from_str(s));
        }
        arr
    }
}

impl SymbolNode {
    pub fn set_classes(&mut self, c: Vec<String>) {
        self.classes = c;
    }

    pub fn push_class(&mut self, s: String) {
        self.classes.push(s);
    }
}
