use crate::build::common::{make_span, make_vlist, PositionType, VListChild, VListParam};
use crate::build::HTML::{DomType, Side};
use crate::define::functions::{FunctionDefSpec, FunctionPropSpec};
use crate::dom_tree::symbol_node::SymbolNode;
use crate::mathML_tree::public::MathDomNode;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{build, make_em, parse_node, AnyParseNode, HtmlDomNode};
use std::any::Any;
use std::f64;
use std::sync::Mutex;

/**
 * Sometimes, groups perform special rules when they have superscripts or
 * subscripts attached to them. This function lets the `supsub` group know that
 * Sometimes, groups perform special rules when they have superscripts or
 * its inner element should handle the superscripts and subscripts instead of
 * handling them itself.
 */
fn html_builder_delegate(
    group: &parse_node::types::supsub,
    options: &Options,
) -> Option<fn(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode>> {
    let style = crate::Style::DISPLAY.lock().unwrap();
    if let Some(base) = &group.base {
        if let Some(op) = base.as_any().downcast_ref::<parse_node::types::op>() {
            // Operators handle supsubs differently when they have limits
            // (e.g. `\displaystyle\sum_2^3`)
            let delegate =
                op.limits && (options.get_style().size == style.size || op.alwaysHandleSupSub);
            return if delegate {
                panic!("op::htmlBuilder");
            } else {
                None
            };
        } else if let Some(op_name) = base
            .as_any()
            .downcast_ref::<parse_node::types::operatorname>()
        {
            let delegate = op_name.alwaysHandleSupSub
                && (options.get_style().size == style.size || op_name.limits);
            return if delegate {
                panic!("operatorname::htmlBuilder")
            } else {
                None
            };
        } else if let Some(ac) = base.as_any().downcast_ref::<parse_node::types::accent>() {
            return if is_character_box(&(ac.base.as_ref().unwrap())) {
                panic!("accent::htmlBuilder")
            } else {
                None
            };
        } else if let Some(hb) = base
            .as_any()
            .downcast_ref::<parse_node::types::horizBrace>()
        {
            panic!("h B")
            // let isSup = !group.sub;
            // return if isSup === base.isOver ? horizBrace::htmlBuilder : null;
        } else {
            return None;
        }
    } else {
        return None;
    }
}

fn supsub_html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    // Superscript and subscripts are handled in the TeXbook on page
    // 445-446, rules 18(a-f).
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::supsub>()
        .unwrap();
    // Here is where we defer to the inner group if it should handle
    // superscripts and subscripts itself.
    if let Some(builder_delegate) = html_builder_delegate(&group, &options) {
        return builder_delegate(_group, options);
    }
    let parse_node::types::supsub {
        base: value_base,
        sup: value_sup,
        sub: value_sub,
        ..
    } = group;
    let base = crate::build::HTML::build_group(value_base.clone(), options.clone(), None);
    println!("base mathord HtmlDomNode = {:#?}", base);
    let mut _supm = None;
    let mut _subm = None;

    let metrics = options.get_font_metrics();

    // Rule 18a
    let mut sup_shift = 0.0;
    let mut subShift = 0.0;

    let value_base_is_character_box = value_base.is_some() && is_character_box(&value_base.as_ref().unwrap());
    if value_sup.is_some() {
        let new_options = options.having_style(&options.get_style().sup());
        _supm = Some(crate::build::HTML::build_group(
            value_sup.clone(),
            new_options.clone(),
            Some(options.clone()),
        ));
        if !value_base_is_character_box {
            sup_shift = base.get_height()
                - new_options.get_font_metrics().supDrop * new_options.sizeMultiplier
                    / options.sizeMultiplier;
        }
    }

    if value_sub.is_some() {
        let new_options = options.having_style(&options.get_style().sub());
        _subm = Some(crate::build::HTML::build_group(
            value_sub.clone(),
            new_options.clone(),
            Some(options.clone()),
        ));
        if !value_base_is_character_box {
            subShift = base.get_depth()
                + new_options.get_font_metrics().subDrop * new_options.sizeMultiplier.clone()
                    / options.sizeMultiplier.clone();
        }
    }

    let display_style = crate::Style::DISPLAY.lock().unwrap();
    // Rule 18c
    let min_sup_shift;
    if options.get_style() == display_style.clone() {
        min_sup_shift = metrics.sup1;
    } else if options.get_style().cramped {
        min_sup_shift = metrics.sup3;
    } else {
        min_sup_shift = metrics.sup2;
    }

    // scriptspace is a font-size-independent size, so scale it
    // appropriately for use as the marginRight.
    let multiplier = options.sizeMultiplier;
    let marginRight = make_em((0.5 / metrics.ptPerEm) / multiplier);

    let mut marginLeft = None;
    if _subm.is_some() {
        // Subscripts shouldn't be shifted by the base's italic correction.
        // Account for that by shifting the subscript back the appropriate
        // amount. Note we only do this when the base is a single symbol.
        let is_oiint = if let Some(b) = &group.base {
            if let Some(op) = b.as_any().downcast_ref::<parse_node::types::op>() {
                if let Some(name) = &op.name {
                    name == "\\oiint" || name == "\\oiiint"
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };
        if let Some(b) = base.as_any().downcast_ref::<SymbolNode>() {
            marginLeft = Some(make_em(-b.italic));
        } else {
            if is_oiint {
                panic!("emmmmm base type = ");
            }
        }
    }

    let supsub;
    if let Some(supm) = _supm {
        if let Some(subm) = _subm {
            sup_shift = f64::max(
                sup_shift,
                f64::max(min_sup_shift, supm.get_depth() + 0.25 * metrics.xHeight),
            );
            subShift = f64::max(subShift, metrics.sub2);

            let ruleWidth = metrics.defaultRuleThickness;

            // Rule 18e
            let maxWidth = 4.0 * ruleWidth;
            if ((sup_shift - supm.get_depth()) - (subm.get_height() - subShift) < maxWidth) {
                subShift = maxWidth - (sup_shift - supm.get_depth()) + subm.get_height();
                let psi = 0.8 * metrics.xHeight - (sup_shift - supm.get_depth());
                if (psi > 0.0) {
                    sup_shift += psi;
                    subShift -= psi;
                }
            }

            let vlistElem = [
                VListChild::Elem {
                    elem: subm,
                    margin_left: marginLeft,
                    margin_right: None,
                    wrapper_classes: None,
                    wrapper_style: None,
                    shift: Some(subShift ),
                },
                VListChild::Elem {
                    elem: supm,
                    margin_left: None,
                    margin_right: Some(marginRight),
                    wrapper_classes: None,
                    wrapper_style: None,
                    shift: Some(-sup_shift),
                },
            ];
            panic!("make_v_list");

            // supsub = crate::build::common::make_VList({
            //                                    positionType: "individualShift",
            //                                    children: vlistElem,
            //                                }, options);
        } else {
            // Rule 18c, d
            sup_shift = f64::max(
                f64::max(sup_shift, min_sup_shift),
                supm.get_depth() + 0.25 * metrics.xHeight,
            );
            supsub = make_vlist(
                VListParam {
                    position_type: PositionType::Shift,
                    children: vec![VListChild::Elem {
                        elem: supm,
                        margin_left: None,
                        margin_right: Some(marginRight),
                        wrapper_classes: None,
                        wrapper_style: None,
                        shift: None,
                    }],
                    position_data: Some( - sup_shift),
                },
                options.clone(),
            );
        }
    } else {
        if let Some(subm) = _subm {
            // Rule 18b
            subShift = f64::max(
                f64::max(subShift, metrics.sub1),
                subm.get_height() - 0.8 * metrics.xHeight,
            );
            panic!("make_v_list");

            // let vlistElem =
            //     [{type: "elem", elem: subm, marginLeft, marginRight}];
            //
            // supsub = buildCommon.makeVList({
            //                                    positionType: "shift",
            //                                    positionData: subShift,
            //                                    children: vlistElem,
            //                                }, options);
        } else {
            panic!("supsub must have either sup or sub.");
        }
    }

    // Wrap the supsub vlist in a span.msupsub to reset text-align.
    let mclass = crate::build::HTML::get_type_of_dom_tree(&mut base.clone(), Some(Side::Right))
        .unwrap_or(DomType::mord);
    return Box::new(make_span(
        vec![mclass.as_str().to_string()],
        vec![
            base,
            Box::new(make_span(
                vec!["msupsub".to_string()],
                vec![Box::new(supsub) as Box<dyn HtmlDomNode>],
                None,
                Default::default(),
            )) as Box<dyn HtmlDomNode>,
        ],
        Some(&options),
        Default::default(),
    )) as Box<dyn HtmlDomNode>;
}

fn supsub_mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::supsub>()
        .unwrap();
    panic!("mathml_Builder");
    // Is the inner group a relevant horizonal brace?
    // let isBrace = false;
    // let isOver;
    // let isSup;
    //
    // if (group.base && group.base.type === "horizBrace") {
    //     isSup = !!group.sup;
    //     if (isSup === group.base.isOver) {
    //         isBrace = true;
    //         isOver = group.base.isOver;
    //     }
    // }
    //
    // if (group.base &&
    //     (group.base.type === "op" || group.base.type === "operatorname")) {
    // group.base.parentIsSupSub = true;
    // }
    //
    // let children = [mml.buildGroup(group.base, options)];
    //
    // if (group.sub) {
    //     children.push(mml.buildGroup(group.sub, options));
    // }
    //
    // if (group.sup) {
    //     children.push(mml.buildGroup(group.sup, options));
    // }
    //
    // let nodeType: MathNodeType;
    // if (isBrace) {
    //     nodeType = (isOver ? "mover" : "munder");
    // } else if (!group.sub) {
    //     let base = group.base;
    //     if (base && base.type === "op" && base.limits &&
    //         (options.style === Style.DISPLAY || base.alwaysHandleSupSub)) {
    //         nodeType = "mover";
    //     } else if (base && base.type === "operatorname" &&
    //         base.alwaysHandleSupSub &&
    //         (base.limits || options.style === Style.DISPLAY)) {
    //         nodeType = "mover";
    //     } else {
    //         nodeType = "msup";
    //     }
    // } else if (!group.sup) {
    //     let base = group.base;
    //     if (base && base.type === "op" && base.limits &&
    //         (options.style === Style.DISPLAY || base.alwaysHandleSupSub)) {
    //         nodeType = "munder";
    //     } else if (base && base.type === "operatorname" &&
    //         base.alwaysHandleSupSub &&
    //         (base.limits || options.style === Style.DISPLAY)) {
    //         nodeType = "munder";
    //     } else {
    //         nodeType = "msub";
    //     }
    // } else {
    //     let base = group.base;
    //     if (base && base.type === "op" && base.limits &&
    //         options.style === Style.DISPLAY) {
    //         nodeType = "munderover";
    //     } else if (base && base.type === "operatorname" &&
    //         base.alwaysHandleSupSub &&
    //         (options.style === Style.DISPLAY || base.limits)) {
    //         nodeType = "munderover";
    //     } else {
    //         nodeType = "msubsup";
    //     }
    // }
    //
    // return new mathMLTree.MathNode(nodeType, children);
}
// Super scripts and subscripts, whose precise placement can depend on other
// functions that precede them.

lazy_static! {
    pub static ref SUPSUB: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();

        FunctionDefSpec {
            def_type: "supsub".to_string(),
            names: vec![],
            props,
            handler: |a, b, c| panic!("error"),
            html_builder: Some(supsub_html_builder),
            mathml_builder: None,
        }
    });
}
