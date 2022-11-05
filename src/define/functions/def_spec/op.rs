use crate::build::common::{PositionType, VListChild, VListParam};
use crate::build::mathML::make_text;
use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
use crate::define::functions::def_spec::assembleSupSub::assemble_sup_sub;
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::dom_tree::symbol_node::SymbolNode;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::ParseNodeToAny;
use crate::types::Mode;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode, VirtualNode};
use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;
// Limits, symbols

// Most operators have a large successor symbol, but these don't.
const NO_SUCCESSOR: [&'static str; 1] = ["\\smallint"];

// NOTE: Unlike most `html_builder`s, this one handles not only "op".to_string(), but also
// "supsub" since some of them (like \int) can affect super/subscripting.
pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    // Operators are handled in the TeXbook pg. 443-444, rule 13(a).
    let mut sup_group = &None;
    let mut sub_group = &None;
    let mut has_limits = false;
    let group;
    if let Some(grp) = _group.as_any().downcast_ref::<parse_node::types::supsub>() {
        // If we have limits, supsub will pass us its group to handle. Pull
        // out the superscript and subscript and set the group to the op in
        // its base.
        sup_group = &grp.sup;
        sub_group = &grp.sub;
        group = grp
            .base
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<parse_node::types::op>()
            .unwrap();
        has_limits = true;
    } else {
        group = _group
            .as_any()
            .downcast_ref::<parse_node::types::op>()
            .unwrap();
    }

    let style = options.get_style();

    let mut large = false;
    let _display = crate::Style::DISPLAY.read().unwrap();
    if style.size == _display.size
        && group.symbol
        && !NO_SUCCESSOR.contains(&&**group.name.as_ref().unwrap())
    {
        // Most symbol operators get larger in displaystyle (rule 13)
        large = true;
    }

    let mut base;
    let mut base_italic = 0.0;
    let mut group_name: String = group.name.clone().unwrap();
    if group.symbol {
        // If this is a symbol, create the symbol.
        let font_name = if large {
            "Size2-Regular"
        } else {
            "Size1-Regular"
        };

        let mut stash = "";
        if let Some(gn) = &group.name {
            if gn == "\\oiint" || gn == "\\oiiint" {
                // No font glyphs yet, so use a glyph w/o the oval.
                // TODO: When font glyphs are available, delete this code.
                stash = &gn[1..];
                group_name = if stash == "oiint" {
                    "\\iint"
                } else {
                    "\\iiint"
                }
                .to_string();
            }
        }

        base = Box::new(common::make_symbol(
            group_name.to_string(),
            font_name.to_string(),
            Mode::math,
            Some(&options),
            vec![
                "mop".to_string(),
                "op-symbol".to_string(),
                if large { "large-op" } else { "small-op" }.to_string(),
            ],
        )) as Box<dyn HtmlDomNode>;

        if stash.len() > 0 {
            // We're in \oiint or \oiiint. Overlay the oval.
            // TODO: When font glyphs are available, delete this code.
            let italic = base.as_any().downcast_ref::<SymbolNode>().unwrap().italic;
            let oval = common::static_svg(
                format!("{}Size{}", stash, (if large { "2" } else { "1" })),
                options.clone(),
            );
            let mut tmp = common::make_vlist(
                VListParam {
                    position_type: PositionType::IndividualShift,
                    children: vec![
                        VListChild::Elem {
                            elem: base,
                            margin_left: None,
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: Some(0.0),
                        },
                        VListChild::Elem {
                            elem: Box::new(oval) as Box<dyn HtmlDomNode>,
                            margin_left: None,
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: Some(if large { 0.08 } else { 0.0 }),
                        },
                    ],
                    position_data: None,
                },
                options.clone(),
            );
            group_name = format!("\\{stash}");
            tmp.get_mut_classes().insert(0, "mop".to_string());
            base_italic = italic;
            base = Box::new(tmp) as Box<dyn HtmlDomNode>;
        }
    } else if group.body.is_some() {
        // If this is a list, compose that list.
        let inner = HTML::build_expression(
            group.body.clone().unwrap(),
            options.clone(),
            IsRealGroup::T,
            (None, None),
        );
        base = if (inner.len() == 1) {
            if let Some(sym) = inner[0].as_any().downcast_ref::<SymbolNode>() {
                let mut tmp = sym.clone();
                tmp.get_mut_classes()[0] = "mop".to_string(); // replace old mclass
                Box::new(tmp) as Box<dyn HtmlDomNode>
            } else {
                Box::new(common::make_span(
                    vec!["mop".to_string()],
                    inner,
                    Some(&options),
                    Default::default(),
                )) as Box<dyn HtmlDomNode>
            }
        } else {
            Box::new(common::make_span(
                vec!["mop".to_string()],
                inner,
                Some(&options),
                Default::default(),
            )) as Box<dyn HtmlDomNode>
        }
    } else {
        // Otherwise, this is a text operator. Build the text from the
        // operator's name.
        let output = group_name
            .chars()
            .into_iter()
            .map(|c| {
                Box::new(common::math_sym(
                    c.to_string(),
                    group.mode,
                    options.clone(),
                    vec![],
                )) as Box<dyn HtmlDomNode>
            })
            .collect::<Vec<_>>();
        base = Box::new(common::make_span(
            vec!["mop".to_string()],
            output,
            Some(&options),
            Default::default(),
        )) as Box<dyn HtmlDomNode>;
    }

    // If content of op is a single symbol, shift it vertically.
    let mut base_shift = 0.0;
    let mut slant = 0.0;
    if (base.as_any().is::<SymbolNode>() || group_name == "\\oiint" || group_name == "\\oiiint")
        && !group.suppressBaseShift
    {
        // We suppress the shift of the base of \overset and \underset. Otherwise,
        // shift the symbol so its center lies on the axis (rule 13). It
        // appears that our fonts have the centers of the symbols already
        // almost on the axis, so these numbers are very small. Note we
        // don't actually apply this here, but instead it is used either in
        // the vlist creation or separately when there are no limits.
        base_shift =
            (base.get_height() - base.get_depth()) / 2.0 - options.get_font_metrics().axisHeight;

        // The slant of the symbol is just its italic correction.
        // $FlowFixMe
        slant = base_italic;
    }

    if has_limits {
        return Box::new(assemble_sup_sub(
            &base, sup_group, sub_group, &options, &style, slant, base_shift,
        )) as Box<dyn HtmlDomNode>;
    } else {
        if base_shift != 0.0 {
            base.get_mut_style().position = Some("relative".to_string());
            base.get_mut_style().top = Some(crate::units::make_em(base_shift));
        }

        return base;
    }
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined mathml builder")
    // let group = _group
    //     .as_any()
    //     .downcast_ref::<parse_node::types::op>()
    //     .unwrap();
    // let mut node;
    //
    // if (group.symbol) {
    //     // This is a symbol. Just add the symbol.
    //     node = MathNode::new(MathNodeType::Mo , vec![mml.makeText(group.name, group.mode)]);
    //     if NO_SUCCESSOR.contains(&&*group.name.unwrap()) {
    //         node.set_attribute("largeop".to_string(), "false".to_string());
    //     }
    // } else if (group.body) {
    //     // This is an operator with children. Add them.
    //     node = MathNode::new(MathNodeType::Mo , mml.buildExpression(group.body, options));
    // } else {
    //     // This is a text operator. Add all of the characters from the
    //     // operator's name.
    //     node = MathNode::new(MathNodeType::Mi , vec![ TextNode(group.name.slice(1))]);
    //     // Append an <mo>&ApplyFunction;</mo>.
    //     // ref: https://www.w3.org/TR/REC-MathML/chap3_2.html#sec3.2.4
    //     let operator = MathNode::new(MathNodeType::Mo, vec![make_text("\u{2061}".to_string(), Mode::text)], vec![]);
    //     if (group.parentIsSupSub) {
    //         node = MathNode::new(MathNodeType::Mrow, vec![node, operator], vec![]);
    //     } else {
    //         node = mathMLTree.newDocumentFragment([node, operator]);
    //     }
    // }
    //
    // return node;
}

