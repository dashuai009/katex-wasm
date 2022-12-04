use crate::build::common::{PositionType, VListChild, VListParam};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::dom_tree::symbol_node::SymbolNode;
use crate::mathML_tree::public::MathDomNode;
use crate::types::{ArgType, Mode};
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{make_em, parse_node, stretchy, utils, AnyParseNode, HtmlDomNode};
use std::any::Any;
use std::sync::Mutex;

// NOTE: Unlike most `htmlBuilder`s, this one handles not only "accent", but
// also "supsub" since an accent can affect super/subscripting.
fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    // Accents are handled in the TeXbook pg. 443, rule 12.

    let mut supSubGroup = None;
    let group;
    let base;
    if let Some(grp) = _group.as_any().downcast_ref::<parse_node::types::supsub>() {
        // If our base is a character box, and we have superscripts and
        // subscripts, the supsub will defer to us. In particular, we want
        // to attach the superscripts and subscripts to the inner body (so
        // that the position of the superscripts and subscripts won't be
        // affected by the height of the accent). We accomplish this by
        // sticking the base of the accent into the base of the supsub, and
        // rendering that, while keeping track of where the accent is.

        // The real accent group is the base of the supsub group
        group = grp
            .base
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<parse_node::types::accent>()
            .unwrap();
        // The character box is the base of the accent group
        base = &group.base;
        // Stick the character box into the base of the supsub group
        let tmp_grp = Box::new(parse_node::types::supsub {
            base: base.clone(),
            ..grp.clone()
        }) as Box<dyn AnyParseNode>;

        // Rerender the supsub group with its new base, and store that
        // result.
        supSubGroup = Some(crate::build::HTML::build_group(
            Some(tmp_grp),
            options.clone(),
            None,
        ));

        // reset original base
        // grp.base = group;
    } else {
        group = _group
            .as_any()
            .downcast_ref::<parse_node::types::accent>()
            .unwrap();
        base = &group.base;
    }

    // Build the base group
    let body = crate::build::HTML::build_group(base.clone(), options.having_cramped_style(), None);

    // Does the accent need to shift for the skew of a character?
    let must_shift = group.isShifty && is_character_box(&base.as_ref().unwrap());

    // Calculate the skew of the accent. This is based on the line "If the
    // nucleus is not a single character, let s = 0; otherwise set s to the
    // kern amount for the nucleus followed by the \skewchar of its font."
    // Note that our skew metrics are just the kern between each character
    // and the skewchar.
    let mut skew = 0.0;
    if (must_shift) {
        // If the base is a character box, then we want the skew of the
        // innermost character. To do that, we find the innermost character:
        let baseChar = utils::get_base_elem(&base.as_ref().unwrap());
        // Then, we render its group to get the symbol inside it
        let baseGroup = crate::build::HTML::build_group(
            Some(baseChar.clone()),
            options.having_cramped_style(),
            None,
        );
        // Finally, we pull the skew off of the symbol.
        skew = baseGroup
            .as_any()
            .downcast_ref::<SymbolNode>()
            .unwrap()
            .skew;
        // Note that we now throw away baseGroup, because the layers we
        // removed with getBaseElem might contain things like \color which
        // we can't get rid of.
        // TODO(emily): Find a better way to get the skew
    }

    let accentBelow = group.label == "\\c";

    // calculate the amount of space between the body and the accent
    let mut clearance = if accentBelow {
        body.get_height() + body.get_depth()
    } else {
        body.get_height().min(options.get_font_metrics().xHeight)
    };

    // Build the accent
    let mut accentBody;
    if !group.isStretchy {
        let accent;
        let mut width: f64 = 0.0;
        if (group.label == "\\vec") {
            // Before version 0.9, \vec used the combining font glyph U+20D7.
            // But browsers, especially Safari, are not consistent in how they
            // render combining characters when not preceded by a character.
            // So now we use an SVG.
            // If Safari reforms, we should consider reverting to the glyph.
            accent = Box::new(crate::build::common::static_svg(
                "vec".to_string(),
                options.clone(),
            )) as Box<dyn HtmlDomNode>;
            // width = crate::build::common::svgData.vec[1];
        } else {
            let mut _accent = crate::build::common::make_ord(
                Box::new(parse_node::types::textord {
                    mode: group.mode,
                    loc: None,
                    text: group.label.clone(),
                }) as Box<dyn AnyParseNode>,
                options.clone(),
                "textord".to_string(),
            );
            // accent = assertSymbolDomNode(accent);
            // Remove the italic correction of the accent, because it only serves to
            // shift the accent over to a place we don't want.
            _accent.italic = 0.0;
            width = _accent.width;
            if accentBelow {
                clearance += _accent.get_depth();
            }
            accent = Box::new(_accent) as Box<dyn HtmlDomNode>
        }

        accentBody = crate::build::common::make_span(
            vec!["accent-body".to_string()],
            vec![accent],
            None,
            Default::default(),
        );

        // "Full" accents expand the width of the resulting symbol to be
        // at least the width of the accent, and overlap directly onto the
        // character without any vertical offset.
        let accentFull = (group.label == "\\textcircled");
        if accentFull {
            accentBody.get_mut_classes().push("accent-full".to_string());
            clearance = body.get_height();
        }

        // Shift the accent over by the skew.
        let mut left = skew;

        // CSS defines `.katex .accent .accent-body:not(.accent-full) { width: 0 }`
        // so that the accent doesn't contribute to the bounding box.
        // We need to shift the character by its width (effectively half
        // its width) to compensate.
        if (!accentFull) {
            left -= width / 2.0;
        }

        accentBody.get_mut_style().left = Some(make_em(left as f64));

        // \textcircled uses the \bigcirc glyph, so it needs some
        // vertical adjustment to match LaTeX.
        if (group.label == "\\textcircled") {
            accentBody.get_mut_style().top = Some(".2em".to_string());
        }

        accentBody = crate::build::common::make_vlist(
            VListParam {
                position_type: PositionType::FirstBaseline,
                children: vec![
                    VListChild::Elem {
                        elem: body,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: None,
                    },
                    VListChild::Kern { size: -clearance },
                    VListChild::Elem {
                        elem: Box::new(accentBody) as Box<dyn HtmlDomNode>,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: None,
                    },
                ],
                position_data: None,
            },
            options.clone(),
        );
    } else {
        accentBody = stretchy::svg_span(
            Box::new(group.clone()) as Box<dyn AnyParseNode>,
            options.clone(),
        );

        accentBody = crate::build::common::make_vlist(
            VListParam {
                position_type: PositionType::FirstBaseline,
                children: vec![
                    VListChild::Elem {
                        elem: body,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: None,
                    },
                    VListChild::Elem {
                        elem: Box::new(accentBody) as Box<dyn HtmlDomNode>,
                        margin_left: None,
                        margin_right: None,
                        wrapper_classes: Some(vec!["svg-align".to_string()]),
                        wrapper_style: if skew > 0.0 {
                            Some(CssStyle {
                                width: Some(format!("calc(100% - {})", make_em(2.0 * skew))),
                                margin_left: Some(make_em(2.0 * skew)),
                                ..CssStyle::default()
                            })
                        } else {
                            None
                        },
                        shift: None,
                    },
                ],
                position_data: None,
            },
            options.clone(),
        );
    }

    let accentWrap = crate::build::common::make_span(
        vec!["mord".to_string(), "accent".to_string()],
        vec![Box::new(accentBody) as Box<dyn HtmlDomNode>],
        Some(&options.clone()),
        Default::default(),
    );

    if let Some(mut s) = supSubGroup {
        // Here, we replace the "base" child of the supsub with our newly
        // generated accent.
        let accent_wrap_h = accentWrap.get_height();
        s.get_mut_children().unwrap()[0] = Box::new(accentWrap) as Box<dyn HtmlDomNode>;

        // Since we don't rerun the height calculation after replacing the
        // accent, we manually recalculate height.
        s.set_height(accent_wrap_h.max(s.get_height()));

        // Accents should always be ords, even when their innards are not.
        s.get_mut_classes()[0] = "mord".to_string();

        return s;
    } else {
        return Box::new(accentWrap) as Box<dyn HtmlDomNode>;
    }
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("todo mathml builder");
    // let accentNode =
    //     group.isStretchy ?
    //         stretchy.mathMLnode(group.label) :
    //         new mathMLTree.MathNode("mo", [mml.makeText(group.label, group.mode)]);
    //
    // let node = new mathMLTree.MathNode(
    //     "mover",
    //     [mml.buildGroup(group.base, options), accentNode]);
    //
    // node.setAttribute("accent", "true");
    //
    // return node;
}

