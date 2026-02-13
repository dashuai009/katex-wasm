use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::dom_tree::symbol_node::SymbolNode;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::ParseNodeToAny;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::Parser::Parser;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::any::{Any, TypeId};
use std::sync::Mutex;

// NOTE: Unlike most `html_builder`s, this one handles not only
// "operatorname".to_string(), but also  "supsub" since \operatorname* can
// affect super/subscripting.
fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    // Operators are handled in the TeXbook pg. 443-444, rule 13(a).
    let mut supGroup = None;
    let mut subGroup = None;
    let mut hasLimits = false;
    let group;
    if let Some(grp) = _group.as_any().downcast_ref::<parse_node::types::supsub>() {
        // If we have limits, supsub will pass us its group to handle. Pull
        // out the superscript and subscript and set the group to the op in
        // its base.
        supGroup = grp.sup.clone();
        subGroup = grp.sub.clone();
        group = grp
            .base
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<parse_node::types::operatorname>()
            .unwrap();
        hasLimits = true;
    } else {
        group = _group
            .as_any()
            .downcast_ref::<parse_node::types::operatorname>()
            .unwrap();
    }

    let base;
    if group.body.len() > 0 {
        let body = group
            .body
            .iter()
            .map(|child| {
                return if let Some(c) = child.as_any().downcast_ref::<parse_node::types::atom>() {
                    let res = parse_node::types::textord {
                        mode: c.mode,
                        loc: None,
                        text: c.text.clone(),
                    };
                    Box::new(res) as Box<dyn AnyParseNode>
                } else {
                    child.clone()
                }
            })
            .collect();

        // Consolidate function names into symbol characters.
        let mut expression = HTML::build_expression(
            body,
            options.with_font("mathrm".to_string()),
            IsRealGroup::T,
            (None, None),
        );

        for child in expression.iter_mut() {
            if let Some(c) = child.as_mut_any().downcast_mut::<SymbolNode>() {
                // Per amsopn package,
                // change minus to hyphen and \ast to asterisk
                c.text = c.text.replace("\u{2212}", "-");
                c.text = c.text.replace("\u{2217}", "*");
            }
        }
        base = common::make_span(
            vec!["mop".to_string()],
            expression,
            Some(&options),
            Default::default(),
        );
    } else {
        base = common::make_span(
            vec!["mop".to_string()],
            vec![],
            Some(&options),
            Default::default(),
        );
    }

    let _base = Box::new(base) as Box<dyn HtmlDomNode>;

    if hasLimits {
        let res = super::assembleSupSub::assemble_sup_sub(
            &_base,
            &supGroup,
            &subGroup,
            &options,
            &options.get_style(),
            0.0,
            0.0,
        );
        return Box::new(res) as Box<dyn HtmlDomNode>;
    } else {
        return _base;
    }
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined")
    // The steps taken here are similar to the html version.
    // let expression = mathML::build_expression(
    //     group.body, options.withFont("mathrm".to_string()));
    //
    // // Is expression a string or has it something like a fraction?
    // let isAllString = true;  // default
    // for (let i = 0; i < expression.length; i+ +) {
    //     let node = expression[i];
    //     if (node
    //     instanceof
    //     mathMLTree.SpaceNode) {
    //         // Do nothing
    //     } else if (node
    //     instanceof
    //     mathMLTree.MathNode) {
    //         switch(node. type ) {
    //             case
    //             "mi":
    //                 case
    //             "mn":
    //                 case
    //             "ms":
    //                 case
    //             "mspace":
    //                 case
    //             "mtext":
    //             break;  // Do nothing yet.
    //             case
    //             "mo": {
    //                 let child = node.children[0];
    //                 if (node.children.length == 1 &&
    //                     child
    //                 instanceof
    //                 mathMLTree.TextNode) {
    //                     child.text =
    //                         child.text.replace(/ \u2212 /, "-".to_string())
    //                             .replace(/ \u2217 /, "*".to_string());
    //                 } else {
    //                     isAllString = false;
    //                 }
    //                 break;
    //             }
    //             default:
    //                 isAllString = false;
    //         }
    //     } else {
    //         isAllString = false;
    //     }
    // }
    //
    // if (isAllString) {
    //     // Write a single TextNode instead of multiple nested tags.
    //     let word = expression.map(node => node.toText()).join("".to_string());
    //     expression = [new
    //     mathMLTree.TextNode(word)];
    // }
    //
    // let identifier = MathNode::new("mi".to_string(), expression);
    // identifier.set_attribute("mathvariant".to_string(), "normal".to_string());
    //
    // // \u2061 is the same as &ApplyFunction;
    // // ref: https://www.w3schools.com/charsets/ref_html_entities_a.asp
    // let operator = MathNode::new("mo".to_string(),
    //                              [mml.makeText("\u2061".to_string(), "text".to_string())]);
    //
    // if (group.parentIsSupSub) {
    //     return MathNode::new("mrow".to_string(), [identifier, operator]);
    // } else {
    //     return mathMLTree.newDocumentFragment([identifier, operator]);
    // }
}

// \operatorname
// amsopn.dtx: \mathop{#1\kern\z@\operator@font#3}\newmcodes@
fn handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let ctx = context.borrow();
    let body = args[0].clone();
    let res = parse_node::types::operatorname {
        mode: ctx.parser.mode,
        loc: None,
        body: ord_argument(&body),
        always_handle_sup_sub: (ctx.func_name == "\\operatornamewithlimits".to_string()),
        limits: false,
        parent_is_sup_sub: false,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

lazy_static! {
    pub static ref ORDGROUP: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);

        FunctionDefSpec {
            def_type: "operatorname".to_string(),
            names: vec![
                "\\operatorname@".to_string(),
                "\\operatornamewithlimits".to_string(),
            ],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
// defineMacro("\\operatorname".to_string(),
// "\\@ifstar\\operatornamewithlimits\\operatorname@".to_string());