lazy_static! {
    static ref SINGLE_CHAR_BIG_OPS: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("\u{220F}", "\\prod"),
            ("\u{2210}", "\\coprod"),
            ("\u{2211}", "\\sum"),
            ("\u{22c0}", "\\bigwedge"),
            ("\u{22c1}", "\\bigvee"),
            ("\u{22c2}", "\\bigcap"),
            ("\u{22c3}", "\\bigcup"),
            ("\u{2a00}", "\\bigodot"),
            ("\u{2a01}", "\\bigoplus"),
            ("\u{2a02}", "\\bigotimes"),
            ("\u{2a04}", "\\biguplus"),
            ("\u{2a06}", "\\bigsqcup"),
        ])
    };
}

pub fn op_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let tmp = context.func_name.as_str();
    let f_name = SINGLE_CHAR_BIG_OPS
        .get(context.func_name.as_str())
        .unwrap_or(&tmp);
    let res = parse_node::types::op {
        mode: context.parser.mode,
        loc: None,
        limits: true,
        alwaysHandleSupSub: false,
        suppressBaseShift: false,
        parentIsSupSub: false,
        symbol: true,
        name: Some(f_name.to_string()),
        body: None,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

lazy_static! {
    pub static ref OP: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);

        FunctionDefSpec {
            def_type: "op".to_string(),
            names: vec![
                "\\coprod".to_string(),
                "\\bigvee".to_string(),
                "\\bigwedge".to_string(),
                "\\biguplus".to_string(),
                "\\bigcap".to_string(),
                "\\bigcup".to_string(),
                "\\intop".to_string(),
                "\\prod".to_string(),
                "\\sum".to_string(),
                "\\bigotimes".to_string(),
                "\\bigoplus".to_string(),
                "\\bigodot".to_string(),
                "\\bigsqcup".to_string(),
                "\\smallint".to_string(),
                "\u{220F}".to_string(),
                "\u{2210}".to_string(),
                "\u{2211}".to_string(),
                "\u{22c0}".to_string(),
                "\u{22c1}".to_string(),
                "\u{22c2}".to_string(),
                "\u{22c3}".to_string(),
                "\u{2a00}".to_string(),
                "\u{2a01}".to_string(),
                "\u{2a02}".to_string(),
                "\u{2a04}".to_string(),
                "\u{2a06}".to_string(),
            ],
            props,
            handler: op_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

pub fn math_op_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let body = &args[0];
    let res = parse_node::types::op {
        mode: context.parser.mode,
        loc: None,
        limits: false,
        alwaysHandleSupSub: false,
        suppressBaseShift: false,
        parentIsSupSub: false,
        symbol: false,
        name: None,
        body: Some(ord_argument(body)),
    };

    return Box::new(res) as Box<dyn AnyParseNode>;
}

// Note: calling defineFunction with a type that's already been defined only
// works because the same html_builder and mathml_builder are being used.
lazy_static! {
    pub static ref MATH_OP: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_primitive(true);

        FunctionDefSpec {
            def_type: "op".to_string(),
            names: vec!["\\mathop".to_string()],
            props,
            handler: math_op_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

// There are 2 flags for operators; whether they produce limits in
// displaystyle, and whether they are symbols and should grow in
// displaystyle. These four groups cover the four possible choices.
lazy_static! {
    static ref SINGLE_CHAR_INTEGRALS: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("\u{222b}", "\\int"),
            ("\u{222c}", "\\iint"),
            ("\u{222d}", "\\iiint"),
            ("\u{222e}", "\\oint"),
            ("\u{222f}", "\\oiint"),
            ("\u{2230}", "\\oiiint"),
        ])
    };
}

