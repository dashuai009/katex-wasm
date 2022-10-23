use crate::dom_tree::anchor::Anchor;
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::document_fragment::DocumentFragment;
use crate::dom_tree::path_node::PathNode;
use crate::dom_tree::svg_node::SvgNode;
use crate::dom_tree::{span::Span, symbol_node::SymbolNode};
use crate::metrics::public::CharacterMetrics;
use crate::parse_node::types::{AnyParseNode, ParseNodeToAny};
use crate::symbols::public::Font;
use crate::symbols::LIGATURES;
use crate::tree::{HtmlDomNode, VirtualNode};
use crate::types::{FontVariant, Mode};
use crate::units::make_em;
use crate::wideCharacter::wide_character_font;
use crate::Options::Options;
use crate::{get_character_metrics, get_symbol, parse_node};
use std::any::Any;
use std::collections::HashMap;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct TmpSymbol {
    pub value: String,
    pub metrics: Option<CharacterMetrics>,
}

/**
 * Looks up the given symbol in fontMetrics, after applying any symbol
 * replacements defined in symbol.js
 * TODO(#963): Use a union type for font_name.
 */
pub fn lookup_symbol(value: String, font_name: String, mode: Mode) -> TmpSymbol {
    // Replace the value with its replaced value from symbol.js
    let tmp_metrics = get_character_metrics(&value, &font_name, mode);

    // if tmp.is_some_and(|&t| t.replace.is_some()) {}
    if let Some(tmp) = get_symbol(mode, &value) {
        if let Some(tmp_replace) = tmp.replace {
            return TmpSymbol {
                value: tmp_replace,
                metrics: tmp_metrics,
            };
        }
    }

    return TmpSymbol {
        value: value,
        metrics: tmp_metrics,
    };
}

/**
 * Makes a symbolNode after translation via the list of symbols in symbols.js.
 * Correctly pulls out metrics for the character, and optionally takes a list of
 * classes to be attached to the node.
 *
 * TODO: make argument order closer to makeSpan
 * TODO: add a separate argument for math class (e.g. `mop`, `mbin`), which
 * should if present come first in `classes`.
 * TODO(#953): Make `options` mandatory and always pass it in.
 */
pub fn make_symbol(
    value: String,
    font_name: String,
    mode: Mode,
    options: Option<&Options>,
    classes: Vec<String>,
) -> SymbolNode {
    let lookup = lookup_symbol(value, font_name, mode);
    let value = lookup.value;

    let mut symbol_node = SymbolNode::new("init_node".to_string());
    if let Some(metrics) = lookup.metrics {
        let mut italic = metrics.italic;
        if let Some(opt) = options.clone() {
            if opt.font == "mathit" {
                italic = 0.0;
            }
        }
        if mode == Mode::text {
            italic = 0.0;
        }
        symbol_node = SymbolNode::new(value);
        symbol_node.height = metrics.height;
        symbol_node.depth = metrics.depth;
        symbol_node.italic = italic;
        symbol_node.skew = metrics.skew;
        symbol_node.width = metrics.width;
        symbol_node.set_classes(classes);
    } else {
        // TODO(emily): Figure out a good way to only print this in development
        //         typeof console !== "undefined" && console.warn("No character metrics " +
        //                                                        `for '${value}' in style '${fontName}' and mode '${mode}'`);

        symbol_node = SymbolNode::new(value);
        symbol_node.height = 0.0;
        symbol_node.depth = 0.0;
        symbol_node.italic = 0.0;
        symbol_node.skew = 0.0;
        symbol_node.width = 0.0;
        symbol_node.set_classes(classes);
    }

    if let Some(opt) = options {
        symbol_node.max_font_size = opt.sizeMultiplier.clone();
        if opt.get_style().isTight() {
            symbol_node.get_mut_classes().push("mtight".to_string());
        }
        let color = opt.get_color();
        // if (color) {
        symbol_node.get_mut_style().color = color;
        // }
    }

    return symbol_node;
}

/**
 * Makes a symbol in Main-Regular or AMS-Regular.
 * Used for rel, bin, open, close, inner, and punct.
 */
