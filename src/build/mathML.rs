use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::mathML_tree::text_node::TextNode;
use crate::parse_node::types::AnyParseNode;
use crate::tree::HtmlDomNode;
use crate::types::Mode;
use crate::Options::Options;
use crate::{get_symbol, LIGATURES};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/**
 * Takes a symbol and converts it into a MathML text node after performing
 * optional replacement from symbols.js.
 */
pub fn make_text(text: String, mode: Mode, options: Option<&Options>) -> TextNode {
    let mut flag = false;
    if LIGATURES.contains(&text.as_str()) && options.is_some() {
        if let Some(opt) = options {
            if opt.fontFamily != ""
                && &opt.fontFamily[4..6] != "tt"
                && opt.font != ""
                && &opt.font[4..6] != "tt"
            {
                flag = true;
            }
        }
    }
    if let Some(c) = text.chars().next() {
        if c as u32 != 0xD835 && !flag {
            if let Some(sym) = get_symbol(mode, &text) {
                if let Some(r) = sym.replace {
                    return TextNode::new(r);
                }
            }
        }
    }

    return TextNode::new(text);
}

// symbols[mode][text] &&
// symbols[mode][text].replace &&
// text.charCodeAt(0) !== 0xD835 &&
// !(
//     ligatures.hasOwnProperty(text) &&
//     options &&
//     (
//         (options.fontFamily && options.fontFamily.substr(4, 2) === "tt") ||
//         (options.font && options.font.substr(4, 2) === "tt")
//     )
// )

// /**
//  * Wrap the given array of nodes in an <mrow> node if needed, i.e.,
//  * unless the array has length 1.  Always returns a single node.
//  */
// pub fn makeRow(body: $ReadOnlyArray<MathDomNode>)->MathDomNode {
//     if (body.length === 1) {
//         return body[0];
//     } else {
//         return MathNode::new("mrow", body);
//     }
// };

// /**
//  * Returns the math variant as a String or null if none is required.
//  */
// pub fn getVariant(
//     group: SymbolParseNode,
//     options: Options,
// )->?FontVariant {
//     // Handle \text... font specifiers as best we can.
//     // MathML has a limited list of allowable mathvariant specifiers; see
//     // https://www.w3.org/TR/MathML3/chapter3.html#presm.commatt
//     if (options.fontFamily === "texttt") {
//         return "monospace";
//     } else if (options.fontFamily === "textsf") {
//         if (options.fontShape === "textit" &&
//             options.fontWeight === "textbf") {
//             return "sans-serif-bold-italic";
//         } else if (options.fontShape === "textit") {
//             return "sans-serif-italic";
//         } else if (options.fontWeight === "textbf") {
//             return "bold-sans-serif";
//         } else {
//             return "sans-serif";
//         }
//     } else if (options.fontShape === "textit" &&
//                options.fontWeight === "textbf") {
//         return "bold-italic";
//     } else if (options.fontShape === "textit") {
//         return "italic";
//     } else if (options.fontWeight === "textbf") {
//         return "bold";
//     }

//     let font = options.font;
//     if (!font || font === "mathnormal") {
//         return null;
//     }

//     let mode = group.mode;
//     if (font === "mathit") {
//         return "italic";
//     } else if (font === "boldsymbol") {
//         return group.type === "textord" ? "bold" : "bold-italic";
//     } else if (font === "mathbf") {
//         return "bold";
//     } else if (font === "mathbb") {
//         return "double-struck";
//     } else if (font === "mathfrak") {
//         return "fraktur";
//     } else if (font === "mathscr" || font === "mathcal") {
//         // MathML makes no distinction between script and caligrahpic
//         return "script";
//     } else if (font === "mathsf") {
//         return "sans-serif";
//     } else if (font === "mathtt") {
//         return "monospace";
//     }

//     let text = group.text;
//     if (utils.contains(["\\imath", "\\jmath"], text)) {
//         return null;
//     }

//     if (symbols[mode][text] && symbols[mode][text].replace) {
//         text = symbols[mode][text].replace;
//     }

//     let fontName = buildCommon.fontMap[font].fontName;
//     if (getCharacterMetrics(text, fontName, mode)) {
//         return buildCommon.fontMap[font].variant;
//     }

//     return null;
// };

/**
 * Takes a list of nodes, builds them, and returns a list of the generated
 * MathML nodes.  Also combine consecutive <mtext> outputs into a single
 * <mtext> tag.
 */
