use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::ParseNodeToAny;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::sync::Mutex;
// @flow

fn pad() -> MathNode {
    let mut pad_node = MathNode::new(MathNodeType::Mtd, vec![], vec![]);
    pad_node.set_attribute("width".to_string(), "50%".to_string());
    return pad_node;
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::tag>()
        .unwrap();
    let mut table = MathNode::new(
        MathNodeType::Mtable,
        vec![Box::new(MathNode::new(
            MathNodeType::Mtr,
            vec![
                Box::new(pad()) as Box<dyn MathDomNode>,
                Box::new(MathNode::new(
                    MathNodeType::Mtd,
                    vec![mathML::build_expression_row(
                        group.body.clone(),
                        options.clone(),
                        false,
                    )],
                    vec![],
                )) as Box<dyn MathDomNode>,
                Box::new(pad()) as Box<dyn MathDomNode>,
                Box::new(MathNode::new(
                    MathNodeType::Mtd,
                    vec![mathML::build_expression_row(
                        group.tag.clone(),
                        options,
                        false,
                    )],
                    vec![],
                )) as Box<dyn MathDomNode>,
            ],
            vec![],
        )) as Box<dyn MathDomNode>],
        vec![],
    );
    table.set_attribute("width".to_string(), "100%".to_string());
    return Box::new(table) as Box<dyn MathDomNode>;

    // TODO: Left-aligned tags.
    // Currently, the group and options passed here do not contain
    // enough info to set tag alignment. `leqno` is in Settings but it is
    // not passed to Options. On the HTML side, leqno is
    // set by a CSS class applied in buildTree.js. That would have worked
    // in MathML if browsers supported <mlabeledtr>. Since they don't, we
    // need to rewrite the way this function is called.
}

lazy_static! {
    pub static ref TAG: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);

        FunctionDefSpec {
            def_type: "tag".to_string(),
            names: vec![],
            props,
            handler: |a, b, c| panic!("error"),
            html_builder: None,
            mathml_builder: Some(mathml_builder),
        }
    });
}