pub fn math_sym(value: String, mode: Mode, options: Options, classes: Vec<String>) -> SymbolNode {
    // Decide what font to render the symbol in by its entry in the symbols
    // table.
    // Have a special case for when the value = \ because the \ is used as a
    // textord in unsupported command errors but cannot be parsed as a regular
    // text ordinal and is therefore not present as a symbol in the symbols
    // table for text, as well as a special case for boldsymbol because it
    // can be used for bold + and -
    if options.font == "boldsymbol"
        && lookup_symbol(value.clone(), "Main-Bold".to_string(), mode)
            .metrics
            .is_some()
    {
        return make_symbol(
            value.clone(),
            "Main-Bold".to_string(),
            mode,
            Some(&options),
            [classes, vec!["mathbf".to_string()]].concat(),
        );
    } else if value == "\\" || get_symbol(mode, &value).unwrap().font == Font::main {
        return make_symbol(
            value,
            "Main-Regular".to_string(),
            mode,
            Some(&options),
            classes,
        );
    } else {
        return make_symbol(
            value,
            "AMS-Regular".to_string(),
            mode,
            Some(&options),
            [classes, vec!["amsrm".to_string()]].concat(),
        );
    }
}

/**
 * Determines which of the two font names (Main-Bold and Math-BoldItalic) and
 * corresponding style tags (mathbf or boldsymbol) to use for font "boldsymbol",
 * depending on the symbol.  Use this function instead of fontMap for font
 * "boldsymbol".
 */
pub fn bold_symbol(
    value: String,
    mode: Mode,
    options: Options,
    classes: Vec<String>,
    _type: String,
) -> [&'static str; 2] /*{ fontName: String, fontClass: String }*/ {
    if _type != "textord"
        && lookup_symbol(value, "Math-BoldItalic".to_string(), mode)
            .metrics
            .is_some()
    {
        return ["Math-BoldItalic", "boldsymbol"];
    } else {
        // Some glyphs do not exist in Math-BoldItalic so we need to use
        // Main-Bold instead.
        return ["Main-Bold", "mathbf"];
    }
}

impl Options {}
/**
 * Makes either a mathord or textord in the correct font and color.
 */
