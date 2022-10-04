// "mathord" and "textord" ParseNodes created in Parser.js from symbol Groups in
// src/symbols.js.

use crate::build::mathML::{get_variant, make_text};
use crate::define::functions::{FunctionDefSpec, FunctionPropSpec};
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::types::FontVariant;
use crate::Options::Options;
use crate::{parse_node, AnyParseNode, HtmlDomNode};
use std::collections::HashMap;
use std::sync::Mutex;

pub fn mathord_html_builder(
    _group: Box<dyn AnyParseNode>,
    options: Options,
) -> Box<dyn HtmlDomNode> {
    return Box::new(crate::build::common::make_ord(
        _group,
        options,
        "mathord".to_string(),
    )) as Box<dyn HtmlDomNode>;
}

pub fn mathord_mathml_builder(
    _group: Box<dyn AnyParseNode>,
    options: Options,
) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::mathord>()
        .unwrap();
    let mut node = MathNode::new(
        MathNodeType::Mi,
        vec![Box::new(make_text(
            group.text.clone(),
            group.mode,
            Some(&options.clone()),
        )) as Box<dyn MathDomNode>],
        vec![],
    );

    let variant = get_variant(&_group, &options).unwrap_or(FontVariant::italic);
    let default_variant = DEFAULT_VARIANT.lock().unwrap();
    if &variant.as_str() != default_variant.get(node.get_node_type().as_str()).unwrap() {
        node.set_attribute("mathvariant".to_string(), variant.as_str().to_string());
    }
    return Box::new(node) as Box<dyn MathDomNode>;
}

lazy_static! {
    static ref DEFAULT_VARIANT: Mutex<HashMap<&'static str, &'static str>> =
        Mutex::new({ HashMap::from([("mi", "italic"), ("mn", "normal"), ("mtext", "normal"),]) });
    pub static ref MATHORD: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();

        FunctionDefSpec {
            def_type: "mathord".to_string(),
            names: vec![],
            props,
            handler: |a, b, c| panic!("error"),
            html_builder: Some(mathord_html_builder),
            mathml_builder: Some(mathord_mathml_builder),
        }
    });
}

// defineFunctionBuilders({
// type: "textord",
// htmlBuilder(group, options) {
// return buildCommon.makeOrd(group, options, "textord");
// },
// mathmlBuilder(group: ParseNode<"textord">, options) {
// const text = mml.makeText(group.text, group.mode, options);
// const variant = mml.getVariant(group, options) || "normal";
//
// let node;
// if (group.mode === 'text') {
// node = new mathMLTree.MathNode("mtext", [text]);
// } else if (/[0-9]/.test(group.text)) {
// node = new mathMLTree.MathNode("mn", [text]);
// } else if (group.text === "\\prime") {
// node = new mathMLTree.MathNode("mo", [text]);
// } else {
// node = new mathMLTree.MathNode("mi", [text]);
// }
// if (variant !== defaultVariant[node.type]) {
// node.setAttribute("mathvariant", variant);
// }
//
// return node;
// },
// });
