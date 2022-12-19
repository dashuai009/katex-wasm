use crate::build::common::{PositionType, VListChild, VListParam};
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
use crate::{
    calculate_size, parse_node, types::ArgType, AnyParseNode, HtmlDomNode, StyleInterface,
};
use std::any::Any;
use std::sync::Mutex;

pub fn adjust_style(size: &String, original_style: StyleInterface) -> StyleInterface {
    use crate::Style::{DISPLAY, SCRIPT, SCRIPTSCRIPT, TEXT};
    let _script = SCRIPT.read().unwrap();
    let _display = DISPLAY.read().unwrap();
    let _script_script = SCRIPTSCRIPT.read().unwrap();
    let _text = TEXT.read().unwrap();
    // Figure out what style this fraction should be in based on the
    // function used
    let mut style = original_style;
    if size == "display" {
        // Get display style as a default.
        // If incoming style is sub/sup, use style.text() to get correct size.
        style = if style.id >= _script.id {
            style.text()
        } else {
            _display.clone()
        };
    } else if size == "text" && style.size == _display.size {
        // We're in a \tfrac but incoming style is displaystyle, so:
        style = _text.clone();
    } else if size == "script"  {
        style = _script.clone();
    } else if size == "scriptscript" {
        style = _script_script.clone();
    }
    return style;
}

pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::genfrac>()
        .unwrap();
    // Fractions are handled in the TeXbook on pages 444-445, rules 15(a-e).
    let style = adjust_style(&group.size, options.get_style());

    let nstyle = style.fracNum();
    let dstyle = style.fracDen();
    let mut newOptions = options.having_style(&nstyle);
    let mut numerm =
        HTML::build_group(Some(group.numer.clone()), newOptions, Some(options.clone()));

    if group.continued {
        // \cfrac inserts a \strut into the numerator.
        // Get \strut dimensions from TeXbook page 353.
        let hStrut = 8.5 / options.get_font_metrics().ptPerEm;
        let dStrut = 3.5 / options.get_font_metrics().ptPerEm;
        if numerm.get_height() < hStrut {
            numerm.set_height(hStrut);
        }
        if numerm.get_depth() < dStrut {
            numerm.set_depth(dStrut);
        }
    }

    newOptions = options.having_style(&dstyle);
    let denomm = HTML::build_group(Some(group.denom.clone()), newOptions, Some(options.clone()));

    let mut rule = None;
    let mut rule_width;
    let mut rule_spacing;
    if group.hasBarLine {
        let rule_span = if let Some(bar) = &group.barSize {
            rule_width = calculate_size(bar, &options);
            common::make_line_span(
                "frac-line".to_string(),
                &options,
                Some(rule_width),
            )
        } else {
            common::make_line_span(
                "frac-line".to_string(),
                &options,
                None,
            )
        };
        rule_width = rule_span.get_height();
        rule_spacing = rule_span.get_height();
        rule = Some(rule_span);
    } else {
        rule = None;
        rule_width = 0.0;
        rule_spacing = options.get_font_metrics().defaultRuleThickness;
    }

    // Rule 15b
    let mut numShift;
    let clearance;
    let mut denomShift;
    let _display = crate::Style::DISPLAY.read().unwrap();
    if style.size == _display.size || group.size == "display" {
        numShift = options.get_font_metrics().num1;
        if rule_width > 0.0 {
            clearance = 3.0 * rule_spacing;
        } else {
            clearance = 7.0 * rule_spacing;
        }
        denomShift = options.get_font_metrics().denom1;
    } else {
        if rule_width > 0.0 {
            numShift = options.get_font_metrics().num2;
            clearance = rule_spacing;
        } else {
            numShift = options.get_font_metrics().num3;
            clearance = 3.0 * rule_spacing;
        }
        denomShift = options.get_font_metrics().denom2;
    }

    let mut frac;
    if rule.is_none() {
        // Rule 15c
        let candidateClearance = (numShift - numerm.get_depth()) - (denomm.get_height() - denomShift);
        if (candidateClearance < clearance) {
            numShift += 0.5 * (clearance - candidateClearance);
            denomShift += 0.5 * (clearance - candidateClearance);
        }

        frac = common::make_vlist(
            VListParam {
                position_type: PositionType::IndividualShift,
                children: vec![
                    VListChild::Elem {
                        elem: denomm,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(denomShift),
                    },
                    VListChild::Elem {
                        elem: numerm,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(-numShift),
                    },
                ],
                position_data: None,
            }
        );
    } else {
        // Rule 15d
        let axisHeight = options.get_font_metrics().axisHeight;

        if (numShift - numerm.get_depth()) - (axisHeight + 0.5 * rule_width) < clearance {
            numShift += clearance - ((numShift - numerm.get_depth()) - (axisHeight + 0.5 * rule_width));
        }

        if (axisHeight - 0.5 * rule_width) - (denomm.get_height() - denomShift) < clearance {
            denomShift +=
                clearance - ((axisHeight - 0.5 * rule_width) - (denomm.get_height() - denomShift));
        }

        let midShift = -(axisHeight - 0.5 * rule_width);

        frac = common::make_vlist(
            VListParam {
                position_type: PositionType::IndividualShift,
                children: vec![
                    VListChild::Elem {
                        elem: denomm,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(denomShift),
                    },
                    VListChild::Elem {
                        elem: Box::new(rule.unwrap()) as Box<dyn HtmlDomNode>,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(midShift),
                    },
                    VListChild::Elem {
                        elem: numerm,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: Some(-numShift),
                    },
                ],
                position_data: None,
            }
        );
    }

    // Since we manually change the style sometimes (with \dfrac or \tfrac),
    // account for the possible size change here.
    newOptions = options.having_style(&style);
    frac.set_height(frac.get_height() * newOptions.sizeMultiplier / options.sizeMultiplier);
    frac.set_depth(frac.get_depth() * newOptions.sizeMultiplier / options.sizeMultiplier);

    // Rule 15e
    let _scriptscript = crate::Style::SCRIPTSCRIPT.read().unwrap();
    let _script = crate::Style::SCRIPT.read().unwrap();
    let delimSize = if style.size == _display.size {
        options.get_font_metrics().delim1
    } else if style.size == _scriptscript.size {
        options.having_style(&_script).get_font_metrics().delim2
    } else {
        options.get_font_metrics().delim2
    };

    let leftDelim;
    let rightDelim;
    if let Some(group_left_delim) = &group.leftDelim {
        leftDelim = crate::delimiter::make_custom_sized_delim(
            group_left_delim,
            delimSize,
            true,
            &options.having_style(&style),
            group.mode,
            vec!["mopen".to_string()],
        );
    } else {
        leftDelim = HTML::make_null_delimiter(&options, vec!["mopen".to_string()]);
    }
    if group.continued {
        rightDelim = common::make_span(vec![], vec![], None, Default::default());
    // zero width for \cfrac
    } else if let Some(r_del) = &group.rightDelim {
        rightDelim = crate::delimiter::make_custom_sized_delim(
            r_del,
            delimSize,
            true,
            &options.having_style(&style),
            group.mode,
            vec!["mclose".to_string()],
        );
    } else {
        rightDelim = HTML::make_null_delimiter(&options, vec!["mclose".to_string()]);
    }

    let chidren = vec![
        leftDelim,
        common::make_span(
            vec!["mfrac".to_string()],
            vec![Box::new(frac) as Box<dyn HtmlDomNode>],
            None,
            Default::default(),
        ),
        rightDelim,
    ]
    .into_iter()
    .map(|x| Box::new(x) as Box<dyn HtmlDomNode>)
    .collect();
    let res = common::make_span(
        [
            vec!["mord".to_string()],
            newOptions.sizing_classes(&options),
        ]
        .concat(),
        chidren,
        Some(&options),
        Default::default(),
    );
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined mathml builder");
    // let group = _group.as_any().downcast_ref::<parse_node::types::genfrac>().unwrap();
    // let node = MathNode::new(
    //     "mfrac".to_string(),
    //     vec![
    //         mathML::build_group(group.numer, options),
    //         mathML::build_group(group.denom, options),
    //     ]);
    //
    // if !group.hasBarLine {
    //     node.set_attribute("linethickness".to_string(), "0px".to_string());
    // } else if (group.barSize) {
    //     let ruleWidth = calculateSize(group.barSize, options);
    //     node.set_attribute("linethickness".to_string(), crate::units::make_em(ruleWidth));
    // }
    //
    // let style = adjustStyle(group.size, options.style);
    // if (style.size != options.style.size) {
    //     node = MathNode::new("mstyle".to_string(), [node]);
    //     let isDisplay = (style.size == Style.DISPLAY.size)?
    //     "true": "false";
    //     node.set_attribute("displaystyle".to_string(), isDisplay);
    //     node.set_attribute("scriptlevel".to_string(), "0".to_string());
    // }
    //
    // if (group.leftDelim != null | | group.rightDelim != null) {
    //     let withDelims = [];
    //
    //     if (group.leftDelim != null) {
    //         let leftOp = MathNode::new(
    //             "mo".to_string(),
    //             [new mathMLTree.TextNode(group.leftDelim.replace("\\".to_string(), "".to_string()))]
    //         );
    //
    //         leftOp.set_attribute("fence".to_string(), "true".to_string());
    //
    //         withDelims.push(leftOp);
    //     }
    //
    //     withDelims.push(node);
    //
    //     if (group.rightDelim != null) {
    //         let rightOp = MathNode::new(
    //             "mo".to_string(),
    //             [new mathMLTree.TextNode(group.rightDelim.replace("\\".to_string(), "".to_string()))]
    //         );
    //
    //         rightOp.set_attribute("fence".to_string(), "true".to_string());
    //
    //         withDelims.push(rightOp);
    //     }
    //
    //     return mml.makeRow(withDelims);
    // }
    //
    // return node;
}