pub fn make_ord(
    //<NODETYPE: "spacing" | "mathord" | "textord">
    group: Box<dyn AnyParseNode>,
    options: Options,
    _type: String,
) -> SymbolNode {
    let (mode, text) = if let Some(s) = group.as_any().downcast_ref::<parse_node::types::spacing>()
    {
        (s.mode, s.text.clone())
    } else if let Some(m) = group.as_any().downcast_ref::<parse_node::types::mathord>() {
        (m.mode, m.text.clone())
    } else if let Some(t) = group.as_any().downcast_ref::<parse_node::types::textord>() {
        (t.mode, t.text.clone())
    } else {
        panic!("make_ord {}", group.get_type());
    };
    let mut classes = vec!["mord".to_string()];
    // Math mode or Old font (i.e. \rm)
    let is_font = mode == Mode::math || (mode == Mode::text); //TODO && options.font);
    let font_or_family = if is_font {
        options.font.clone()
    } else {
        options.fontFamily.clone()
    };
    if text.clone().chars().nth(0).unwrap() as u32 == 0xD835 {
        // surrogate pairs get special treatment
        let [wide_font_name, wide_font_class] = wide_character_font(&text, mode).unwrap();
        classes.push(wide_font_class.to_string());
        return make_symbol(
            text.clone(),
            wide_font_name.to_string(),
            mode,
            Some(&options),
            classes,
        );
    } else if font_or_family != "" {
        let font_name;
        let font_classes;
        if font_or_family == "boldsymbol" {
            let font_data = bold_symbol(
                text.clone(),
                mode,
                options.clone(),
                classes.clone(),
                _type.clone(),
            );
            font_name = font_data[0].to_string();
            font_classes = vec![font_data[1].to_string()];
        } else if is_font {
            let font_map = FONT_MAP.lock().unwrap();
            font_name = font_map
                .get(&font_or_family.as_str())
                .unwrap()
                .fontName
                .clone()
                .to_string();
            font_classes = vec![font_or_family];
        } else {
            font_name = options.retrieve_text_font_name(font_or_family.clone());
            font_classes = vec![font_or_family, options.fontWeight(), options.fontShape()];
        }
        if lookup_symbol(text.clone(), font_name.clone(), mode)
            .metrics
            .is_some()
        {
            return make_symbol(
                text.clone(),
                font_name.clone(),
                mode,
                Some(&options),
                [classes, font_classes].concat(),
            );
        } else if LIGATURES.contains(&text.as_str()) && font_name.starts_with("Typewriter") {
            // Deconstruct ligatures in monospace fonts (\texttt, \tt).
            let mut parts = vec![];
            for c in text.clone().chars() {
                parts.push(make_symbol(
                    c.to_string(),
                    font_name.clone(),
                    mode,
                    Some(&options),
                    [classes.clone(), font_classes.clone()].concat(),
                ));
            }
            // return makeFragment(parts);
        }
    }
    // Makes a symbol in the default font for mathords and textords.
    if _type == "mathord" {
        return make_symbol(
            text.clone(),
            "Math-Italic".to_string(),
            mode,
            Some(&options),
            vec![classes, vec!["mathnormal".to_string()]].concat(),
        );
    } else if _type == "textord" {
        let font = get_symbol(mode, &text).unwrap().font;
        if font == Font::ams {
            let font_name = options.retrieve_text_font_name("amsrm".to_string());
            return make_symbol(
                text.clone(),
                font_name,
                mode,
                Some(&options),
                vec![
                    classes,
                    vec![
                        "amsrm".to_string(),
                        options.fontWeight(),
                        options.fontShape(),
                    ],
                ]
                .concat(),
            );
        } else if font == Font::main {
            let font_name = options.retrieve_text_font_name("textrm".to_string());
            return make_symbol(
                text.clone(),
                font_name,
                mode,
                Some(&options),
                vec![classes, vec![options.fontWeight(), options.fontShape()]].concat(),
            );
        } else {
            // fonts added by plugins
            let font_name = options.retrieve_text_font_name(font.as_str().to_string());
            // We add font name as a css class
            return make_symbol(
                text.clone(),
                font_name.clone(),
                mode,
                Some(&options),
                [
                    classes,
                    vec![font_name, options.fontWeight(), options.fontShape()],
                ]
                .concat(),
            );
        }
    } else {
        panic!("unexpected type: {} in make_ord", _type);
    }
}

// // SVG one is simpler -- doesn't require height, depth, max-font setting.
// // This is also a separate method for typesafety.
// let makeSvgSpan = (
//     classes?: string[],
//     children?: SvgNode[],
//     options?: Options,
//     style?: CssStyle,
// ): SvgSpan => new Span(classes, children, options, style);

pub fn make_line_span(
    className: String,
    options: &Options,
    thickness:Option<f64>,
)->Span {
    let mut line = make_span(vec![className], vec![], Some(options), Default::default());
    line.set_height(f64::max(
        thickness.unwrap_or(options.get_font_metrics().defaultRuleThickness),
        options.minRuleThickness,
    ));
    line.get_mut_style().border_bottom_width = Some(make_em(line.get_height()));
    line.set_max_font_size(1.0);
    return line;
}

/**
 * Combine consecutive domTree.symbolNodes into a single symbolNode.
 * Note: this function mutates the argument.
 */
pub fn try_combine_chars(mut chars: &Vec<Box<dyn HtmlDomNode>>) -> &Vec<Box<dyn HtmlDomNode>> {
    // let mut res = vec![];
    // let mut pairs = chars.windows(2);
    // while let Some([_prev, _nxt]) = pairs.next() {
    //     if let Some(prev) = _prev.as_any().downcast_ref::<SymbolNode>() {
    //         if let Some(nxt) = _nxt.as_any().downcast_ref::<SymbolNode>() {
    //             let mut x = prev.clone();
    //             x.set_text(format!("{}{}", prev.get_text(), nxt.get_text()));
    //             x.set_height(f64::max(prev.get_height(), nxt.get_height()));
    //             x.set_depth(f64::max(prev.get_depth(), nxt.get_depth()));
    //             // Use the last character's italic correction since we use
    //             // it to add padding to the right of the span created from
    //             // the combined characters.
    //             x.italic = nxt.italic;
    //             res.push(x);
    //             if pairs.next().is_none() {
    //                 //跳走一个
    //                 break;
    //             }
    //         }
    //     }
    // }
    return chars;
}

