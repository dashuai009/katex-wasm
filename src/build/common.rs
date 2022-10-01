use crate::dom_tree::anchor::Anchor;
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::path_node::PathNode;
use crate::dom_tree::svg_node::SvgNode;
use crate::dom_tree::{span::Span, symbol_node::SymbolNode};
use crate::metrics::public::CharacterMetrics;
use crate::parse_node::types::{AnyParseNode, GetMode, GetText, ParseNodeToAny};
use crate::symbols::public::Font;
use crate::symbols::LIGATURES;
use crate::tree::{HtmlDomNode, VirtualNode};
use crate::types::{FontVariant, Mode};
use crate::units::make_em;
use crate::wideCharacter::{wideCharacterFont, wide_character_font};
use crate::Options::Options;
use crate::{get_character_metrics, get_symbol};
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
    let tmp_metrics = get_character_metrics(&value, font_name, mode);

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
        if opt.style().isTight() {
            symbol_node.push_class("mtight".to_string());
        }
        let color = opt.getColor();
        // if (color) {
        symbol_node.set_style_color(Some(color));
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
    if (_type != "textord"
        && lookup_symbol(value, "Math-BoldItalic".to_string(), mode)
            .metrics
            .is_some())
    {
        return ["Math-BoldItalic", "boldsymbol"];
    } else {
        // Some glyphs do not exist in Math-BoldItalic so we need to use
        // Main-Bold instead.
        return ["Main-Bold", "mathbf"];
    }
}

/**
 * Makes either a mathord or textord in the correct font and color.
 */
// pub fn make_ord<T:GetMode + GetText>(
//     group: T,
//     options: Options,
//     _type: String,
// )->impl VirtualNode/*  HtmlDocumentFragment | SymbolNode */{
//     let mode = group.get_mode();
//     let text = group.get_text();

//     let classes = ["mord".to_string()];

//     // Math mode or Old font (i.e. \rm)
//     let isFont = mode == Mode::math || (mode == Mode::text && options.font != "");
//     let fontOrFamily = if isFont { options.font }else  {options.fontFamily};
//     let code_point = text.chars().nth(0).unwrap() as usize;
//     if code_point == 0xD835 {
//         // surrogate pairs get special treatment
//         let [wideFontName, wideFontClass] = wide_character_font(text, mode).unwrap();
//         return make_symbol(text, wideFontName.to_string(), mode, options,
//             [classes,[wideFontClass.to_string()]].concat());
//     } else {//if (fontOrFamily) {
//         let mut fontName = "";
//         let fontClasses:Vec<String> = Vec::new();
//         if (fontOrFamily == "boldsymbol") {
//             let fontData = bold_symbol(text, mode, options, classes, _type);
//             fontName = fontData.fontName;
//             fontClasses.push(fontData.fontClass);
//         } else if (isFont) {
//             let f = FONT_MAP.lock().unwrap();
//             fontName = f.get(fontOrFamily).unwrap();
//             fontClasses = [fontOrFamily];
//         } else {
//             fontName = &retrieve_text_font_name(fontOrFamily, options.fontWeight,
//                                             options.font_shape.unwrap());
//             fontClasses = [fontOrFamily, options.fontWeight, options.fontShape];
//         }

//         if (lookup_symbol(text, fontName.to_string(), mode).metrics.is_some()) {
//             return make_symbol(text, fontName.to_string(), mode, options,
//                 [classes,fontClasses].concat());
//         } else if (LIGATURES.contains(text) &&
//                    fontName[0:10] == "Typewriter") {
//             // Deletruct ligatures in monospace fonts (\texttt, \tt).
//             let parts =  text.chars().map(|c|{
//                 make_symbol(c.to_string(), fontName.to_string(), mode, options,
//                     [classes,fontClasses].concat())
//             });
//             return make_fragment(parts);
//         }
//     }

//     // Makes a symbol in the default font for mathords and textords.
//     if _type == "mathord" {
//         return make_symbol(text, "Math-Italic".to_string(), mode, options,
//             [classes,["mathnormal".to_string()]].concat());
//     } else if (_type == "textord") {
//         if let Some(sym) = get_symbol(mode,&text){
//             match sym.font{
//                 Font::main => todo!(),
//                 Font::ams => todo!(),
//             }
//         }
//         if (font == "ams") {
//             let fontName = retrieve_text_font_name("amsrm".to_string(), options.fontWeight,
//                   options.fontShape().unwrap());
//             return make_symbol(
//                 text, fontName, mode, Some(&options),
//                 [classes,["amsrm".to_string(), options.fontWeight(), options.fontShape()]].concat());
//         } else if (font == "main" || !font) {
//             let fontName = retrieve_text_font_name("textrm".to_string(), options.fontWeight,
//                   options.fontShape().unwrap());
//             return make_symbol(
//                 text, fontName, mode, Some(&options),
//                 [classes,[options.fontWeight(), options.fontShape().unwrap()].concat());
//         } else { // fonts added by plugins
//             let fontName = retrieve_text_font_name(font, options.fontWeight,
//                   options.fontShape());
//             // We add font name as a css class
//             return make_symbol(
//                 text, fontName, mode, Some(&options),
//                 [classes,[fontName, options.fontWeight(), options.fontShape].concat());
//         }
//     }
//     // else {
//     //     throw new Error("unexpected type: " + type + " in makeOrd");
//     // }
// }

