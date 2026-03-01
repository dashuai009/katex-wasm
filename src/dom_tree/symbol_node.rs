use std::any::Any;
use crate::dom_tree::css_style::CssStyle;
use crate::units::make_em;
use crate::utils::escape;
use crate::{scriptFromCodepoint, HtmlDomNode, VirtualNode};
use js_sys::Array;
use std::collections::HashMap;
use std::sync::Mutex;
use struct_format::html_dom_node;
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

///A symbol node contains information about a single symbol. It either renders
///to a single text node, or a span with a single text node in it, depending on
///whether it has CSS classes, styles, or needs italic correction.

#[derive(html_dom_node, Clone, Debug)]
pub struct SymbolNode {
    pub text: String,
    pub height: f64,
    pub depth: f64,
    pub italic: f64,
    pub skew: f64,
    pub width: f64,
    pub max_font_size: f64,
    classes: Vec<String>,
    style: CssStyle,
}

/// 构造函数
/// 一些成员方法
impl SymbolNode {
    pub fn new(text: String) -> SymbolNode {
        let mut res = SymbolNode {
            text,
            height: 0.0,
            depth: 0.0,
            italic: 0.0,
            skew: 0.0,
            width: 0.0,
            max_font_size: 0.0,
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

    ///Returns true if subsequent symbolNodes have the same classes, skew, maxFont,
    ///and styles.
    pub fn can_combine(prev: &SymbolNode, next: &SymbolNode) -> bool {
        if prev.classes.join(" ") != next.classes.join(" ")
            || prev.skew != next.skew
            || prev.max_font_size != next.max_font_size
        {
            return false;
        }

        // If prev and next both are just "mbin"s or "mord"s we don't combine them
        // so that the proper spacing can be preserved.
        if prev.classes.len() == 1 {
            let cls = &prev.classes[0];
            if cls == "mbin" || cls == "mord" {
                return false;
            }
        }

        return prev.style == next.style;
    }
}

/// getter and setter
impl SymbolNode {
    pub fn get_text(&self) -> &String {
        return &self.text;
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}
impl VirtualNode for SymbolNode {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    /**
    ///Creates a text node or span from a symbol node. Note that a span is only
    ///created if it is needed.
     */
    fn to_node(&self) -> web_sys::Node {
        let document = web_sys::window().expect("").document().expect("");
        let node = document.create_text_node(&self.text);
        let mut styles = String::new();
        if self.italic > 0.0 {
            styles.push_str(&format!("margin-right:{}em;", self.italic));
        }
        styles.push_str(&self.style.to_css_str());
        if self.italic > 0.0 || self.classes.len() > 0 || !styles.is_empty() {
            let mut span_node = document.create_element("span").expect("");
            if self.classes.len() > 0 {
                web_sys::Element::set_attribute(
                    &span_node,
                    "class",
                    self.classes.join(" ").as_str(),
                );
            }
            if !styles.is_empty() {
                web_sys::Element::set_attribute(&span_node, "style", styles.as_str());
            }
            span_node.append_child(&node);
            return web_sys::Node::from(span_node);
        } else {
            return web_sys::Node::from(node);
        }
    }

    /**
    ///Creates markup for a symbol node.
     */
    fn to_markup(&self) -> String {
        // TODO(alpert): More duplication than I'd like from
        // span.prototype.toMarkup and symbolNode.prototype.toNode...
        let mut markup = "<span".to_string();

        if self.classes.len() > 0 {
            let cl = self.classes.iter()
                .filter(|c| !c.is_empty())
                .cloned()
                .collect::<Vec<_>>()
                .join(" ");
            markup.push_str(&format!(
                " class=\"{}\"",
                escape(&cl).as_str()
            ));
        }

        let mut styles = String::new();

        if self.italic > 0.0 {
            styles.push_str(&format!("margin-right:{}em;", self.italic).as_str());
        }

        styles.push_str(&self.style.to_css_str());
        if styles != "" {
            markup.push_str(&format!(" style=\"{}\"", escape(&styles.to_string())).as_str());
        }
        let needs_span = !self.classes.is_empty() || !styles.is_empty();
        let escaped_text = escape(&self.text);
        return if needs_span{
            markup.push_str(&format!(">{escaped_text}</span>").as_str());
            markup.to_string()
        } else {
            escaped_text
        };
    }
}