/**
 * Makes a span with the given list of classes, list of children, and options.
 *
 * TODO(#953): Ensure that `options` is always provided (currently some call
 * sites don't pass it) and make the type below mandatory.
 * TODO: add a separate argument for math class (e.g. `mop`, `mbin`), which
 * should if present come first in `classes`.
 */
pub fn make_span(
    classes: Vec<String>,
    children: Vec<Box<dyn HtmlDomNode>>,
    options: Option<&Options>,
    style: CssStyle,
) -> Span {
    let mut span = Span::new(
        classes,
        children,
        if let Some(o) = options {
            Some(o.clone())
        } else {
            None
        },
        style,
    );

    span.size_element_from_children();

    return span;
}

// pub fn makeLineSpan<T:VirtualNode>(className: String, options: Options, thickness: f64) -> Span<T> {
//     let line = make_span([className], [], options);
//     line.height = f64::max(
//         thickness ,//|| options.fontMetrics().defaultRuleThickness,
//         options.minRuleThickness,
//     );
//     line.style.borderBottomWidth = make_em(line.height);
//     line.maxFontSize = 1.0;
//     return line;
// }

/**
 * Makes an anchor with the given href, list of classes, list of children,
 * and options.
 */
pub fn make_anchor(
    href: String,
    classes: Vec<String>,
    children: Vec<Box<dyn HtmlDomNode>>,
    options: Options,
) -> Anchor {
    let mut anchor = Anchor::new(href, classes, children, options);

    anchor.size_element_from_children();

    return anchor;
}

/**
 * Makes a document fragment with the given list of children.
 */
pub fn make_fragment(children: Vec<Box<dyn HtmlDomNode>>) -> DocumentFragment {
    let mut fragment = DocumentFragment::new(children);

    fragment.size_element_from_children();

    return fragment;
}

/**
 * Wraps group in a span if it's a document fragment, allowing to apply classes
 * and styles
 */
pub fn wrap_fragment(group: Box<dyn HtmlDomNode>, options: &Options) -> Box<dyn HtmlDomNode> {
    return if let Some(g) = group.as_any().downcast_ref::<DocumentFragment>() {
        Box::new(make_span(
            vec![],
            vec![group],
            Some(options),
            Default::default(),
        )) as Box<dyn HtmlDomNode>
    } else {
        group
    };
}

// These are exact object types to catch typos in the names of the optional fields.
#[derive(Clone)]
pub enum VListChild {
    Elem {
        // type: "elem",
        elem: Box<dyn HtmlDomNode>,
        margin_left: Option<String>,
        margin_right: Option<String>,
        wrapper_classes: Option<Vec<String>>,
        wrapper_style: Option<CssStyle>,
        shift: Option<f64>, //only for individual_shift
    },
    Kern {
        size: f64,
    },
}
pub enum PositionType {
    IndividualShift, // Each child contains how much it should be shifted downward.
    Top,             // "top": The positionData specifies the topmost point of the vlist (note this
    //        is expected to be a height, so positive values move up).
    Bottom, // "bottom": The positionData specifies the bottommost point of the vlist (note
    //           this is expected to be a depth, so positive values move down).
    Shift, // "shift": The vlist will be positioned such that its baseline is positionData
    //          away from the baseline of the first child which MUST be an
    //          "elem". Positive values move downwards.
    FirstBaseline, // The vlist is positioned so that its baseline is aligned with the baseline
                   // of the first child which MUST be an "elem". This is equivalent to "shift"
                   // with positionData=0.
}
pub struct VListParam {
    pub(crate) position_type: PositionType,
    pub(crate) children: Vec<VListChild>,
    pub(crate) position_data: Option<f64>,
}

