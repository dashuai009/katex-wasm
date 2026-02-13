use crate::{
    dom_tree::{css_style::CssStyle,  span::Span},
    parse_node,
    parse_node::types::AnyParseNode,
    tree::{HtmlDomNode, VirtualNode},
    units::make_em,
    Options::Options,
};
use std::any::{Any, TypeId};
use std::str::FromStr;
/**
 * This file does the main work of building a domTree structure from a parse
 * tree. The entry point is the `buildHTML` function, which takes a parse tree.
 * Then, the buildExpression, build_group, and various groupBuilders functions
 * are called, to produce a final HTML tree.
 */
use std::vec;

use super::common::make_span;
use crate::define::functions::public::_HTML_GROUP_BUILDERS;
use crate::dom_tree::anchor::Anchor;
use crate::parse_node::types::ParseNodeToAny;

// Binary atoms (first class `mbin`) change into ordinary atoms (`mord`)
// depending on their surroundings. See TeXbook pg. 442-446, Rules 5 and 6,
// and the text before Rule 19.
const BIN_LEFT_CANCELLER: [&'static str; 6] =
    ["leftmost", "mbin", "mopen", "mrel", "mop", "mpunct"];
const BIN_RIGHT_CANCELLER: [&'static str; 4] = ["rightmost", "mrel", "mclose", "mpunct"];

#[derive(PartialEq)]
pub enum Side {
    Left,
    Right,
}
// type Side = "left" | "right";

pub enum DomType {
    mord,
    mop,
    mbin,
    mrel,
    mopen,
    mclose,
    mpunct,
    minner,
}
impl DomType {
    pub fn as_str(&self) -> &str {
        match self {
            DomType::mord => "mord",
            DomType::mop => "mop",
            DomType::mbin => "mbin",
            DomType::mrel => "mrel",
            DomType::mopen => "mopen",
            DomType::mclose => "mclose",
            DomType::mpunct => "mpunct",
            DomType::minner => "minner",
        }
    }
}
impl FromStr for DomType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mord" => Ok(DomType::mord),
            "mop" => Ok(DomType::mop),
            "mbin" => Ok(DomType::mbin),
            "mrel" => Ok(DomType::mrel),
            "mopen" => Ok(DomType::mopen),
            "mclose" => Ok(DomType::mclose),
            "mpunct" => Ok(DomType::mpunct),
            "minner" => Ok(DomType::minner),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq)]
pub enum IsRealGroup {
    T,
    F,
    Root,
}

