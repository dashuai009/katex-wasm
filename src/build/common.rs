use crate::dom_tree::anchor::Anchor;
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::path_node::PathNode;
use crate::dom_tree::svg_node::SvgNode;
use crate::dom_tree::{span::Span, symbol_node::SymbolNode};
use crate::metrics::public::CharacterMetrics;
use crate::parse_node::types::{AnyParseNode, GetMode, GetText};
use crate::symbols::public::Font;
use crate::symbols::LIGATURES;
use crate::tree::{DocumentFragment, HtmlDomNode, VirtualNode};
use crate::types::{FontVariant, Mode};
use crate::units::make_em;
use crate::wideCharacter::{wideCharacterFont, wide_character_font};
use crate::Options::Options;
use crate::{get_character_metrics, get_symbol};
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
 */
pub fn lookup_symbol(
    value: String,
    // TODO(#963): Use a union type for this.
    font_name: String,
    mode: Mode,
) -> TmpSymbol {
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

#[wasm_bindgen]
pub fn _lookup_symbol(
    value: String,
    // TODO(#963): Use a union type for this.
    font_name: String,
    mode: String,
) -> TmpSymbol {
    return lookup_symbol(value, font_name, Mode::from_str(mode.as_str()).unwrap());
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
        symbol_node.max_font_size = opt.sizeMultiplier;
        if (opt.style().isTight()) {
            symbol_node.push_class("mtight".to_string());
        }
        let color = opt.getColor();
        // if (color) {
        symbol_node.set_style_color(Some(color));
        // }
    }

    return symbol_node;
}

#[wasm_bindgen]
pub fn canCombine(prev: &SymbolNode, next: &SymbolNode) -> bool {
    return SymbolNode::can_combine(prev, next);
}
#[wasm_bindgen]
pub fn MakeSymbol(
    value: String,
    font_name: String,
    _mode: String,
    options: &Options,
    classes: js_sys::Array,
) -> SymbolNode {
    let mut c = vec![];
    for cl in classes.to_vec().iter() {
        if let Some(t) = cl.as_string() {
            c.push(t);
        }
    }
    let mode = Mode::from_str(_mode.as_str()).unwrap();
    make_symbol(value, font_name, mode, Some(options), c)
}

#[wasm_bindgen]
pub fn MakeSymbol_none(
    value: String,
    font_name: String,
    _mode: String,
    classes: js_sys::Array,
) -> SymbolNode {
    let mut c = vec![];
    for cl in classes.to_vec().iter() {
        //classes fontShape 可能是undefined
        if let Some(t) = cl.as_string() {
            c.push(t);
        }
    }
    let mode = Mode::from_str(_mode.as_str()).unwrap();
    make_symbol(value, font_name, mode, None, c)
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
//             // Deconstruct ligatures in monospace fonts (\texttt, \tt).
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

/**
 * Makes a span with the given list of classes, list of children, and options.
 *
 * TODO(#953): Ensure that `options` is always provided (currently some call
 * sites don't pass it) and make the type below mandatory.
 * TODO: add a separate argument for math class (e.g. `mop`, `mbin`), which
 * should if present come first in `classes`.
 */
pub fn make_span<T: HtmlDomNode>(
    classes: Vec<String>,
    children: Vec<T>,
    options: Options,
    style: CssStyle,
) -> Span<T> {
    let mut span = Span::new(classes, children, Some(options), style);

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

pub fn static_svg(value: String, options: Options) -> Span<SvgNode> {
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
        vec![svgNode],
        Some(options),
        CssStyle::new(),
    );
    // span.height = height;
    // span.style.height = make_em(height);
    // span.style.width = make_em(width);
    return span;
}