// Computes the updated `children` list and the overall depth.
//
// This helper function for makeVList makes it easier to enforce type safety by
// allowing early exits (returns) in the logic.
pub fn get_vlist_children_and_depth(params: VListParam) -> (Vec<VListChild>, f64) {
    let mut depth = 0.0;
    match params.position_type {
        PositionType::IndividualShift => {
            // Add in kerns to the list of params.children to get each element to be
            // shifted to the correct specified shift
            let mut children_iter = params.children.iter();
            let mut pre_child = children_iter.next().unwrap();
            let mut children = vec![pre_child.clone()];
            depth = match pre_child {
                VListChild::Elem { elem, shift, .. } => -shift.unwrap() - elem.get_depth(),
                VListChild::Kern { size } => unreachable!(),
            } as f64;
            let mut curr_pos = depth;
            for cur_child in children_iter {
                match cur_child {
                    VListChild::Elem { elem, shift, .. } => {
                        let diff = -(shift.unwrap()) - curr_pos - elem.get_depth();
                        let size = match pre_child {
                            VListChild::Elem { elem, .. } => {
                                diff - (elem.get_height() + elem.get_depth())
                            }
                            VListChild::Kern { size } => todo!(),
                        };

                        curr_pos = curr_pos + diff;
                        children.push(VListChild::Kern { size });
                        children.push(cur_child.clone());
                        pre_child = cur_child;
                    }
                    VListChild::Kern { size } => unreachable!(),
                };
            }
            return (children, depth);
        }
        PositionType::Top => {
            // We always start at the bottom, so calculate the bottom by adding up
            // all the sizes
            let mut bottom = params.position_data.unwrap();
            for child in params.children.iter() {
                bottom -= match child {
                    VListChild::Elem { elem, .. } => (elem.get_height() - elem.get_depth()),
                    VListChild::Kern { size } => *size,
                };
            }
            depth = bottom;
        }
        PositionType::Bottom => {
            depth = -params.position_data.unwrap();
        }
        PositionType::Shift | PositionType::FirstBaseline => match &params.children[0] {
            VListChild::Elem { elem, .. } => {
                depth = match params.position_type {
                    PositionType::Shift => -elem.get_depth() - params.position_data.unwrap(),
                    PositionType::FirstBaseline => elem.get_depth(),
                    _ => {
                        panic!("Invalid positionType")
                    }
                };
            }
            VListChild::Kern { size } => {
                panic!("First child must have type \"elem\".")
            }
        },
    }
    return (params.children, depth);
}

/**
 * Makes a vertical list by stacking elements and kerns on top of each other.
 * Allows for many different ways of specifying the positioning method.
 *
 * See VListParam documentation above.
 */
