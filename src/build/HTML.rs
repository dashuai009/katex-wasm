// @flow
/**
 * This file does the main work of building a domTree structure from a parse
 * tree. The entry point is the `buildHTML` function, which takes a parse tree.
 * Then, the buildExpression, build_group, and various groupBuilders functions
 * are called, to produce a final HTML tree.
 */

// import ParseError from "./ParseError";
// import Style from "./Style";
// import buildCommon from "./buildCommon";
// import {Span, Anchor} from "./domTree";
// import utils from "./utils";
// import {makeEm} from "./units";
// import {get_spacings, get_tightSpacings} from "./katex-wasm";
// import {_htmlGroupBuilders as groupBuilders} from "./defineFunction";
// import {DocumentFragment} from "./tree";

// import type Options from "./Options";
// import type {AnyParseNode} from "./parseNode";
// import type {HtmlDomNode, DomSpan} from "./domTree";

use std::vec;

use crate::{parse_node::types::AnyParseNode, tree::{HtmlDomNode, DocumentFragment}, Options::Options, dom_tree::css_style::CssStyle};

use super::common::make_span;
use crate::define::functions::public::_HTML_GROUP_BUILDERS;

// Binary atoms (first class `mbin`) change into ordinary atoms (`mord`)
// depending on their surroundings. See TeXbook pg. 442-446, Rules 5 and 6,
// and the text before Rule 19.
const BIN_LEFT_CANCELLER:[&'static str;6] = ["leftmost", "mbin", "mopen", "mrel", "mop", "mpunct"];
const BIN_RIGHT_CANCELLER:[&'static str;4] = ["rightmost", "mrel", "mclose", "mpunct"];

// let styleMap = {
//     "display": Style.DISPLAY,
//     "text": Style.TEXT,
//     "script": Style.SCRIPT,
//     "scriptscript": Style.SCRIPTSCRIPT,
// };

enum Side{
    Left,
    Right
}
// type Side = "left" | "right";

enum DomType {
    mord,
    mop,
    mbin,
    mrel,
    mopen,
    mclose,
    mpunct,
    minner,
}
enum IsRealGroup{
    T,
    F,
    Root
}
/**
 * Take a list of nodes, build them in order, and return a list of the built
 * nodes. documentFragments are flattened into their contents, so the
 * returned list contains no fragments. `isRealGroup` is true if `expression`
 * is a real group (no atoms will be added on either side), as opposed to
 * a partial group (e.g. one created by \color). `surrounding` is an array
 * consisting type of nodes that will be added to the left and right.
 */
 
pub fn buildExpression(
    expression: Vec<dyn AnyParseNode>,
    options: Options,
    isRealGroup: IsRealGroup,
    surrounding: (DomType, DomType),
)->Vec<Box<dyn HtmlDomNode>> {
    // Parse expressions into `groups`.
    let mut groups :Vec<Box<dyn HtmlDomNode> > = vec![];
    for expr in expression.iter(){
        let output = build_group(expr, options,None);
        if let Some(k)= (output as &dyn Any).downcast_ref::< DocumentFragment>{
            groups.append(k.children);
        }else {
            groups.push(output);
        }
    }

    // Combine consecutive domTree.symbolNodes into a single symbolNode.
    buildCommon.tryCombineChars(groups);

    // If `expression` is a partial group, let the parent handle spacings
    // to avoid processing groups multiple times.
    if (isRealGroup == IsRealGroup::F) {
        return groups;
    }

    let glueOptions = options;
    if (expression.length() == 1) {
        let node = expression[0];
        if (node.get_type() == "sizing") {
            glueOptions = options.havingSize(node.size);
        } else if (node.get_type() == "styling") {
            glueOptions = options.havingStyle(styleMap[node.style]);
        }
    }

    // Dummy spans for determining spacings between surrounding atoms.
    // If `expression` has no atoms on the left or right, class "leftmost"
    // or "rightmost", respectively, is used to indicate it.
    let dummyPrev = make_span([surrounding[0] || "leftmost"], [], options);
    let dummyNext = make_span([surrounding[1] || "rightmost"], [], options);

    // TODO: These code assumes that a node's math class is the first element
    // of its `classes` array. A later cleanup should ensure this, for
    // instance by changing the signature of `make_span`.

    // Before determining what spaces to insert, perform bin cancellation.
    // Binary operators change to ordinary symbols in some contexts.
    let isRoot = (isRealGroup == IsRealGroup::Root);
    traverseNonSpaceNodes(groups, (node, prev) => {
        let prevType = prev.classes[0];
        let _type = node.classes[0];
        if (prevType === "mbin" && utils.contains(binRightCanceller, type)) {
            prev.classes[0] = "mord";
        } else if (type === "mbin" && utils.contains(binLeftCanceller, prevType)) {
            node.classes[0] = "mord";
        }
    }, {node: dummyPrev}, dummyNext, isRoot);

    traverseNonSpaceNodes(groups, (node, prev) => {
        let prevType = getTypeOfDomTree(prev);
        let type = getTypeOfDomTree(node);

        // 'mtight' indicates that the node is script or scriptscript style.
        //console.log(node,prevType,type);
        let space = prevType && type ? (node.hasClass("mtight")
            ? get_tightSpacings(prevType, type)
            : get_spacings(prevType, type)) : null;
        //console.log(space);
        if (space) { // Insert glue (spacing) after the `prev`.
            return buildCommon.makeGlue(space, glueOptions);
        }
    }, {node: dummyPrev}, dummyNext, isRoot);

    return groups;
}


// type InsertAfter = 
struct TraversePrev {
    node: HtmlDomNode,
    insert_after: fn(HtmlDomNode) -> (),
}
// Depth-first traverse non-space `nodes`, calling `callback` with the current and
// previous node as arguments, optionally returning a node to insert after the
// previous node. `prev` is an object with the previous node and `insertAfter`
// function to insert after it. `next` is a node that will be added to the right.
// // Used for bin cancellation and inserting spacings.
pub fn  traverseNonSpaceNodes(
    nodes: Vec<Box<dyn HtmlDomNode> >,
    callback: fn(HtmlDomNode, HtmlDomNode) -> HtmlDomNode,
    prev:TraversePrev,
    next: Option<HtmlDomNode>,
    is_root: bool,
) {
    if (next) { // temporarily append the right node, if exists
        nodes.push(next);
    }
    let i = 0;
    for (; i < nodes.length; i++) {
        let node = nodes[i];
        let partialGroup = checkPartialGroup(node);
        if (partialGroup) { // Recursive DFS
            // $FlowFixMe: make nodes a $ReadOnlyArray by returning a new array
            traverseNonSpaceNodes(partialGroup.children,
                callback, prev, null, isRoot);
            continue;
        }

        // Ignore explicit spaces (e.g., \;, \,) when determining what implicit
        // spacing should go between atoms of different classes
        let nonspace = !node.hasClass("mspace");
        if (nonspace) {
            let result = callback(node, prev.node);
            if (result) {
                if (prev.insertAfter) {
                    prev.insertAfter(result);
                } else { // insert at front
                    nodes.unshift(result);
                    i++;
                }
            }
        }

        if (nonspace) {
            prev.node = node;
        } else if (isRoot && node.hasClass("newline")) {
            prev.node = make_span(["leftmost"]); // treat like beginning of line
        }
        prev.insertAfter = (index => n => {
            nodes.splice(index + 1, 0, n);
            i++;
        })(i);
    }
    if (next) {
        nodes.pop();
    }
}

// Check if given node is a partial group, i.e., does not affect spacing around.
let checkPartialGroup = function(
    node: HtmlDomNode,
): ?(DocumentFragment<HtmlDomNode> | Anchor | DomSpan) {
    if (node instanceof DocumentFragment || node instanceof Anchor
        || (node instanceof Span && node.hasClass("enclosing"))) {
        return node;
    }
    return null;
};

// Return the outermost node of a domTree.
// let getOutermostNode = function(
//     node: HtmlDomNode,
//     side: Side,
// ): HtmlDomNode {
//     let partialGroup = checkPartialGroup(node);
//     if (partialGroup) {
//         let children = partialGroup.children;
//         if (children.length) {
//             if (side === "right") {
//                 return getOutermostNode(children[children.length - 1], "right");
//             } else if (side === "left") {
//                 return getOutermostNode(children[0], "left");
//             }
//         }
//     }
//     return node;
// };

// Return math atom class (mclass) of a domTree.
// If `side` is given, it will get the type of the outermost node at given side.
// export let getTypeOfDomTree = function(
//     node: ?HtmlDomNode,
//     side: ?Side,
// ): ?DomType {
//     if (!node) {
//         return null;
//     }
//     if (side) {
//         node = getOutermostNode(node, side);
//     }
//     // This makes a lot of assumptions as to where the type of atom
//     // appears.  We should do a better job of enforcing this.
//     return DomEnum[node.classes[0]] || null;
// };

// export let makeNullDelimiter = function(
//     options: Options,
//     classes: string[],
// ): DomSpan {
//     let moreClasses = ["nulldelimiter"].concat(options.baseSizingClasses());
//     return make_span(classes.concat(moreClasses));
// };

/** 
 * build_group is the function that takes a group and calls the correct groupType
 * function for it. It also handles the interaction of size and style changes
 * between parents and children.
 */
pub fn build_group(
    group: Option<Box<dyn AnyParseNode>>,
    options: Options,
    base_options: Option<Options>,
)-> Box<dyn HtmlDomNode> {
    if let Some(g) = group{
        let _builders = _HTML_GROUP_BUILDERS.lock().unwrap();
        let group_node: HtmlDomNode = _builders.get(g.get_type()).unwrap()(g, options);

        // If the size changed between the parent and the current group, account
        // for that size difference.
        if (baseOptions && options.size != baseOptions.size) {
            group_node = make_span(options.sizingClasses(baseOptions),
                vec![group_node], options,CssStyle::default());

            let multiplier =
                options.sizeMultiplier / baseOptions.sizeMultiplier;

            group_node.height *= multiplier;
            group_node.depth *= multiplier;
        }

        return group_node;
    }else{
        return make_span(vec![],vec![],options,CssStyle::default());
    }
}

// /**
//  * Combine an array of HTML DOM nodes (e.g., the output of `buildExpression`)
//  * into an unbreakable HTML node of class .base, with proper struts to
//  * guarantee correct vertical extent.  `buildHTML` calls this repeatedly to
//  * make up the entire expression as a sequence of unbreakable units.
//  */
// function buildHTMLUnbreakable(children, options) {
//     // Compute height and depth of this chunk.
//     let body = make_span(["base"], children, options);

//     // Add strut, which ensures that the top of the HTML element falls at
//     // the height of the expression, and the bottom of the HTML element
//     // falls at the depth of the expression.
//     let strut = make_span(["strut"]);
//     strut.style.height = makeEm(body.height + body.depth);
//     if (body.depth) {
//         strut.style.verticalAlign = makeEm(-body.depth);
//     }
//     body.children.unshift(strut);

//     return body;
// }

// /**
//  * Take an entire parse tree, and build it into an appropriate set of HTML
//  * nodes.
//  */
// export default function buildHTML(tree: AnyParseNode[], options: Options): DomSpan {
//     // Strip off outer tag wrapper for processing below.
//     let tag = null;
//     if (tree.length === 1 && tree[0].type === "tag") {
//         tag = tree[0].tag;
//         tree = tree[0].body;
//     }

//     // Build the expression contained in the tree
//     let expression = buildExpression(tree, options, "root");

//     let eqnNum;
//     if (expression.length === 2 && expression[1].hasClass("tag")) {
//         // An environment with automatic equation numbers, e.g. {gather}.
//         eqnNum = expression.pop();
//     }

//     let children = [];

//     // Create one base node for each chunk between potential line breaks.
//     // The TeXBook [p.173] says "A formula will be broken only after a
//     // relation symbol like $=$ or $<$ or $\rightarrow$, or after a binary
//     // operation symbol like $+$ or $-$ or $\times$, where the relation or
//     // binary operation is on the ``outer level'' of the formula (i.e., not
//     // enclosed in {...} and not part of an \over letruction)."

//     let parts = [];
//     for (let i = 0; i < expression.length; i++) {
//         parts.push(expression[i]);
//         if (expression[i].hasClass("mbin") ||
//             expression[i].hasClass("mrel") ||
//             expression[i].hasClass("allowbreak")) {
//             // Put any post-operator glue on same line as operator.
//             // Watch for \nobreak along the way, and stop at \newline.
//             let nobreak = false;
//             while (i < expression.length - 1 &&  
//                    expression[i + 1].hasClass("mspace") &&
//                    !expression[i + 1].hasClass("newline")) {
//                 i++;
//                 parts.push(expression[i]);
//                 if (expression[i].hasClass("nobreak")) {
//                     nobreak = true;
//                 }
//             }
//             // Don't allow break if \nobreak among the post-operator glue.
//             if (!nobreak) {
//                 children.push(buildHTMLUnbreakable(parts, options));
//                 parts = [];
//             }
//         } else if (expression[i].hasClass("newline")) {
//             // Write the line except the newline
//             parts.pop();
//             if (parts.length > 0) {
//                 children.push(buildHTMLUnbreakable(parts, options));
//                 parts = [];
//             }
//             // Put the newline at the top level
//             children.push(expression[i]);
//         }
//     }
//     if (parts.length > 0) {
//         children.push(buildHTMLUnbreakable(parts, options));
//     }

//     // Now, if there was a tag, build it too and append it as a final child.
//     let tagChild;
//     if (tag) {
//         tagChild = buildHTMLUnbreakable(
//             buildExpression(tag, options, true)
//         );
//         tagChild.classes = ["tag"];
//         children.push(tagChild);
//     } else if (eqnNum) {
//         children.push(eqnNum);
//     }

//     let htmlNode = make_span(["katex-html"], children);
//     htmlNode.setAttribute("aria-hidden", "true");

//     // Adjust the strut of the tag to be the maximum height of all children
//     // (the height of the enclosing htmlNode) for proper vertical alignment.
//     if (tagChild) {
//         let strut = tagChild.children[0];
//         strut.style.height = makeEm(htmlNode.height + htmlNode.depth);
//         if (htmlNode.depth) {
//             strut.style.verticalAlign = makeEm(-htmlNode.depth);
//         }
//     }

//     return htmlNode;
// }
