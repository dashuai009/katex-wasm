use crate::dom_tree::span::Span;
use crate::parse::parseTree;
use crate::settings::Settings;
use crate::VirtualNode;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

/**
 * Generates and returns the katex build tree. This is used for advanced
 * use cases (like rendering to custom output).
 */
pub fn render_to_dom_tree(expression: String, settings: Settings) -> Span {
    let tree = parseTree(expression.clone(), settings.clone());
    println!("tree parse nodes = {:#?}", tree);
    return crate::build::build_tree(tree, expression, settings);
}

pub fn render(expression: String, base_node: &web_sys::Node, options: &JsValue) {
    base_node.set_text_content(Some(""));
    let node = render_to_dom_tree(expression, Settings::new_from_js(options)).to_node();
    base_node.append_child(&node);
}

/**
 * Parse and build an expression, and return the markup for that.
 */
pub fn render_to_string(expression: String, settings: Settings) -> String {
    return render_to_dom_tree(expression, settings).to_markup();
}

#[cfg(test)]
mod tests {
    use crate::katex::render_to_string;
    use crate::settings::Settings;

    #[test]
    fn test_parse_tree() {
        let mut settings = Settings::new();
        settings.set_display_mode(true);
        settings.set_error_color("#cc0000".to_string());
        settings.trust = true;


        settings.set_max_expand(Some(1000));
        settings.set_max_size(Some(200000.0));
        println!("setting = {:#?}", settings);
        let test_string = "E=mc^2".to_string();
        println!("{}", render_to_string(test_string, settings).as_str());
    }
}