pub fn build_expression(
    expression: Vec<Box<dyn AnyParseNode>>,
    options: Options,
    isOrdgroup: bool,
) -> Vec<MathNode> {
    return vec![];
    // if (expression.len() == 1) {
    //     let group = buildGroup(expression[0], options);
    //     if (isOrdgroup && group instanceof MathNode && group.type === "mo") {
    //         // When TeX writers want to suppress spacing on an operator,
    //         // they often put the operator by itself inside braces.
    //         group.set_attribute("lspace", "0em");
    //         group.set_attribute("rspace", "0em");
    //     }
    //     return [group];
    // }

    // let groups = [];
    // let lastGroup;
    // for (let i = 0; i < expression.length; i++) {
    //     let group = buildGroup(expression[i], options);
    //     if (group instanceof MathNode && lastGroup instanceof MathNode) {
    //         // Concatenate adjacent <mtext>s
    //         if (group.type === 'mtext' && lastGroup.type === 'mtext'
    //             && group.getAttribute('mathvariant') ===
    //                lastGroup.getAttribute('mathvariant')) {
    //             lastGroup.children.push(...group.children);
    //             continue;
    //         // Concatenate adjacent <mn>s
    //         } else if (group.type === 'mn' && lastGroup.type === 'mn') {
    //             lastGroup.children.push(...group.children);
    //             continue;
    //         // Concatenate <mn>...</mn> followed by <mi>.</mi>
    //         } else if (group.type === 'mi' && group.children.length === 1 &&
    //                    lastGroup.type === 'mn') {
    //             let child = group.children[0];
    //             if (child instanceof TextNode && child.text === '.') {
    //                 lastGroup.children.push(...group.children);
    //                 continue;
    //             }
    //         } else if (lastGroup.type === 'mi' && lastGroup.children.length === 1) {
    //             let lastChild = lastGroup.children[0];
    //             if (lastChild instanceof TextNode && lastChild.text === '\u0338' &&
    //                 (group.type === 'mo' || group.type === 'mi' ||
    //                     group.type === 'mn')) {
    //                 let child = group.children[0];
    //                 if (child instanceof TextNode && child.text.length > 0) {
    //                     // Overlay with combining character long solidus
    //                     child.text = child.text.slice(0, 1) + "\u0338" +
    //                         child.text.slice(1);
    //                     groups.pop();
    //                 }
    //             }
    //         }
    //     }
    //     groups.push(group);
    //     lastGroup = group;
    // }
    // return groups;
}

// /**
//  * Equivalent to buildExpression, but wraps the elements in an <mrow>
//  * if there's more than one.  Returns a single node instead of an array.
//  */
// pub fn buildExpressionRow(
//     expression:Vec<AnyParseNode>,
//     options: Options,
//     isOrdgroup?: bool,
// )->MathDomNode {
//     return makeRow(buildExpression(expression, options, isOrdgroup));
// };

// /**
//  * Takes a group from the parser and calls the appropriate groupBuilders function
//  * on it to produce a MathML node.
//  */
// pub fn buildGroup(
//     group: ?AnyParseNode,
//     options: Options,
// )->MathNode {
//     if (!group) {
//         return MathNode::new("mrow");
//     }

//     if (groupBuilders[group.type]) {
//         // Call the groupBuilders function
//         // $FlowFixMe
//         let result: MathDomNode = groupBuilders[group.type](group, options);
//         // $FlowFixMe
//         return result;
//     } else {
//         throw new ParseError(
//             "Got group of unknown type: '" + group.type + "'");
//     }
// };

/**
 * Takes a full parse tree and settings and builds a MathML representation of
 * it. In particular, we put the elements from building the parse tree into a
 * <semantics> tag so we can also include that TeX source as an annotation.
 *
 * Note that we actually return a domTree element with a `<math>` inside it so
 * we can do appropriate styling.
 */
pub fn build_math_ml(
    tree: Vec<Box<dyn AnyParseNode>>,
    tex_expression: String,
    options: Options,
    is_display_mode: bool,
    for_mathml_only: bool,
) -> Span {
    let expression = build_expression(tree, options, false);

    // TODO: Make a pass thru the MathML similar to buildHTML.traverseNonSpaceNodes
    // and add spacing nodes. This is necessary only adjacent to math operators
    // like \sin or \lim or to subsup elements that contain math operators.
    // MathML takes care of the other spacing issues.
    // Wrap up the expression in an mrow so it is presented in the semantics
    // tag correctly, unless it's a single <mrow> or <mtable>.
    let wrapper;
    if expression.len() == 1 && ["mrow", "mtable"].contains(&expression[0].get_node_type().as_str())
    {
        wrapper = expression[0].clone();
    } else {
        wrapper = MathNode::new(
            MathNodeType::Mrow,
            expression
                .iter()
                .map(|x| Box::new(x.clone()) as Box<dyn MathDomNode>)
                .collect(),
            vec![],
        );
    }

    // Build a TeX annotation of the source
    let mut annotation = MathNode::new(
        MathNodeType::Annotation,
        vec![Box::new(TextNode::new(tex_expression)) as Box<dyn MathDomNode>],
        vec![],
    );

    annotation.set_attribute("encoding".to_string(), "application/x-tex".to_string());

    let semantics = MathNode::new(
        MathNodeType::Semantics,
        vec![Box::new(wrapper), Box::new(annotation)],
        vec![],
    );

    let mut math = MathNode::new(MathNodeType::Math, vec![Box::new(semantics)], vec![]);
    math.set_attribute(
        "xmlns".to_string(),
        "http://www.w3.org/1998/Math/MathML".to_string(),
    );
    if (is_display_mode) {
        math.set_attribute("display".to_string(), "block".to_string());
    }

    // You can't style <math> nodes, so we wrap the node in a span.
    // NOTE: The span class is not typed to have <math> nodes as children, and
    // we don't want to make the children type more generic since the children
    // of span are expected to have more fields in `buildHtml` contexts.
    let wrapper_class = if for_mathml_only {
        "katex"
    } else {
        "katex-mathml"
    };
    return Span::new(
        vec![wrapper_class.to_string()],
        vec![/*Box::new(math) as Box<dyn HtmlDomNode>*/],
        None,
        CssStyle::default(),
    );
}