// // SVG one is simpler -- doesn't require height, depth, max-font setting.
// // This is also a separate method for typesafety.
// let makeSvgSpan = (
//     classes?: string[],
//     children?: SvgNode[],
//     options?: Options,
//     style?: CssStyle,
// ): SvgSpan => new Span(classes, children, options, style);

// let makeLineSpan = function(
//     className: string,
//     options: Options,
//     thickness?: number,
// ): DomSpan {
//     let line = makeSpan([className], [], options);
//     line.height = Math.max(
//         thickness || options.fontMetrics().defaultRuleThickness,
//         options.minRuleThickness,
//     );
//     line.style.borderBottomWidth = makeEm(line.height);
//     line.maxFontSize = 1.0;
//     return line;
// };

/**
 * Combine consecutive domTree.symbolNodes into a single symbolNode.
 * Note: this function mutates the argument.
 */
pub fn try_combine_chars(mut chars: Vec<Box<dyn HtmlDomNode>>) -> Vec<Box<dyn HtmlDomNode>> {
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
// fn make_fragment<T:HtmlDomNode>(
//     children: Vec<T>,
// )-> DocumentFragment<T> {
//     let fragment =  DocumentFragment::new(children);

//     size_element_from_children(&mut fragment);

//     return fragment;
// }

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
        shift: Option<i32>, //only for individual_shift
    },
    Kern {
        size: i32,
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
    position_type: PositionType,
    children: Vec<VListChild>,
    position_data: Option<i32>,
}

