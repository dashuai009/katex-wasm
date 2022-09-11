/**
 * Generates and returns the katex build tree. This is used for advanced
 * use cases (like rendering to custom output).
 */
const renderToDomTree = function(
    expression: string,
    options: SettingsOptions,
): DomSpan {
    const settings = new Settings(options);
    try {
        const tree = parseTree(expression, settings);
        return buildTree(tree, expression, settings);
    } catch (error) {
        return renderError(error, expression, settings);
    }
};


let render: (string, Node, SettingsOptions) => void = function(
    expression: string,
    baseNode: Node,
    options: SettingsOptions,
) {
    baseNode.textContent = "";
    const node = renderToDomTree(expression, options).toNode();
    baseNode.appendChild(node);
};