pub fn trigonometric_op_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let res = parse_node::types::op {
        mode: context.parser.mode,
        loc: None,
        limits: false,
        alwaysHandleSupSub: false,
        suppressBaseShift: false,
        parentIsSupSub: false,
        symbol: false,
        name: Some(context.func_name.clone()),
        body: None,
    };

    return Box::new(res) as Box<dyn AnyParseNode>;
}
// No limits, not symbols
lazy_static! {
    pub static ref TRIGNONOMETRIC_OP: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);

        FunctionDefSpec {
            def_type: "op".to_string(),
            names: vec![
                "\\arcsin".to_string(),
                "\\arccos".to_string(),
                "\\arctan".to_string(),
                "\\arctg".to_string(),
                "\\arcctg".to_string(),
                "\\arg".to_string(),
                "\\ch".to_string(),
                "\\cos".to_string(),
                "\\cosec".to_string(),
                "\\cosh".to_string(),
                "\\cot".to_string(),
                "\\cotg".to_string(),
                "\\coth".to_string(),
                "\\csc".to_string(),
                "\\ctg".to_string(),
                "\\cth".to_string(),
                "\\deg".to_string(),
                "\\dim".to_string(),
                "\\exp".to_string(),
                "\\hom".to_string(),
                "\\ker".to_string(),
                "\\lg".to_string(),
                "\\ln".to_string(),
                "\\log".to_string(),
                "\\sec".to_string(),
                "\\sin".to_string(),
                "\\sinh".to_string(),
                "\\sh".to_string(),
                "\\tan".to_string(),
                "\\tanh".to_string(),
                "\\tg".to_string(),
                "\\th".to_string(),
            ],
            props,
            handler: trigonometric_op_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

// Limits, not symbols
pub fn gcd_op_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let res = parse_node::types::op {
        mode: context.parser.mode,
        loc: None,
        limits: true,
        alwaysHandleSupSub: false,
        suppressBaseShift: false,
        parentIsSupSub: false,
        symbol: false,
        name: Some(context.func_name.clone()),
        body: None,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}
lazy_static! {
    pub static ref GCD_OP: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        FunctionDefSpec {
            def_type: "op".to_string(),
            names: vec![
                "\\det".to_string(),
                "\\gcd".to_string(),
                "\\inf".to_string(),
                "\\lim".to_string(),
                "\\max".to_string(),
                "\\min".to_string(),
                "\\Pr".to_string(),
                "\\sup".to_string(),
            ],
            props,
            handler: gcd_op_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

// No limits, symbols
pub fn int_op_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let tmp = context.func_name.as_str();
    let f_name = SINGLE_CHAR_INTEGRALS
        .get(context.func_name.as_str())
        .unwrap_or(&tmp);

    let res = parse_node::types::op {
        mode: context.parser.mode,
        loc: None,
        limits: false,
        alwaysHandleSupSub: false,
        suppressBaseShift: false,
        parentIsSupSub: false,
        symbol: true,
        name: Some(f_name.to_string()),
        body: None,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}
lazy_static! {
    pub static ref INT_OP: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);

        FunctionDefSpec {
            def_type: "op".to_string(),
            names: vec![
                "\\int".to_string(),
                "\\iint".to_string(),
                "\\iiint".to_string(),
                "\\oint".to_string(),
                "\\oiint".to_string(),
                "\\oiiint".to_string(),
                "\u{222b}".to_string(),
                "\u{222c}".to_string(),
                "\u{222d}".to_string(),
                "\u{222e}".to_string(),
                "\u{222f}".to_string(),
                "\u{2230}".to_string(),
            ],
            props,
            handler: int_op_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
