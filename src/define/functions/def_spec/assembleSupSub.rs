// For an operator with limits, assemble the base, sup, and sub into a span.

use crate::build::common::{PositionType, VListChild, VListParam};
use crate::build::{common, HTML};
use crate::dom_tree::span::Span;
use crate::parse_node::types::lap;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{make_em, AnyParseNode, HtmlDomNode, StyleInterface};

pub fn assemble_sup_sub(
    _base: &Box<dyn HtmlDomNode>,
    supGroup: &Option<Box<dyn AnyParseNode>>,
    subGroup: &Option<Box<dyn AnyParseNode>>,
    options: &Options,
    style: &StyleInterface,
    slant: f64,
    baseShift: f64,
) -> Span {
    let base = common::make_span(vec![], vec![_base.clone()], None, Default::default());
    let subIsSingleCharacter = subGroup.is_some() && is_character_box(&subGroup.as_ref().unwrap());
    let mut _sub: Option<(Box<dyn HtmlDomNode>, f64)> = None;
    let mut _sup: Option<(Box<dyn HtmlDomNode>, f64)> = None;
    // We manually have to handle the superscripts and subscripts. This,
    // aside from the kern calculations, is copied from supsub.
    if supGroup.is_some() {
        let elem = HTML::build_group(
            supGroup.clone(),
            options.having_style(&style.sup()),
            Some(options.clone()),
        );

        let elem_depth = elem.get_depth();
        _sup = Some((
            elem,
            f64::max(
                options.get_font_metrics().bigOpSpacing1,
                options.get_font_metrics().bigOpSpacing3 - elem_depth,
            ),
        ));
    }

    if subGroup.is_some() {
        let elem = HTML::build_group(
            subGroup.clone(),
            options.having_style(&style.sub()),
            Some(options.clone()),
        );

        let elem_height = elem.get_height();
        _sub = Some((
            elem,
            f64::max(
                options.get_font_metrics().bigOpSpacing2,
                options.get_font_metrics().bigOpSpacing4 - elem_height,
            ),
        ));
    }

    // Build the final group as a vlist of the possible subscript, base,
    // and possible superscript.
    let finalGroup;
    if let Some(sup) = _sup {
        if let Some(sub) = _sub {
            let bottom = options.get_font_metrics().bigOpSpacing5
                + sub.0.get_height()
                + sub.0.get_depth()
                + sub.1
                + base.get_depth()
                + baseShift;

            finalGroup = common::make_vlist(
                VListParam {
                    position_type: PositionType::Bottom,
                    position_data: Some(bottom),
                    children: vec![
                        VListChild::Kern {
                            size: options.get_font_metrics().bigOpSpacing5,
                        },
                        VListChild::Elem {
                            elem: sub.0,
                            margin_left: Some(make_em(-slant)),
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: None,
                        },
                        VListChild::Kern { size: sub.1 },
                        VListChild::Elem {
                            elem: Box::new(base) as Box<dyn HtmlDomNode>,
                            margin_left: None,
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: None,
                        },
                        VListChild::Kern { size: sup.1 },
                        VListChild::Elem {
                            elem: sup.0,
                            margin_left: Some(make_em(slant)),
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: None,
                        },
                        VListChild::Kern {
                            size: options.get_font_metrics().bigOpSpacing5,
                        },
                    ],
                }
            );
        } else {
            let bottom = base.get_depth() + baseShift;

            finalGroup = common::make_vlist(
                VListParam {
                    position_type: PositionType::Bottom,
                    position_data: Some(bottom),
                    children: vec![
                        VListChild::Elem {
                            elem: Box::new(base) as Box<dyn HtmlDomNode>,
                            margin_left: None,
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: None,
                        },
                        VListChild::Kern { size: sup.1 },
                        VListChild::Elem {
                            elem: sup.0,
                            margin_left: Some(make_em(slant)),
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: None,
                        },
                        VListChild::Kern {
                            size: options.get_font_metrics().bigOpSpacing5,
                        },
                    ],
                }
            );
        }
    } else {
        if let Some(sub) = _sub {
            let top = base.get_height() - baseShift;

            // Shift the limits by the slant of the symbol. Note
            // that we are supposed to shift the limits by 1/2 of the slant,
            // but since we are centering the limits adding a full slant of
            // margin will shift by 1/2 that.
            finalGroup = common::make_vlist(
                VListParam {
                    position_type: PositionType::Top,
                    position_data: Some(top),
                    children: vec![
                        VListChild::Kern {
                            size: options.get_font_metrics().bigOpSpacing5,
                        },
                        VListChild::Elem {
                            elem: sub.0,
                            margin_left: Some(make_em(-slant)),
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: None,
                        },
                        VListChild::Kern { size: sub.1 },
                        VListChild::Elem {
                            elem: Box::new(base) as Box<dyn HtmlDomNode>,
                            margin_left: None,
                            margin_right: None,
                            wrapper_classes: None,
                            wrapper_style: None,
                            shift: None,
                        },
                    ],
                }
            );
        } else {
            // This case probably shouldn't occur (this would mean the
            // supsub was sending us a group with no superscript or
            // subscript) but be safe.
            return base;
        }
    }

    let mut parts = vec![Box::new(finalGroup) as Box<dyn HtmlDomNode>];
    if subGroup.is_some() && slant != 0.0 && !subIsSingleCharacter {
        // A negative margin-left was applied to the lower limit.
        // Avoid an overlap by placing a spacer on the left on the group.
        let mut spacer = common::make_span(
            vec!["mspace".to_string()],
            vec![],
            Some(options),
            Default::default(),
        );
        spacer.get_mut_style().margin_right = Some(make_em(slant));
        parts.insert(0, Box::new(spacer) as Box<dyn HtmlDomNode>);
    }
    return common::make_span(
        vec!["mop".to_string(), "op-limits".to_string()],
        parts,
        Some(options),
        Default::default(),
    );
}