pub fn make_vlist(params: VListParam, options: Options) -> Span {
    let (children, depth) = get_vlist_children_and_depth(params);
    // Create a strut that is taller than any list item. The strut is added to
    // each item, where it will determine the item's baseline. Since it has
    // `overflow:hidden`, the strut's top edge will sit on the item's line box's
    // top edge and the strut's bottom edge will sit on the item's baseline,
    // with no additional line-height spacing. This allows the item baseline to
    // be positioned precisely without worrying about font ascent and
    // line-height.
    let mut pstrut_size: f64 = 0.0;
    for child in children.iter() {
        if let VListChild::Elem { elem, .. } = child {
            pstrut_size = pstrut_size
                .max(elem.get_max_font_size())
                .max(elem.get_height());
        }
    }
    pstrut_size += 2.0;
    let mut pstrut = make_span(
        vec!["pstrut".to_string()],
        vec![],
        None,
        CssStyle::default(),
    );
    pstrut.get_mut_style().height = Some(make_em(pstrut_size));
    // Create a new list of actual children at the correct offsets
    let mut real_children = vec![];
    let mut min_pos: f64 = depth;
    let mut max_pos: f64 = depth;
    let mut curr_pos: f64 = depth;
    // println!("max_pos = {max_pos}, min_pos = {min_pos}");
    for child in children.into_iter() {
        match child {
            VListChild::Elem {
                elem,
                wrapper_classes,
                wrapper_style,
                margin_left,
                margin_right,
                shift,
                ..
            } => {
                let mut child_wrap = make_span(
                    wrapper_classes.unwrap_or(vec![]),
                    vec![
                        Box::new(pstrut.clone()) as Box<dyn HtmlDomNode>,
                        elem.clone(),
                    ],
                    None,
                    wrapper_style.unwrap_or(CssStyle::default()),
                );
                child_wrap.get_mut_style().top =
                    Some(make_em(-pstrut_size - curr_pos - elem.get_depth()));

                child_wrap.get_mut_style().margin_left = margin_left.clone();

                child_wrap.get_mut_style().margin_right = margin_right.clone();

                real_children.push(Box::new(child_wrap) as Box<dyn HtmlDomNode>);
                curr_pos += elem.get_height() + elem.get_depth();
            }
            VListChild::Kern { size } => {
                curr_pos += size;
            }
        }
        min_pos = min_pos.min(curr_pos);
        max_pos = max_pos.max(curr_pos);
        // println!("max_pos = {max_pos}, min_pos = {min_pos}");
    }
    // The vlist contents go in a table-cell with `vertical-align:bottom`.
    // This cell's bottom edge will determine the containing table's baseline
    // without overly expanding the containing line-box.
    let mut vlist = make_span(
        vec!["vlist".to_string()],
        real_children,
        None,
        Default::default(),
    );
    vlist.get_mut_style().height = Some(make_em(max_pos as f64));
    // A second row is used if necessary to represent the vlist's depth.
    let rows;
    if min_pos < 0.0 {
        // We will define depth in an empty span with display: table-cell.
        // It should render with the height that we define. But Chrome, in
        // contenteditable mode only, treats that span as if it contains some
        // text content. And that min-height over-rides our desired height.
        // So we put another empty span inside the depth strut span.
        let empty_span = make_span(vec![], vec![], None, Default::default());
        let mut depth_strut = make_span(
            vec!["vlist".to_string()],
            vec![Box::new(empty_span) as Box<dyn HtmlDomNode>],
            None,
            CssStyle::default(),
        );
        depth_strut.get_mut_style().height = Some(make_em(-min_pos));
        // Safari wants the first row to have inline content; otherwise it
        // puts the bottom of the *second* row on the baseline.
        let top_strut = make_span(
            vec!["vlist-s".to_string()],
            vec![Box::new(SymbolNode::new("\u{200b}".to_string())) as Box<dyn HtmlDomNode>],
            None,
            CssStyle::default(),
        );
        rows = vec![
            Box::new(make_span(
                vec!["vlist-r".to_string()],
                vec![
                    Box::new(vlist) as Box<dyn HtmlDomNode>,
                    Box::new(top_strut) as Box<dyn HtmlDomNode>,
                ],
                None,
                Default::default(),
            )) as Box<dyn HtmlDomNode>,
            Box::new(make_span(
                vec!["vlist-r".to_string()],
                vec![Box::new(depth_strut) as Box<dyn HtmlDomNode>],
                None,
                Default::default(),
            )) as Box<dyn HtmlDomNode>,
        ];
    } else {
        rows = vec![Box::new(make_span(
            vec!["vlist-r".to_string()],
            vec![Box::new(vlist) as Box<dyn HtmlDomNode>],
            None,
            Default::default(),
        )) as Box<dyn HtmlDomNode>];
    }
    let mut vtable = make_span(
        if rows.len() == 2 {
            vec!["vlist-t".to_string(), "vlist-t2".to_string()]
        } else {
            vec!["vlist-t".to_string()]
        },
        rows,
        None,
        Default::default(),
    );
    vtable.set_height(max_pos);
    vtable.set_depth(-min_pos);
    return vtable;
}

/// Glue is a concept from TeX which is a flexible space between elements in
/// either a vertical or horizontal list. In KaTeX, at least for now, it's
/// static space between elements in a horizontal layout.
pub fn make_glue(measurement: crate::units::Measurement, options: &Options) -> Span {
    // Make an empty span for the space
    let mut rule = make_span(
        vec!["mspace".to_string()],
        Vec::<_>::new(),
        Some(&options.clone()),
        CssStyle::default(),
    );
    let size = crate::units::calculate_size(&measurement, &options);
    rule.get_mut_style().margin_right = Some(make_em(size));
    return rule;
}