// let NON_STRETCHY_ACCENT_REGEX = new RegExp([
//     "\\acute", "\\grave", "\\ddot", "\\tilde", "\\bar", "\\breve",
//     "\\check", "\\hat", "\\vec", "\\dot", "\\mathring",
// ].map(accent => `\\${accent}`).join("|"));

fn handler_fn_1(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    panic!("undefined handler");
    // let base = normalizeArgument(args[0]);
    //
    // let isStretchy = !NON_STRETCHY_ACCENT_REGEX.test(context.funcName);
    // let isShifty = !isStretchy
    //     || context.funcName == "\\widehat"
    //     || context.funcName == "\\widetilde"
    //     || context.funcName == "\\widecheck";
    //
    // return Box::new(parse_node::types::accent {
    //     mode: context.parser.mode,
    //     loc: None,
    //     label: context.funcName,
    //     isStretchy: isStretchy,
    //     isShifty: isShifty,
    //     base: base,
    // }) as Box<dyn Any>;
}
// Accents
lazy_static! {
    pub static ref ACCENT: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);

        FunctionDefSpec {
            def_type: "accent".to_string(),
            names: vec![
                "\\acute".to_string(),
                "\\grave".to_string(),
                "\\ddot".to_string(),
                "\\tilde".to_string(),
                "\\bar".to_string(),
                "\\breve".to_string(),
                "\\check".to_string(),
                "\\hat".to_string(),
                "\\vec".to_string(),
                "\\dot".to_string(),
                "\\mathring".to_string(),
                "\\widecheck".to_string(),
                "\\widehat".to_string(),
                "\\widetilde".to_string(),
                "\\overrightarrow".to_string(),
                "\\overleftarrow".to_string(),
                "\\Overrightarrow".to_string(),
                "\\overleftrightarrow".to_string(),
                "\\overgroup".to_string(),
                "\\overlinesegment".to_string(),
                "\\overleftharpoon".to_string(),
                "\\overrightharpoon".to_string(),
            ],
            props,
            handler: handler_fn_1,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}