// Computes the updated `children` list and the overall depth.
//
// This helper function for makeVList makes it easier to enforce type safety by
// allowing early exits (returns) in the logic.
pub fn get_vlist_children_and_depth(params: VListParam) -> (Vec<VListChild>, i32) {
    let mut depth = 0;
    match params.position_type {
        PositionType::IndividualShift => {
            // Add in kerns to the list of params.children to get each element to be
            // shifted to the correct specified shift
            let mut children_iter = params.children.iter();
            let mut pre_child = children_iter.next().unwrap();
            let mut children = vec![pre_child.clone()];
            depth = match pre_child {
                VListChild::Elem {
                    elem,
                    margin_left,
                    margin_right,
                    wrapper_classes,
                    wrapper_style,
                    shift,
                } => shift.unwrap() - elem.get_depth() as i32,
                VListChild::Kern { size } => unreachable!(),
            };
            let mut currPos = depth;
            for cur_child in children_iter {
                match cur_child {
                    VListChild::Elem {
                        elem,
                        margin_left,
                        margin_right,
                        wrapper_classes,
                        wrapper_style,
                        shift,
                    } => {
                        let diff = -(shift.unwrap()) - currPos - elem.get_depth() as i32;
                        let size = match pre_child {
                            VListChild::Elem {
                                elem,
                                margin_left,
                                margin_right,
                                wrapper_classes,
                                wrapper_style,
                                shift,
                            } => diff - (elem.get_height() + elem.get_depth()) as i32,
                            VListChild::Kern { size } => todo!(),
                        };

                        currPos = currPos + diff;
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
                    VListChild::Elem {
                        elem,
                        margin_left,
                        margin_right,
                        wrapper_classes,
                        wrapper_style,
                        shift,
                    } => (elem.get_height() - elem.get_depth()) as i32,
                    VListChild::Kern { size } => *size,
                };
            }
            depth = bottom;
        }
        PositionType::Bottom => {
            depth = -params.position_data.unwrap();
        }
        PositionType::Shift | PositionType::FirstBaseline => match &params.children[0] {
            VListChild::Elem {
                elem,
                margin_left,
                margin_right,
                wrapper_classes,
                wrapper_style,
                shift,
            } => {
                depth = match params.position_type {
                    PositionType::Shift => -elem.get_depth() as i32 - params.position_data.unwrap(),
                    PositionType::FirstBaseline => elem.get_depth() as i32,
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

// /**
//  * Makes a vertical list by stacking elements and kerns on top of each other.
//  * Allows for many different ways of specifying the positioning method.
//  *
//  * See VListParam documentation above.
//  */
// pub fn make_vlist(params: VListParam, options: Options)-> DomSpan {
//     let (children, depth) = get_vlist_children_and_depth(params);
//     // Create a strut that is taller than any list item. The strut is added to
//     // each item, where it will determine the item's baseline. Since it has
//     // `overflow:hidden`, the strut's top edge will sit on the item's line box's
//     // top edge and the strut's bottom edge will sit on the item's baseline,
//     // with no additional line-height spacing. This allows the item baseline to
//     // be positioned precisely without worrying about font ascent and
//     // line-height.
//     let pstrutSize = 0;
//     for (let i = 0; i < children.length; i++) {
//         let child = children[i];
//         if (child.type === "elem") {
//             let elem = child.elem;
//             pstrutSize = Math.max(pstrutSize, elem.maxFontSize, elem.height);
//         }
//     }
//     pstrutSize += 2;
//     let pstrut = makeSpan(["pstrut"], []);
//     pstrut.style.height = makeEm(pstrutSize);
//     // Create a new list of actual children at the correct offsets
//     let realChildren = [];
//     let minPos = depth;
//     let maxPos = depth;
//     let currPos = depth;
//     for (let i = 0; i < children.length; i++) {
//         let child = children[i];
//         if (child.type === "kern") {
//             currPos += child.size;
//         } else {
//             let elem = child.elem;
//             let classes = child.wrapperClasses || [];
//             let style = child.wrapperStyle || {};
//             let childWrap = makeSpan(classes, [pstrut, elem], undefined, style);
//             childWrap.style.top = makeEm(-pstrutSize - currPos - elem.depth);
//             if (child.marginLeft) {
//                 childWrap.style.marginLeft = child.marginLeft;
//             }
//             if (child.marginRight) {
//                 childWrap.style.marginRight = child.marginRight;
//             }
//             realChildren.push(childWrap);
//             currPos += elem.height + elem.depth;
//         }
//         minPos = Math.min(minPos, currPos);
//         maxPos = Math.max(maxPos, currPos);
//     }
//     // The vlist contents go in a table-cell with `vertical-align:bottom`.
//     // This cell's bottom edge will determine the containing table's baseline
//     // without overly expanding the containing line-box.
//     let vlist = makeSpan(["vlist"], realChildren);
//     vlist.style.height = makeEm(maxPos);
//     // A second row is used if necessary to represent the vlist's depth.
//     let rows;
//     if (minPos < 0) {
//         // We will define depth in an empty span with display: table-cell.
//         // It should render with the height that we define. But Chrome, in
//         // contenteditable mode only, treats that span as if it contains some
//         // text content. And that min-height over-rides our desired height.
//         // So we put another empty span inside the depth strut span.
//         let emptySpan = makeSpan([], []);
//         let depthStrut = makeSpan(["vlist"], [emptySpan]);
//         depthStrut.style.height = makeEm(-minPos);
//         // Safari wants the first row to have inline content; otherwise it
//         // puts the bottom of the *second* row on the baseline.
//         let topStrut = makeSpan(["vlist-s"], [new SymbolNode("\u200b")]);
//         rows = [makeSpan(["vlist-r"], [vlist, topStrut]),
//             makeSpan(["vlist-r"], [depthStrut])];
//     } else {
//         rows = [makeSpan(["vlist-r"], [vlist])];
//     }
//     let vtable = makeSpan(["vlist-t"], rows);
//     if (rows.length === 2) {
//         vtable.classes.push("vlist-t2");
//     }
//     vtable.height = maxPos;
//     vtable.depth = -minPos;
//     return vtable;
// };

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

// Takes font options, and returns the appropriate fontLookup name
fn retrieve_text_font_name(font_family: String, font_weight: String, font_shape: String) -> String {
    let base_font_name = match font_family.as_str() {
        "amsrm" => "AMS",
        "textrm" => "Main",
        "textsf" => "SansSerif",
        "texttt" => "Typewriter",
        _ => &font_family, // use fonts added by a plugin
    };

    let font_styles_name;
    if (font_weight == "textbf" && font_shape == "textit") {
        font_styles_name = "BoldItalic";
    } else if (font_weight == "textbf") {
        font_styles_name = "Bold";
    } else if (font_weight == "textit") {
        font_styles_name = "Italic";
    } else {
        font_styles_name = "Regular";
    }

    return format!("{base_font_name}-{font_styles_name}");
}

/**
 * Maps TeX font commands to objects containing:
 * - variant: string used for "mathvariant" attribute in buildMathML.js
 * - fontName: the "style" parameter to fontMetrics.getCharacterMetrics
 */
struct FontInfo {
    variant: FontVariant,
    fontName: &'static str,
}
// A map between tex font commands an MathML mathvariant attribute values
lazy_static! {
    static ref FONT_MAP: std::sync::Mutex< HashMap<&'static str, FontInfo> >= std::sync::Mutex::new({
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
    let svgNode = SvgNode {
        children: vec![Box::new(path)],
        attributes: HashMap::from([
            ("widdth".to_string(), make_em(*width)),
            ("height".to_string(), make_em(*height)),
            // Override CSS rule `.katex svg { width: 100% }`
            ("style".to_string(), format!("width:{}", make_em(*width))),
            (
                "viewBox".to_string(),
                format!("0 0 {} {}", 1000.0 * width, 1000.0 * height),
            ),
            ("preserveAspectRatio".to_string(), "xMinYMin".to_string()),
        ]),
    };
    let mut span = Span::new(
        vec!["overlay".to_string()],
        vec![/*Box::new(svgNode) as Box<dyn HtmlDomNode>*/],
        Some(options),
        CssStyle::new(),
    );
    // span.height = height;
    // span.style.height = make_em(height);
    // span.style.width = make_em(width);
    return span;
}