/// Take a list of nodes, build them in order, and return a list of the built
/// nodes. documentFragments are flattened into their contents, so the
/// returned list contains no fragments. `is_real_group` is true if `expression`
/// is a real group (no atoms will be added on either side), as opposed to
/// a partial group (e.g. one created by \color). `surrounding` is an array
/// consisting type of nodes that will be added to the left and right.
pub fn build_expression(
    expression: Vec<Box<dyn AnyParseNode>>,
    options: Options,
    is_real_group: IsRealGroup,
    surrounding: (Option<DomType>, Option<DomType>),
) -> Vec<Box<dyn HtmlDomNode>> {
    // return vec![];
    // Parse expressions into `groups`.
    println!("build_expression groups = {:#?}", expression);
    let mut groups = vec![];
    for expr in expression.iter() {
        let mut output = build_group(Some(expr.clone()), options.clone(), None);
        if let Some(k) = output.as_any().downcast_ref::<crate::dom_tree::document_fragment::DocumentFragment>() {
            groups.append(&mut k.clone().get_mut_children().unwrap().clone());
        } else {
            groups.push(output);
        }
    }
    //println!("build_expression groups = {:#?}", groups);

    // Combine consecutive domTree.symbolNodes into a single symbolNode.
    super::common::try_combine_chars(&mut groups);

    // If `expression` is a partial group, let the parent handle spacings
    // to avoid processing groups multiple times.
    if is_real_group == IsRealGroup::F {
        return groups.clone();
    }

    let mut glue_options = options.clone();
    if expression.len() == 1 {
        let node = &expression[0];
        if let Some(s) = node.as_any().downcast_ref::<parse_node::types::sizing>() {
            glue_options = options.having_size(s.size as f64);
        } else if let Some(s) = node.as_any().downcast_ref::<parse_node::types::styling>() {
            glue_options = options.having_style(&s.style.as_style());
        }
    }

    // Dummy spans for determining spacings between surrounding atoms.
    // If `expression` has no atoms on the left or right, class "leftmost"
    // or "rightmost", respectively, is used to indicate it.
    let dummy_prev = make_span(
        vec![if let Some(s) = surrounding.0 {
            s.as_str().to_string()
        } else {
            "leftmost".to_string()
        }],
        vec![],
        Some(&options),
        CssStyle::default(),
    );
    let dummy_next = make_span(
        vec![if let Some(s) = surrounding.1 {
            s.as_str().to_string()
        } else {
            "rightmost".to_string()
        }],
        vec![],
        Some(&options),
        CssStyle::default(),
    );

    // TODO: These code assumes that a node's math class is the first element
    // of its `classes` array. A later cleanup should ensure this, for
    // instance by changing the signature of `make_span`.

    // Before determining what spaces to insert, perform bin cancellation.
    // Binary operators change to ordinary symbols in some contexts.
    let xx = |node: &mut Box<dyn HtmlDomNode>,
              prev: &mut Box<dyn HtmlDomNode>|
     -> Option<Box<dyn HtmlDomNode>> {
        if let Some(prev_type) = prev.get_classes().get(0){
            if let Some(_type) = node.get_classes().get(0){
                if prev_type == "mbin" && BIN_RIGHT_CANCELLER.contains(&_type.as_str()) {
                    prev.get_mut_classes()[0] = "mord".to_string();
                } else if _type == "mbin" && BIN_LEFT_CANCELLER.contains(&prev_type.as_str()) {
                    node.get_mut_classes()[0] = "mord".to_string();
                }
            }
        }
        return None;
    };
    let is_root = (is_real_group == IsRealGroup::Root);
    traverse_non_space_nodes(
        &mut groups,
        &xx,
        &mut TraversePrev {
            node: Box::new(dummy_prev.clone()) as Box<dyn HtmlDomNode>,
            insert_after: None,
        },
        Some(Box::new(dummy_next.clone()) as Box<dyn HtmlDomNode>),
        is_root,
    );

    traverse_non_space_nodes(
        &mut groups,
        &Box::new(
            |node: &mut Box<dyn HtmlDomNode>,
             prev: &mut Box<dyn HtmlDomNode>|
             -> Option<Box<dyn HtmlDomNode>> {
                let prev_type = get_type_of_dom_tree(prev, None);
                let _type = get_type_of_dom_tree(node, None);

                // 'mtight' indicates that the node is script or scriptscript style.
                //console.log(node,prev_type,type);
                let space = if prev_type.is_some() && _type.is_some() {
                    if node.get_classes().contains(&"mtight".to_string()) {
                        crate::spacingData::get_tightSpacings(
                            prev_type.unwrap().as_str().to_string(),
                            _type.unwrap().as_str().to_string(),
                        )
                    } else {
                        crate::spacingData::get_spacings(
                            prev_type.unwrap().as_str().to_string(),
                            _type.unwrap().as_str().to_string(),
                        )
                    }
                } else {
                    None
                };
                if let Some(s) = space {
                    // Insert glue (spacing) after the `prev`.
                    return Some(Box::new(super::common::make_glue(
                        &s,
                        &glue_options.clone(),
                    )) as Box<dyn HtmlDomNode>);
                }
                return None;
            },
        ),
        &mut TraversePrev {
            node: Box::new(dummy_prev) as Box<dyn HtmlDomNode>,
            insert_after: None,
        },
        Some(Box::new(dummy_next) as Box<dyn HtmlDomNode>),
        is_root,
    );

    // println!("build_expression groups = {:#?}", groups);
    return groups;
}

// type InsertAfter =
struct TraversePrev {
    node: Box<dyn HtmlDomNode>,
    insert_after: Option<Box<dyn FnMut(Box<dyn HtmlDomNode>) -> ()>>,
}
// Depth-first traverse non-space `nodes`, calling `callback` with the current and
// previous node as arguments, optionally returning a node to insert after the
// previous node. `prev` is an object with the previous node and `insertAfter`
// function to insert after it. `next` is a node that will be added to the right.
// // Used for bin cancellation and inserting spacings.
fn traverse_non_space_nodes(
    mut nodes: &mut Vec<Box<dyn HtmlDomNode>>,
    callback: &dyn Fn(
        &mut Box<dyn HtmlDomNode>,
        &mut Box<dyn HtmlDomNode>,
    ) -> Option<Box<dyn HtmlDomNode>>,

    mut prev: &mut TraversePrev,
    next: Option<Box<dyn HtmlDomNode>>,
    is_root: bool,
) {
    if next.is_some() {
        // temporarily append the right node, if exists
        nodes.push(next.as_ref().unwrap().clone());
    }
    // println!("travers = {:#?}", nodes);
    let mut i = 0;
    let mut insert_after_pos = None;
    while i < nodes.len() {
        // println!("i={} len = {}", i, nodes.len());
        if check_partial_group(&nodes[i]){
            // println!("check");
            if let Some(partial_group) = nodes[i].get_mut_children() {
                // Recursive DFS
                traverse_non_space_nodes(partial_group, callback, &mut prev, None, is_root.clone());
                i += 1;
                continue;
            }

        }

        // Ignore explicit spaces (e.g., \;, \,) when determining what implicit
        // spacing should go between atoms of different classes
        let nonspace = !nodes[i].has_class(&"mspace".to_string());
        if nonspace {
            if let Some(result) = callback(&mut nodes[i], &mut prev.node) {
                //println!("result = {:#?}", result);
                if insert_after_pos.is_some() {
                    nodes.insert(insert_after_pos.unwrap() + 1, result);
                    i+=1;
                } else {
                    // insert at front
                    nodes.insert(0, result);
                    i += 1;
                }
            }
            prev.node = nodes[i].clone();
        } else if is_root && nodes[i].has_class(&"newline".to_string()) {
            prev.node = Box::new(make_span(
                vec!["leftmost".to_string()],
                vec![],
                None,
                CssStyle::default(),
            )) as Box<dyn HtmlDomNode>;
            // treat like beginning of line
        }
        insert_after_pos = Some(i);
        i += 1;
    }
    if next.is_some() {
        nodes.pop();
    }
}

