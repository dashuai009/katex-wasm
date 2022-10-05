use crate::dom_tree::span::Span;
use crate::parse::parseTree;
use crate::settings::Settings;
use crate::VirtualNode;

/**
 * Generates and returns the katex build tree. This is used for advanced
 * use cases (like rendering to custom output).
 */
pub fn render_to_dom_tree(expression: String, options: Settings) -> Span {
    let settings = Settings::new();//TODO
    let tree = parseTree(expression.clone(), settings.clone());
    return crate::build::tree::build_tree(tree, expression, settings);
}

// pub fn render(
//     expression: string,
//     baseNode: Node,
//     options: SettingsOptions,
// ) {
//     baseNode.textContent = "";
//     let node = render_to_dom_tree(expression, options).toNode();
//     baseNode.appendChild(node);
// }

/**
 * Parse and build an expression, and return the markup for that.
 */
pub fn render_to_string(expression: String, options: Settings) -> String {
    return render_to_dom_tree(expression, options).to_markup();
}

#[cfg(test)]
mod tests {
    use crate::katex::render_to_string;
    use crate::settings::Settings;

    #[test]
    fn test_parse_tree() {
        let settings = Settings::new();
        println!("setting = {}", settings);
        let test_string = "E=mc^2".to_string();
        println!("{}", render_to_string(test_string, settings).as_str());
    }
}