pub fn frac_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let has_bar_line;
    let mut left_delim = None;
    let mut right_delim = None;
    let mut size = "auto".to_string();

    match context.func_name.as_str() {
        "\\dfrac" | "\\frac" | "\\tfrac" => {
            has_bar_line = true;
        }
        "\\\\atopfrac" => {
            has_bar_line = false;
        }
        "\\dbinom" | "\\binom" | "\\tbinom" => {
            has_bar_line = false;

            left_delim = Some("(".to_string());
            right_delim = Some(")".to_string());
        }
        "\\\\bracefrac" => {
            has_bar_line = false;
            left_delim = Some("\\{".to_string());
            right_delim = Some("\\}".to_string());
        }
        "\\\\brackfrac" => {
            has_bar_line = false;
            left_delim = Some("[".to_string());
            right_delim = Some("]".to_string());
        }
        _ => panic!("Unrecognized genfrac command"),
    }

    match context.func_name.as_str() {
        "\\dfrac" | "\\dbinom" => {
            size = "display".to_string();
        }
        "\\tfrac" | "\\tbinom" => {
            size = "text".to_string();
        }
        _ => {}
    }

    let res = parse_node::types::genfrac {
        mode: context.parser.mode,
        loc: None,
        continued: false,
        numer: args[0].clone(),
        denom: args[1].clone(),
        hasBarLine: has_bar_line,
        leftDelim: left_delim,
        rightDelim: right_delim,
        size,
        barSize: None,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}