// Check if given node is a partial group, i.e., does not affect spacing around.
fn check_partial_group(node: &Box<dyn HtmlDomNode>) -> bool {
    let t = node.as_any().type_id();
    return t == TypeId::of::<crate::dom_tree::document_fragment::DocumentFragment>()
        || t == TypeId::of::<Anchor>()
        || (t == TypeId::of::<Span>() && node.has_class(&"enclosing".to_string()));
}

// Return the outermost node of a domTree.
fn get_outermost_node(node: &Box<dyn HtmlDomNode>, side: Side) -> &Box<dyn HtmlDomNode> {
    if check_partial_group(node){
        if let Some(children) = node.get_children() {
            if children.len() > 0 {
                if side == Side::Right {
                    let x = { children.len().clone() - 1 };
                    return get_outermost_node(&children[x], Side::Right);
                } else if side == Side::Left {
                    return get_outermost_node(&children[0], Side::Left);
                }
            }
        }
    }

    return node;
}

// Return math atom class (mclass) of a domTree.
// If `side` is given, it will get the type of the outermost node at given side.
pub fn get_type_of_dom_tree(node: &Box<dyn HtmlDomNode>, side: Option<Side>) -> Option<DomType> {
    let _node = if let Some(s) = side {
        get_outermost_node(node, s)
    } else {
        node
    };

    // This makes a lot of assumptions as to where the type of atom
    // appears.  We should do a better job of enforcing this.
    return DomType::from_str(&*_node.get_classes().get(0).unwrap_or(&"".to_string())).ok();
}

pub fn make_null_delimiter(
    options: &Options,
    classes: Vec<String>,
)->Span {
    let more_classes = [classes,vec!["nulldelimiter".to_string()],options.base_sizing_classes()].concat();
    return make_span(more_classes, vec![], None, Default::default());
}

/**
 * build_group is the function that takes a group and calls the correct groupType
 * function for it. It also handles the interaction of size and style changes
 * between parents and children.
 */
pub fn build_group(
    group: Option<Box<dyn AnyParseNode>>,
    options: Options,
    base_options: Option<Options>,
) -> Box<dyn HtmlDomNode> {
    if let Some(g) = group {
        let t = g.get_type();
        let mut group_node = {
            let _builders = _HTML_GROUP_BUILDERS.read().unwrap();
            if let Some(f) = _builders.get(t) {
                f(g, options.clone())
            } else {
                panic!("Got group of unknown type: '{}'", t)
            }
        };

        // If the size changed between the parent and the current group, account
        // for that size difference.
        if let Some(base) = base_options {
            if base.size != options.size {
                group_node = Box::new(make_span(
                    options.sizing_classes(&base),
                    vec![group_node],
                    Some(&options),
                    CssStyle::default(),
                )) as Box<dyn HtmlDomNode>;

                let multiplier = options.sizeMultiplier / base.sizeMultiplier;

                group_node.set_height(group_node.get_height() * multiplier);
                group_node.set_depth(group_node.get_depth() * multiplier);
            }
        }
        return group_node;
    } else {
        return Box::new(make_span(
            vec![],
            vec![],
            Some(&options),
            CssStyle::default(),
        ));
    }
}

/**
 * Combine an array of HTML DOM nodes (e.g., the output of `buildExpression`)
 * into an unbreakable HTML node of class .base, with proper struts to
 * guarantee correct vertical extent.  `buildHTML` calls this repeatedly to
 * make up the entire expression as a sequence of unbreakable units.
 */