/**
 * Maps TeX font commands to objects containing:
 * - variant: string used for "mathvariant" attribute in buildMathML.js
 * - fontName: the "style" parameter to fontMetrics.getCharacterMetrics
 */
pub struct FontInfo {
    pub variant: FontVariant,
    pub fontName: &'static str,
}
// A map between tex font commands an MathML mathvariant attribute values
lazy_static! {
    pub static ref FONT_MAP: std::sync::Mutex< HashMap<&'static str, FontInfo> >= std::sync::Mutex::new({
        HashMap::from([
        ("mathbf",FontInfo{
            variant: FontVariant::bold,
            fontName:"Main-Bold"
        }),
        ("mathrm", FontInfo{
            variant:  FontVariant::normal,
            fontName: "Main-Regular",
        }),
        ("textit", FontInfo{
            variant:  FontVariant::italic,
            fontName: "Main-Italic",
        }),
        ("mathit", FontInfo{
            variant:  FontVariant::italic,
            fontName: "Main-Italic",
        }),
        ("mathnormal", FontInfo{
            variant:  FontVariant::italic,
            fontName: "Math-Italic",
        }),

        // "boldsymbol" is missing because they require the use of multiple fonts:
        // Math-BoldItalic and Main-Bold.  This is handled by a special case in
        // makeOrd which ends up calling boldsymbol.

        // families
        ("mathbb", FontInfo{
            variant:  FontVariant::double_struck,
            fontName: "AMS-Regular",
        }),
        ("mathcal", FontInfo{
            variant:  FontVariant::script,
            fontName: "Caligraphic-Regular",
        }),
        ("mathfrak", FontInfo{
            variant:  FontVariant::fraktur,
            fontName: "Fraktur-Regular",
        }),
        ("mathscr", FontInfo{
            variant:  FontVariant::script,
            fontName: "Script-Regular",
        }),
        ("mathsf", FontInfo{
            variant:  FontVariant::sans_serif,
            fontName: "SansSerif-Regular",
        }),
        ("mathtt", FontInfo{
            variant:  FontVariant::monospace,
            fontName: "Typewriter-Regular",
        })
    ])
    });


    static ref SVG_DATA : std::sync::Mutex<HashMap<&'static str,(&'static str,f64,f64)> > = std::sync::Mutex::new({
        HashMap::from([
            //   path, width, height
            ("vec", ("vec", 0.471, 0.714)),               // values from the font glyph
            ("oiintSize1", ("oiintSize1", 0.957, 0.499)),  // oval to overlay the integrand
            ("oiintSize2", ("oiintSize2", 1.472, 0.659)),
            ("oiiintSize1", ("oiiintSize1", 1.304, 0.499)),
            ("oiiintSize2", ("oiiintSize2", 1.98, 0.659))
        ])
    });

}

pub fn static_svg(value: String, options: Options) -> Span {
    // Create a span with inline SVG for the element.
    let svg_data = SVG_DATA.lock().unwrap();
    let (pathName, width, height) = svg_data.get(value.as_str()).unwrap();
    let path = PathNode::new(pathName.to_string(), None);
    let svg_node_attr = HashMap::from([
        ("widdth".to_string(), make_em(*width)),
        ("height".to_string(), make_em(*height)),
        // Override CSS rule `.katex svg { width: 100% }`
        ("style".to_string(), format!("width:{}", make_em(*width))),
        (
            "viewBox".to_string(),
            format!("0 0 {} {}", 1000.0 * width, 1000.0 * height),
        ),
        ("preserveAspectRatio".to_string(), "xMinYMin".to_string()),
    ]);
    let mut tmp = SvgNode::new(vec![Box::new(path)], svg_node_attr);
    let mut span = Span::new(
        vec!["overlay".to_string()],
        vec![Box::new(tmp) as Box<dyn HtmlDomNode>],
        Some(options),
        CssStyle::new(),
    );
    span.set_height(*height);
    span.get_mut_style().height = Some(make_em(*height));
    if *width > 0.0 {
        span.get_mut_style().width = Some(make_em(*width));
    }
    return span;
}