lazy_static! {
    pub static ref FRAC : Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(2);
        props.set_allowed_in_argument(true);

        FunctionDefSpec{
            def_type: "genfrac".to_string(),
            names: vec![
        "\\dfrac".to_string(), "\\frac".to_string(), "\\tfrac".to_string(),
        "\\dbinom".to_string(), "\\binom".to_string(), "\\tbinom".to_string(),
        "\\\\atopfrac".to_string(), // can’t be entered directly
        "\\\\bracefrac".to_string(), "\\\\brackfrac".to_string(),   // ditto
            ],
            props,
            handler:frac_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
//
// pub fn cfrac_handler_fn(
//     context: FunctionContext,
//     args: Vec<Box<dyn AnyParseNode>>,
//     opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
// ) -> Box<dyn AnyParseNode> {
//     let numer = &args[0];
//     let denom = &args[1];
//
//     return {
//         type : "genfrac".to_string(),
//         mode: parser.mode,
//         continued: true,
//         numer,
//         denom,
//         hasBarLine: true,
//         leftDelim: null,
//         rightDelim: null,
//         size: "display".to_string(),
//         barSize: null,
//     };
// }
// lazy_static! {
//     pub static ref CFRAC : Mutex<FunctionDefSpec> = Mutex::new({
//         let mut props = FunctionPropSpec::new();
//         props.set_num_args(2);
//
//         FunctionDefSpec{
//             def_type: "genfrac".to_string(),
//             names: vec!["\\cfrac".to_string()],
//             props,
//             handler:cfrac_handler_fn,
//             html_builder: Some(html_builder),
//             mathml_builder: Some(mathml_builder),
//         }
//     });
// }
//
//
//
// pub fn mclass_handler_fn(
//     context: FunctionContext,
//     args: Vec<Box<dyn AnyParseNode>>,
//     opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
// ) -> Box<dyn AnyParseNode> {
// let replaceWith;
// match (funcName) {
// "\\over" =>{
// replaceWith = "\\frac";
// },
// "\\choose" =>{
// replaceWith = "\\binom";
// },
// "\\atop" =>{
// replaceWith = "\\\\atopfrac";
// },
// "\\brace" =>{
// replaceWith = "\\\\bracefrac";
// },
// "\\brack" =>{
// replaceWith = "\\\\brackfrac";
// },
//     _ =>{
//         panic!("Unrecognized infix genfrac command".to_string());
//
//     }
// }
// return parse_node::types::infix{
// mode: parser.mode,
//     loc: None,
//     replace_with:replaceWith ,
//
// token,
//     size: None
// }
// }
// lazy_static! {
//     pub static ref OCABB : Mutex<FunctionDefSpec> = Mutex::new({
//         let mut props = FunctionPropSpec::new();
//         props.set_num_args(1);
//         props.set_primitive(true);
//
//         FunctionDefSpec{
//             def_type: "infix".to_string(),
//             names: vec! ["\\over".to_string(), "\\choose".to_string(), "\\atop".to_string(), "\\brace".to_string(), "\\brack".to_string()],
//             props,
//             handler:mclass_handler_fn,
//             html_builder: Some(html_builder),
//             mathml_builder: Some(mathml_builder),
//         }
//     });
// }
//
// // Infix generalized fractions -- these are not rendered directly, but replaced
// // immediately by one of the variants above.
// lazy_static! {
//     type: "infix".to_string(),
//     names:,
//     props: {
//         numArgs: 0,
//         infix: true,
//     },
// ,
// });
//
// let stylArray = ["display".to_string(), "text".to_string(), "script".to_string(), "scriptscript".to_string()];
//
// fn delimFromValue(delimString: String) -> Option<String> {
//     let delim = None;
//     if (delimString.length > 0) {
//         delim = delimString;
//         delim = delim == "."?
//         null: delim;
//     }
//     return delim;
// };
//
// pub fn genfrac_handler_fn(
//     context: FunctionContext,
//     args: Vec<Box<dyn AnyParseNode>>,
//     opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
// ) -> Box<dyn AnyParseNode> {
//     let numer = args[4];
//     let denom = args[5];
//
// // Look into the parse nodes to get the desired delimiters.
//     let leftNode = normalizeArgument(args[0]);
//     let leftDelim = leftNode. type == "atom" && leftNode.family == "open"
//         ?
//     delimFromValue(leftNode.text): null;
//     let rightNode = normalizeArgument(args[1]);
//     let rightDelim = rightNode. type == "atom" && rightNode.family == "close"
//         ?
//     delimFromValue(rightNode.text): null;
//
//     let barNode = assertNodeType(args[2], "size".to_string());
//     let hasBarLine;
//     let barSize = null;
//     if (barNode.isBlank) {
// // \genfrac acts differently than \above.
// // \genfrac treats an empty size group as a signal to use a
// // standard bar size. \above would see size = 0 and omit the bar.
//         hasBarLine = true;
//     } else {
//         barSize = barNode.value;
//         hasBarLine = barSize.number > 0;
//     }
//
// // Find out if we want displaystyle, textstyle, etc.
//     let size = "auto";
//     let styl = args[3];
//     if (styl. type == "ordgroup".to_string()) {
//         if (styl.body.length > 0) {
//             let textOrd = assertNodeType(styl.body[0], "textord".to_string());
//             size = stylArray[Number(textOrd.text)];
//         }
//     } else {
//         styl = assertNodeType(styl, "textord".to_string());
//         size = stylArray[Number(styl.text)];
//     }
//
//     return {
//         type : "genfrac".to_string(),
//         mode: parser.mode,
//         numer,
//         denom,
//         continued: false,
//         hasBarLine,
//         barSize,
//         leftDelim,
//         rightDelim,
//         size,
//     };
// }
// lazy_static! {
//     pub static ref GENFRAC : Mutex<FunctionDefSpec> = Mutex::new({
//         let mut props = FunctionPropSpec::new();
//         props.set_num_args(1);
//         props.set_primitive(true);{
//         numArgs: 6,
//         allowedInArgument: true,
//         argTypes: ["math".to_string(), "math".to_string(), "size".to_string(), "text".to_string(), "math".to_string(), "math".to_string()],
//     }
//
//         FunctionDefSpec{
//             def_type: "genfrac".to_string(),
//             names: vec!["\\genfrac".to_string()],
//             props,
//             handler:genfrac_handler_fn,
//             html_builder: Some(html_builder),
//             mathml_builder: Some(mathml_builder),
//         }
//     });
// }
//
// fn infix_handler_fn(
//     context: FunctionContext,
//     args: Vec<Box<dyn AnyParseNode>>,
//     opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
// ) -> Box<dyn AnyParseNode> {
//     return {
//         type : "infix".to_string(),
//         mode: parser.mode,
//         replaceWith: "\\\\abovefrac".to_string(),
//         size: assertNodeType(args[0], "size".to_string()).value,
//         token,
//     };
// }
// // \above is an infix fraction that also defines a fraction bar size.
// lazy_static! {
//     pub static ref INFIX : Mutex<FunctionDefSpec> = Mutex::new({
//         let mut props = FunctionPropSpec::new();
//         props.set_num_args(1);
//         argTypes: ["size".to_string()],
//         props.set_infix(true);
//
//         FunctionDefSpec{
//             def_type: "mclass".to_string(),
//             names: vec!["\\above".to_string()],
//             props,
//             handler:infix_handler_fn,
//             html_builder: Some(html_builder),
//             mathml_builder: Some(mathml_builder),
//         }
//     });
// }
//
// pub fn above_frac_handler_fn(
//     context: FunctionContext,
//     args: Vec<Box<dyn AnyParseNode>>,
//     opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
// ) -> Box<dyn AnyParseNode> {
//     let numer = &args[0];
//     let barSize = assert(assertNodeType(&args[1], "infix".to_string()).size);
//     let denom = &args[2];
//
//     let hasBarLine = barSize.number > 0;
//     return {
//         type : "genfrac".to_string(),
//         mode: parser.mode,
//         numer,
//         denom,
//         continued: false,
//         hasBarLine,
//         barSize,
//         leftDelim: null,
//         rightDelim: null,
//         size: "auto".to_string(),
//     };
// }
//
// lazy_static! {
//     pub static ref MCLASS : Mutex<FunctionDefSpec> = Mutex::new({
//         let mut props = FunctionPropSpec::new();
//         props.set_num_args(3);
//         argTypes: ["math".to_string(), "size".to_string(), "math".to_string()]
//
//         FunctionDefSpec{
//             def_type: "genfrac".to_string(),
//             names: vec!["\\\\abovefrac".to_string()            ],
//             props,
//             handler:above_frac_handler_fn,
//             html_builder: Some(html_builder),
//             mathml_builder: Some(mathml_builder),
//         }
//     });
// }
