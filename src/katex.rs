/**
 * Generates and returns the katex build tree. This is used for advanced
 * use cases (like rendering to custom output).
 */
pub fn renderToDomTree(    expression: String,
    options: SettingsOptions,
)-> DomSpan {
    let settings = Settings::new(options);
        let tree = parseTree(expression, settings);
        return buildTree(tree, expression, settings);
}


// pub fn render(
//     expression: string,
//     baseNode: Node,
//     options: SettingsOptions,
// ) {
//     baseNode.textContent = "";
//     let node = renderToDomTree(expression, options).toNode();
//     baseNode.appendChild(node);
// }


/**
 * Parse and build an expression, and return the markup for that.
 */
pub fn render_to_string(    expression: String,    options: SettingsOptions)-> String {
    return renderToDomTree(expression, options).toMarkup();
}