fn handler_fn_2(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let base = &args[0];
    let mut mode = context.borrow().parser.mode;
    if mode == Mode::math {
        // context.parser.settings.reportNonstrict("mathVsTextAccents",
        //                                         `LaTeX's accent ${context.funcName} works only in text mode`);
        println!(
            "LaTeX's accent {} works only in text mode",
            context.borrow().func_name
        );
        mode = Mode::text;
    }
    return Box::new(parse_node::types::accent {
        mode,
        loc: None,
        label: context.borrow().func_name.clone(),
        isStretchy: false,
        isShifty: true,
        base: Some(base.clone()),
    }) as Box<dyn AnyParseNode>;
}
// Text-mode accents
lazy_static! {
    pub static ref ACCENT2: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_allowed_in_text(true);
        props.set_allowed_in_math(true);
        props.set_arg_types(vec![ArgType::primitive]);

        FunctionDefSpec {
            def_type: "accent".to_string(),
            names: vec![
                "\\'".to_string(),
                "\\`".to_string(),
                "\\^".to_string(),
                "\\~".to_string(),
                "\\=".to_string(),
                "\\u".to_string(),
                "\\.".to_string(),
                "\\\"".to_string(),
                "\\c".to_string(),
                "\\r".to_string(),
                "\\H".to_string(),
                "\\v".to_string(),
                "\\textcircled".to_string(),
            ],
            props,
            handler: handler_fn_2,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
