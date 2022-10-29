use crate::build::common::make_span;
use crate::build::HTML::IsRealGroup;
use crate::build::{mathML, HTML};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::{Atom, ParseNodeToAny};
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, AnyParseNode, HtmlDomNode};
use std::sync::Mutex;

pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::mclass>()
        .unwrap();
    let elements = HTML::build_expression(
        group.body.clone(),
        options.clone(),
        IsRealGroup::T,
        (None, None),
    );
    return Box::new(make_span(
        vec![group.mclass.clone()],
        elements,
        Some(&options.clone()),
        CssStyle::default(),
    )) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::mclass>()
        .unwrap();
    let inner = mathML::build_expression(group.body.clone(), options, false)
        .into_iter()
        .map(|x| Box::new(x) as Box<dyn MathDomNode>)
        .collect();
    let mut node;
    if (group.mclass == "minner") {
        node = MathNode::new(MathNodeType::Mpadded, inner, vec![]);
    } else if group.mclass == "mord" {
        if group.is_character_box {
            node = inner[0]
                .as_any()
                .downcast_ref::<MathNode>()
                .unwrap()
                .clone();
            node.set_node_type(MathNodeType::Mi);
        } else {
            node = MathNode::new(MathNodeType::Mi, inner, vec![]);
        }
    } else {
        if group.is_character_box {
            node = inner[0]
                .as_any()
                .downcast_ref::<MathNode>()
                .unwrap()
                .clone();
            node.set_node_type(MathNodeType::Mo);
        } else {
            node = MathNode::new(MathNodeType::Mo, inner, vec![]);
        }
        // Set spacing based on what is the most likely adjacent atom type.
        // See TeXbook p170.
        if group.mclass == "mbin" {
            node.set_attribute("lspace".to_string(), "0.22em".to_string()); // medium space
            node.set_attribute("rspace".to_string(), "0.22em".to_string());
        } else if group.mclass == "mpunct" {
            node.set_attribute("lspace".to_string(), "0em".to_string());
            node.set_attribute("rspace".to_string(), "0.17em".to_string()); // thinspace
        } else if group.mclass == "mopen" || group.mclass == "mclose" {
            node.set_attribute("lspace".to_string(), "0em".to_string());
            node.set_attribute("rspace".to_string(), "0em".to_string());
        } else if group.mclass == "minner" {
            node.set_attribute("lspace".to_string(), "0.0556em".to_string()); // 1 mu is the most likely option
            node.set_attribute("width".to_string(), "+0.1111em".to_string());
        }
        // MathML <mo> default space is 5/18 em, so <mrel> needs no action.
        // Ref: https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mo
    }
    return Box::new(node) as Box<dyn MathDomNode>;
}
pub fn mclass_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let body = &args[0];
    return Box::new(parse_node::types::mclass {
        mode: context.parser.mode,
        loc: None,
        mclass: context.func_name[0..5].to_string(), // TODO: don't prefix with 'm'
        body: ord_argument(body),
        is_character_box: is_character_box(body),
    }) as Box<dyn AnyParseNode>;
}
// Math class commands except \mathop
lazy_static! {
    pub static ref MCLASS: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_primitive(true);

        FunctionDefSpec {
            def_type: "mclass".to_string(),
            names: vec![
                "\\mathord".to_string(),
                "\\mathbin".to_string(),
                "\\mathrel".to_string(),
                "\\mathopen".to_string(),
                "\\mathclose".to_string(),
                "\\mathpunct".to_string(),
                "\\mathinner".to_string(),
            ],
            props,
            handler: mclass_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

pub fn binrel_class(arg: &Box<dyn AnyParseNode>) -> String {
    // \binrel@ spacing varies with (bin|rel|ord) of the atom in the argument.
    // (by rendering separately and with {}s before and after, and measuring
    // the change in spacing).  We'll do roughly the same by detecting the
    // atom type directly.
    let _atom = if let Some(ord_group) = arg.as_any().downcast_ref::<parse_node::types::ordgroup>()
    {
        if ord_group.body.len() > 0 {
            &ord_group.body[0]
        } else {
            arg
        }
    } else {
        arg
    };

    return if let Some(atom) = _atom.as_any().downcast_ref::<parse_node::types::atom>() {
        if atom.family == Atom::bin  || atom.family == Atom::rel {
            format!("m{}", atom.family.as_str())
        } else {
            "mord".to_string()
        }
    } else {
        "mord".to_string()
    };
}
//
// // \@binrel{x}{y} renders like y but as mbin/mrel/mord if x is mbin/mrel/mord.
// // This is equivalent to \binrel@{x}\binrel@@{y} in AMSTeX.
// defineFunction({
// type: "mclass",
// names: ["\\ @ binrel"],
// props: {
// numArgs: 2,
// },
// handler({parser}, args) {
// return {
// type: "mclass",
// mode: parser.mode,
// mclass: binrel_class(args[0]),
// body: ordargument(args[1]),
// isCharacterBox: utils.isCharacterBox(args[1]),
// };
// },
// });
//
// // Build a relation or stacked op by placing one symbol on top of another
// defineFunction({
// type: "mclass",
// names: ["\\stackrel", "\\overset", "\\underset"],
// props: {
// numArgs: 2,
// },
// handler({parser, funcName}, args) {
// const baseArg = args[1];
// const shiftedArg = args[0];
//
// let mclass;
// if (funcName !== "\\stackrel") {
// // LaTeX applies \binrel spacing to \overset and \underset.
// mclass = binrel_class(baseArg);
// } else {
// mclass = "mrel";  // for \stackrel
// }
//
// const baseOp = {
// type: "op",
// mode: baseArg.mode,
// limits: true,
// alwaysHandleSupSub: true,
// parentIsSupSub: false,
// symbol: false,
// suppressBaseShift: funcName !== "\\stackrel",
// body: ordargument(baseArg),
// };
//
// const supsub = {
// type: "supsub",
// mode: shiftedArg.mode,
// base: baseOp,
// sup: funcName === "\\underset" ? null : shiftedArg,
// sub: funcName === "\\underset" ? shiftedArg : null,
// };
//
// return {
// type: "mclass",
// mode: parser.mode,
// mclass,
// body: [supsub],
// isCharacterBox: utils.isCharacterBox(supsub),
// };
// },
// htmlBuilder,
// mathmlBuilder,
// });