fn buildHTMLUnbreakable(children: Vec<Box<dyn HtmlDomNode>>, options: Option<&Options>) -> Span {
    // Compute height and depth of this chunk.
    let mut body = make_span(
        vec!["base".to_string()],
        children,
        options.clone(),
        CssStyle::new(),
    );

    // Add strut, which ensures that the top of the HTML element falls at
    // the height of the expression, and the bottom of the HTML element
    // falls at the depth of the expression.
    let mut strut = make_span(vec!["strut".to_string()], vec![], None, CssStyle::new());
    strut.get_mut_style().height = Some(make_em(body.get_height() + body.get_depth()));
    if body.get_depth() > 0.0 {
        strut.get_mut_style().vertical_align = Some(make_em(-body.get_depth()));
    }
    // body.get_mut_children().unshift(strut);

    return body;
}

/**
 * Take an entire parse tree, and build it into an appropriate set of HTML
 * nodes.
 */
pub fn build_html(mut tree: Vec<Box<dyn AnyParseNode>>, options: Options) -> Span {
    // Strip off outer tag wrapper for processing below.
    let mut tag = None;
    if tree.len() == 1 && tree[0].get_type() == "tag" {
        if let Some(t) = tree[0].as_any().downcast_ref::<parse_node::types::tag>() {
            tag = Some(t.tag.clone());
            tree = t.body.clone();
        }
    }

    // Build the expression contained in the tree
    let mut expression = build_expression(tree, options.clone(), IsRealGroup::Root, (None, None));

    let mut eqn_num = None;
    if expression.len() == 2 && expression[1].has_class(&"tag".to_string()) {
        // An environment with automatic equation numbers, e.g. {gather}.
        eqn_num = expression.pop();
    }

    let mut children: Vec<Box<dyn HtmlDomNode>> = vec![];

    // Create one base node for each chunk between potential line breaks.
    // The TeXBook [p.173] says "A formula will be broken only after a
    // relation symbol like $=$ or $<$ or $\rightarrow$, or after a binary
    // operation symbol like $+$ or $-$ or $\times$, where the relation or
    // binary operation is on the ``outer level'' of the formula (i.e., not
    // enclosed in {...} and not part of an \over letruction)."

    let mut parts = vec![];
    let mut i = 0usize;
    while i < expression.len() {
        parts.push(expression[i].clone());
        if expression[i].has_class(&"mbin".to_string())
            || expression[i].has_class(&"mrel".to_string())
            || expression[i].has_class(&"allowbreak".to_string())
        {
            // Put any post-operator glue on same line as operator.
            // Watch for \nobreak along the way, and stop at \newline.
            let mut nobreak = false;
            while i < expression.len() - 1
                && expression[i + 1].has_class(&"mspace".to_string())
                && !expression[i + 1].has_class(&"newline".to_string())
            {
                i += 1;
                parts.push(expression[i].clone());
                if expression[i].has_class(&"nobreak".to_string()) {
                    nobreak = true;
                }
            }
            // Don't allow break if \nobreak among the post-operator glue.
            if (!nobreak) {
                children
                    .push(Box::new(buildHTMLUnbreakable(parts, Some(&options)))
                        as Box<dyn HtmlDomNode>);
                parts = vec![];
            }
        } else if (expression[i].has_class(&"newline".to_string())) {
            // Write the line except the newline
            parts.pop();
            if (parts.len() > 0) {
                children
                    .push(Box::new(buildHTMLUnbreakable(parts, Some(&options)))
                        as Box<dyn HtmlDomNode>);
                parts = vec![];
            }
            // Put the newline at the top level
            children.push(expression[i].clone());
        }

        i += 1;
    }
    if (parts.len() > 0) {
        children
            .push(Box::new(buildHTMLUnbreakable(parts, Some(&options))) as Box<dyn HtmlDomNode>);
    }

    // Now, if there was a tag, build it too and append it as a final child.
    let mut tag_child = None;
    if tag.is_some() {
        let mut _tag_child = Box::new(buildHTMLUnbreakable(
            build_expression(tag.unwrap(), options, IsRealGroup::T, (None, None)),
            None,
        )) as Box<dyn HtmlDomNode>;
        _tag_child.set_classes(vec!["tag".to_string()]);
        children.push(_tag_child.clone());
        tag_child = Some(_tag_child.clone());
    } else if eqn_num.is_some() {
        children.push(eqn_num.unwrap());
    }

    let mut html_node = make_span(
        vec!["katex-html".to_string()],
        children,
        None,
        CssStyle::new(),
    );
    html_node.set_attribute("aria-hidden".parse().unwrap(), "true".to_string());

    // Adjust the strut of the tag to be the maximum height of all children
    // (the height of the enclosing htmlNode) for proper vertical alignment.
    if let Some(mut t) = tag_child {
        let mut strut = &mut t.get_mut_children().unwrap()[0];
        strut.get_mut_style().height =
            Some(make_em(html_node.get_height() + html_node.get_depth()));
        if html_node.get_depth() > 0.0 {
            strut.get_mut_style().vertical_align = Some(make_em(-html_node.get_depth()));
        }
    }

    return html_node;
}
