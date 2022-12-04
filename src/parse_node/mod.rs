use crate::parse_node::types::AnyParseNode;

pub(crate) mod types;


// Some of these have a "-token" suffix since these are also used as `ParseNode`
// types for raw text tokens, and we want to avoid conflicts with higher-level
// `ParseNode` types. These `ParseNode`s are constructed within `Parser` by
// looking up the `symbols` map.
const ATOMS: [&'static str; 6] = ["bin", "close", "inner", "open", "punct", "rel"];
const NON_ATOMS: [&str; 5] = ["accent-token", "mathord", "op-token", "spacing", "textord"];

/**
 * Returns the node more strictly typed iff it is of the given type. Otherwise,
 * returns null.
 */
pub fn check_symbol_node_type(node: &Box<dyn AnyParseNode>) -> bool {
    node.get_type() == "atom" || NON_ATOMS.contains(&node.get_type())
}
