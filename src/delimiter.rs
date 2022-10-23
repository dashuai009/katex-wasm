use crate::dom_tree::path_node::PathNode;
use crate::tree::HtmlDomNode;
/**
 * This file deals with creating delimiters of various sizes. The TeXbook
 * discusses these routines on page 441-442, in the "Another subroutine sets box
 * x to a specified variable delimiter" paragraph.
 *
 * There are three main routines here. `makeSmallDelim` makes a delimiter in the
 * normal font, but in either text, script, or scriptscript style.
 * `makeLargeDelim` makes a delimiter in textstyle, but in one of the Size1,
 * Size2, Size3, or Size4 fonts. `makeStackedDelim` makes a delimiter out of
 * smaller pieces that are stacked on top of one another.
 *
 * The functions take a parameter `center`, which determines if the delimiter
 * should be centered around the axis.
 *
 * Then, there are three exposed functions. `sizedDelim` makes a delimiter in
 * one of the given sizes. This is used for things like `\bigl`.
 * `customSizedDelim` makes a delimiter with a given total height+depth. It is
 * called in places like `\sqrt`. `leftRightDelim` makes an appropriate
 * delimiter which surrounds an expression of a given height an depth. It is
 * used in `\left` and `\right`.
 */
use crate::dom_tree::span::Span;
use crate::dom_tree::svg_node::SvgNode;
use crate::metrics::public::CharacterMetrics;
use crate::types::Mode;
use crate::Options::Options;
use crate::{get_character_metrics, get_symbol, make_em, sqrt_path, StyleInterface, VirtualNode};
use std::collections::HashMap;
use std::sync::RwLock;

/**
 * Get the metrics for a given symbol and font, after transformation (i.e.
 * after following replacement from symbols.js)
 */
fn get_metrics(symbol: &str, font: &str, mode: Mode) -> CharacterMetrics {
    let replace = if let Some(sym) = get_symbol(Mode::math, symbol) {
        sym.replace.unwrap_or(symbol.to_string())
    } else {
        symbol.to_string()
    };
    if let Some(metrics) = get_character_metrics(&replace, font, mode) {
        return metrics;
    } else {
        panic!("Unsupported symbol {symbol} and font size {font}.");
    }
}

// /**
//  * Puts a delimiter span in a given style, and adds appropriate height, depth,
//  * and maxFontSizes.
//  */
// let styleWrap = function(
//     delim: HtmlDomNode,
//     toStyle: StyleInterface,
//     options: Options,
//     classes: String[],
// ): DomSpan {
// let newOptions = options.havingBaseStyle(toStyle);
//
// let span = buildCommon.makeSpan(
//     classes.concat(newOptions.sizingClasses(options)),
//     [delim], options);
//
// let delimSizeMultiplier =
//     newOptions.sizeMultiplier / options.sizeMultiplier;
// span.height *= delimSizeMultiplier;
// span.depth *= delimSizeMultiplier;
// span.maxFontSize = newOptions.sizeMultiplier;
//
// return span;
// };
//
// let centerSpan = function(
//     span: DomSpan,
//     options: Options,
//     style: StyleInterface,
// ) {
// let newOptions = options.havingBaseStyle(style);
// let shift =
//     (1 - options.sizeMultiplier / newOptions.sizeMultiplier) *
//         options.fontMetrics().axisHeight;
//
// span.classes.push("delimcenter");
// span.style.top = makeEm(shift);
// span.height -= shift;
// span.depth += shift;
// };
//
// /**
//  * Makes a small delimiter. This is a delimiter that comes in the Main-Regular
//  * font, but is restyled to either be in textstyle, scriptstyle, or
//  * scriptscriptstyle.
//  */
// let makeSmallDelim = function(
//     delim: String,
//     style: StyleInterface,
//     center: bool,
//     options: Options,
//     mode: Mode,
//     classes: String[],
// ): DomSpan {
// let text = buildCommon.makeSymbol(delim, "Main-Regular", mode, options);
// let span = styleWrap(text, style, options, classes);
// if (center) {
// centerSpan(span, options, style);
// }
// return span;
// };
//
// /**
//  * Builds a symbol in the given font size (note size is an integer)
//  */
// let mathrmSize = function(
//     value: String,
//     size: f64,
//     mode: Mode,
//     options: Options,
// ): SymbolNode {
// return buildCommon.makeSymbol(value, "Size" + size + "-Regular",
// mode, options);
// };
//
// /**
//  * Makes a large delimiter. This is a delimiter that comes in the Size1, Size2,
//  * Size3, or Size4 fonts. It is always rendered in textstyle.
//  */
// let makeLargeDelim = function(delim,
//                                 size: f64,
//                                 center: bool,
//                                 options: Options,
//                                 mode: Mode,
//                                 classes: String[],
// ): DomSpan {
// let inner = mathrmSize(delim, size, mode, options);
// let span = styleWrap(
//     buildCommon.makeSpan(["delimsizing", "size" + size], [inner], options),
//     Style.TEXT, options, classes);
// if (center) {
// centerSpan(span, options, Style.TEXT);
// }
// return span;
// };
//
// /**
//  * Make a span from a font glyph with the given offset and in the given font.
//  * This is used in makeStackedDelim to make the stacking pieces for the delimiter.
//  */
// let makeGlyphSpan = function(
//     symbol: String,
//     font: "Size1-Regular" | "Size4-Regular",
//     mode: Mode,
// ): VListElem {
// let sizeClass;
// // Apply the correct CSS class to choose the right font.
// if (font == "Size1-Regular") {
// sizeClass = "delim-size1";
// } else /* if (font == "Size4-Regular") */ {
// sizeClass = "delim-size4";
// }
//
// let corner = buildCommon.makeSpan(
// ["delimsizinginner", sizeClass],
// [buildCommon.makeSpan([], [buildCommon.makeSymbol(symbol, font, mode)])]);
//
// // Since this will be passed into `makeVList` in the end, wrap the element
// // in the appropriate tag that VList uses.
// return {type: "elem", elem: corner};
// };
//
// let makeInner = function(
//     ch: String,
//     height: f64,
//     options: Options,
// ): VListElem {
// // Create a span with inline SVG for the inner part of a tall stacked delimiter.
// let width = fontMetricsData['Size4-Regular'][ch.charCodeAt(0)]
// ? fontMetricsData['Size4 - Regular'][ch.charCodeAt(0)][4]
// : fontMetricsData['Size1 - Regular'][ch.charCodeAt(0)][4];
// let path = new PathNode("inner", innerPath(ch,  Math.round(1000 * height)));
// let svgNode = new SvgNode([path], {
// "width": makeEm(width),
// "height": makeEm(height),
// // Override CSS rule `.katex svg { width: 100% }`
// "style": "width:" + makeEm(width),
// "viewBox": "0 0 " + 1000 * width + " " + Math.round(1000 * height),
// "preserveAspectRatio": "xMinYMin",
// });
// let span = buildCommon.makeSvgSpan([], [svgNode], options);
// span.height = height;
// span.style.height = makeEm(height);
// span.style.width = makeEm(width);
// return {type: "elem", elem: span};
// };
//
// // Helpers for makeStackedDelim
// let lapInEms = 0.008;
// let lap = { type : "kern", size: -1 * lapInEms };
// let verts = ["|", "\\lvert", "\\rvert", "\\vert"];
// let doubleVerts = ["\\|", "\\lVert", "\\rVert", "\\Vert"];
//
// /**
//  * Make a stacked delimiter out of a given delimiter, with the total height at
//  * least `heightTotal`. This routine is mentioned on page 442 of the TeXbook.
//  */
// let makeStackedDelim = function(
//     delim: String,
//     heightTotal: f64,
//     center: bool,
//     options: Options,
//     mode: Mode,
//     classes: String[],
// ): DomSpan {
// // There are four parts, the top, an optional middle, a repeated part, and a
// // bottom.
// let top;
// let middle;
// let repeat;
// let bottom;
// let svgLabel = "";
// let viewBoxWidth = 0;
// top = repeat = bottom = delim;
// middle = null;
// // Also keep track of what font the delimiters are in
// let font = "Size1-Regular";
//
// // We set the parts and font based on the symbol. Note that we use
// // '\u23d0' instead of '|' and '\u2016' instead of '\\|' for the
// // repeats of the arrows
// if (delim == "\\uparrow") {
// repeat = bottom = "\u23d0";
// } else if (delim == "\\Uparrow") {
// repeat = bottom = "\u2016";
// } else if (delim == "\\downarrow") {
// top = repeat = "\u23d0";
// } else if (delim == "\\Downarrow") {
// top = repeat = "\u2016";
// } else if (delim == "\\updownarrow") {
// top = "\\uparrow";
// repeat = "\u23d0";
// bottom = "\\downarrow";
// } else if (delim == "\\Updownarrow") {
// top = "\\Uparrow";
// repeat = "\u2016";
// bottom = "\\Downarrow";
// } else if (utils.contains(verts, delim)) {
// repeat = "\u2223";
// svgLabel = "vert";
// viewBoxWidth = 333;
// } else if (utils.contains(doubleVerts, delim)) {
// repeat = "\u2225";
// svgLabel = "doublevert";
// viewBoxWidth = 556;
// } else if (delim == "[" | | delim == "\\lbrack") {
// top = "\u23a1";
// repeat = "\u23a2";
// bottom = "\u23a3";
// font = "Size4-Regular";
// svgLabel = "lbrack";
// viewBoxWidth = 667;
// } else if (delim == "]" | | delim == "\\rbrack") {
// top = "\u23a4";
// repeat = "\u23a5";
// bottom = "\u23a6";
// font = "Size4-Regular";
// svgLabel = "rbrack";
// viewBoxWidth = 667;
// } else if (delim == "\\lfloor" | | delim == "\u230a") {
// repeat = top = "\u23a2";
// bottom = "\u23a3";
// font = "Size4-Regular";
// svgLabel = "lfloor";
// viewBoxWidth = 667;
// } else if (delim == "\\lceil" | | delim == "\u2308") {
// top = "\u23a1";
// repeat = bottom = "\u23a2";
// font = "Size4-Regular";
// svgLabel = "lceil";
// viewBoxWidth = 667;
// } else if (delim == "\\rfloor" | | delim == "\u230b") {
// repeat = top = "\u23a5";
// bottom = "\u23a6";
// font = "Size4-Regular";
// svgLabel = "rfloor";
// viewBoxWidth = 667;
// } else if (delim == "\\rceil" | | delim == "\u2309") {
// top = "\u23a4";
// repeat = bottom = "\u23a5";
// font = "Size4-Regular";
// svgLabel = "rceil";
// viewBoxWidth = 667;
// } else if (delim == "(" | | delim == "\\lparen") {
// top = "\u239b";
// repeat = "\u239c";
// bottom = "\u239d";
// font = "Size4-Regular";
// svgLabel = "lparen";
// viewBoxWidth = 875;
// } else if (delim == ")" | | delim == "\\rparen") {
// top = "\u239e";
// repeat = "\u239f";
// bottom = "\u23a0";
// font = "Size4-Regular";
// svgLabel = "rparen";
// viewBoxWidth = 875;
// } else if (delim == "\\{" | | delim == "\\lbrace") {
// top = "\u23a7";
// middle = "\u23a8";
// bottom = "\u23a9";
// repeat = "\u23aa";
// font = "Size4-Regular";
// } else if (delim == "\\}" | | delim == "\\rbrace") {
// top = "\u23ab";
// middle = "\u23ac";
// bottom = "\u23ad";
// repeat = "\u23aa";
// font = "Size4-Regular";
// } else if (delim == "\\lgroup" | | delim == "\u27ee") {
// top = "\u23a7";
// bottom = "\u23a9";
// repeat = "\u23aa";
// font = "Size4-Regular";
// } else if (delim == "\\rgroup" | | delim == "\u27ef") {
// top = "\u23ab";
// bottom = "\u23ad";
// repeat = "\u23aa";
// font = "Size4-Regular";
// } else if (delim == "\\lmoustache" | | delim == "\u23b0") {
// top = "\u23a7";
// bottom = "\u23ad";
// repeat = "\u23aa";
// font = "Size4-Regular";
// } else if (delim == "\\rmoustache" | | delim == "\u23b1") {
// top = "\u23ab";
// bottom = "\u23a9";
// repeat = "\u23aa";
// font = "Size4-Regular";
// }
//
// // Get the metrics of the four sections
// let topMetrics = get_metrics(top, font, mode);
// let topHeightTotal = topMetrics.height + topMetrics.depth;
// let repeatMetrics = get_metrics(repeat, font, mode);
// let repeatHeightTotal = repeatMetrics.height + repeatMetrics.depth;
// let bottomMetrics = get_metrics(bottom, font, mode);
// let bottomHeightTotal = bottomMetrics.height + bottomMetrics.depth;
// let middleHeightTotal = 0;
// let middleFactor = 1;
// if (middle != = null) {
// let middleMetrics = get_metrics(middle, font, mode);
// middleHeightTotal = middleMetrics.height + middleMetrics.depth;
// middleFactor = 2; // repeat symmetrically above and below middle
// }
//
// // Calcuate the minimal height that the delimiter can have.
// // It is at least the size of the top, bottom, and optional middle combined.
// let minHeight = topHeightTotal + bottomHeightTotal + middleHeightTotal;
//
// // Compute the f64 of copies of the repeat symbol we will need
// let repeatCount = Math.max(0, Math.ceil(
//     (heightTotal - minHeight) / (middleFactor * repeatHeightTotal)));
//
// // Compute the total height of the delimiter including all the symbols
// let realHeightTotal =
//     minHeight + repeatCount * middleFactor * repeatHeightTotal;
//
// // The center of the delimiter is placed at the center of the axis. Note
// // that in this context, "center" means that the delimiter should be
// // centered around the axis in the current style, while normally it is
// // centered around the axis in textstyle.
// let axisHeight = options.fontMetrics().axisHeight;
// if (center) {
// axisHeight *= options.sizeMultiplier;
// }
// // Calculate the depth
// let depth = realHeightTotal / 2 - axisHeight;
//
// // Now, we start building the pieces that will go into the vlist
// // Keep a list of the pieces of the stacked delimiter
// let stack = [];
//
// if (svgLabel.length > 0) {
// // Instead of stacking glyphs, create a single SVG.
// // This evades browser problems with imprecise positioning of spans.
// let midHeight = realHeightTotal - topHeightTotal - bottomHeightTotal;
// let viewBoxHeight = Math.round(realHeightTotal * 1000);
// let pathStr = tallDelim(svgLabel, Math.round(midHeight * 1000));
// let path = new PathNode(svgLabel, pathStr);
// let width = (viewBoxWidth / 1000).toFixed(3) + "em";
// let height = (viewBoxHeight / 1000).toFixed(3) + "em";
// let svg = new SvgNode([path], {
// "width": width,
// "height": height,
// "viewBox": `0 0 ${viewBoxWidth} ${viewBoxHeight}`,
// });
// let wrapper = buildCommon.makeSvgSpan([], [svg], options);
// wrapper.height = viewBoxHeight / 1000;
// wrapper.style.width = width;
// wrapper.style.height = height;
// stack.push({type: "elem", elem: wrapper});
// } else {
// // Stack glyphs
// // Start by adding the bottom symbol
// stack.push(makeGlyphSpan(bottom, font, mode));
// stack.push(lap); // overlap
//
// if (middle == null) {
// // The middle section will be an SVG. Make it an extra 0.016em tall.
// // We'll overlap by 0.008em at top and bottom.
// let innerHeight = realHeightTotal - topHeightTotal - bottomHeightTotal
// + 2 * lapInEms;
// stack.push(makeInner(repeat, innerHeight, options));
// } else {
// // When there is a middle bit, we need the middle part and two repeated
// // sections
// let innerHeight = (realHeightTotal - topHeightTotal -
// bottomHeightTotal - middleHeightTotal) / 2 + 2 * lapInEms;
// stack.push(makeInner(repeat, innerHeight, options));
// // Now insert the middle of the brace.
// stack.push(lap);
// stack.push(makeGlyphSpan(middle, font, mode));
// stack.push(lap);
// stack.push(makeInner(repeat, innerHeight, options));
// }
//
// // Add the top symbol
// stack.push(lap);
// stack.push(makeGlyphSpan(top, font, mode));
// }
//
// // Finally, build the vlist
// let newOptions = options.havingBaseStyle(Style.TEXT);
// let inner = buildCommon.makeVList({
//                                         positionType: "bottom",
//                                         positionData: depth,
//                                         children: stack,
//                                     }, newOptions);
//
// return styleWrap(
// buildCommon.makeSpan(["delimsizing", "mult"], [inner], newOptions),
// Style.TEXT, options, classes);
// };
//
// All surds have 0.08em padding above the viniculum inside the SVG.
// That keeps browser span height rounding error from pinching the line.
const VB_PAD: f64 = 80.0;
// padding above the surd, measured inside the viewBox.
const EM_PAD: f64 = 0.08;

// padding, in ems, measured in the document.
//
fn sqrt_svg(
    sqrtName: String,
    height: f64,
    viewBoxHeight: f64,
    extraViniculum: f64,
    options: &Options,
) -> Span {
    let path = sqrt_path(&sqrtName, extraViniculum, viewBoxHeight);
    let pathNode = PathNode::new(sqrtName, Some(path));

    let svg = SvgNode::new(vec![Box::new(pathNode) as Box<dyn VirtualNode>], HashMap::from([
    // Note: 1000:1 ratio of viewBox to document em width.
                                                             ("width".to_string(), "400em".to_string()),
                                                             ("height".to_string(), make_em(height)),
                                                              ("viewBox".to_string(), format!("0 0 400000 {}" , viewBoxHeight)),
                                                              ("preserveAspectRatio".to_string(), "xMinYMin slice".to_string())

    ]

    )
    );

    return Span::new(vec!["hide-tail".to_string()], vec![Box::new(svg) as Box<dyn HtmlDomNode>], Some(options.clone()), Default::default());
}

//
/**
 * Make a sqrt image of the given height,
 * return (img, advance_width, rule_width)
 */
pub fn make_sqrt_image(height: f64, options: &Options) -> (Span, f64, f64) {
    // Define a new_options that removes the effect of size changes such as \Huge.
    // We don't pick different a height surd for \Huge. For it, we scale up.
    let new_options = options.having_base_sizing();

    // Pick the desired surd glyph from a sequence of surds.
    let stack_large_delimiter_sequence = STACK_LARGE_DELIMITER_SEQUEUE.read().unwrap();
    let delim = traverse_sequence(
        &"\\surd".to_string(),
        height * new_options.sizeMultiplier,
        stack_large_delimiter_sequence.clone(),
        &new_options,
    );

    let mut size_multiplier = new_options.sizeMultiplier; // default

    // The standard sqrt SVGs each have a 0.04em thick viniculum.
    // If Settings.minRuleThickness is larger than that, we add extra_viniculum.
    let extra_viniculum = f64::max(
        0.0,
        options.minRuleThickness - options.get_font_metrics().sqrtRuleThickness,
    );

    // Create a span containing an SVG image of a sqrt symbol.
    let mut span;
    let mut span_height = 0.0;
    let mut tex_height = 0.0;
    let mut view_box_height = 0.0;
    let advance_width;

    // We create viewBoxes with 80 units of "padding" above each surd.
    // Then browser rounding error on the parent span height will not
    // encroach on the ink of the viniculum. But that padding is not
    // included in the TeX-like `height` used for calculation of
    // vertical alignment. So tex_height = span.height < span.style.height.

    match delim {
        Delimiter::Small(s) => {
            // Get an SVG that is derived from glyph U+221A in font KaTeX-Main.
            // 1000 unit normal glyph height.
            view_box_height = 1000.0 + 1000.0 * extra_viniculum + VB_PAD;
            if (height < 1.0) {
                size_multiplier = 1.0; // mimic a \textfont radical
            } else if (height < 1.4) {
                size_multiplier = 0.7; // mimic a \scriptfont radical
            }
            span_height = (1.0 + extra_viniculum + EM_PAD) / size_multiplier;
            tex_height = (1.0 + extra_viniculum) / size_multiplier;
            span = sqrt_svg(
                "sqrtMain".to_string(),
                span_height,
                view_box_height,
                extra_viniculum,
                options,
            );
            span.get_mut_style().min_width = Some("0.853em".to_string());
            advance_width = 0.833 / size_multiplier; // from the font.
        }
        Delimiter::Large(size) => {
            // These SVGs come from fonts: KaTeX_Size1, _Size2, etc.
            view_box_height = (1000.0 + VB_PAD) * SIZE_TO_MAX_HEIGHT[size];
            tex_height = (SIZE_TO_MAX_HEIGHT[size] + extra_viniculum) / size_multiplier;
            span_height = (SIZE_TO_MAX_HEIGHT[size] + extra_viniculum + EM_PAD) / size_multiplier;
            span = sqrt_svg(
                format!("sqrtSize{}", size),
                span_height,
                view_box_height,
                extra_viniculum,
                options,
            );
            span.get_mut_style().min_width = Some("1.02em".to_string());
            advance_width = 1.0 / size_multiplier; // 1.0 from the font.
        }
        Delimiter::Stack => {
            // Tall sqrt. In TeX, this would be stacked using multiple glyphs.
            // We'll use a single SVG to accomplish the same thing.
            span_height = height + extra_viniculum + EM_PAD;
            tex_height = height + extra_viniculum;
            view_box_height = f64::floor(1000.0 * height + extra_viniculum) + VB_PAD;
            span = sqrt_svg(
                "sqrtTall".to_string(),
                span_height,
                view_box_height,
                extra_viniculum,
                options,
            );
            span.get_mut_style().min_width = Some("0.742em".to_string());
            advance_width = 1.056;
        }
    };

    span.set_height(tex_height);
    span.get_mut_style().height = Some(make_em(span_height));

    return (
        span,
        advance_width,
        // Calculate the actual line width.
        // This actually should depend on the chosen font -- e.g. \boldmath
        // should use the thicker surd symbols from e.g. KaTeX_Main-Bold, and
        // have thicker rules.
        (options.get_font_metrics().sqrtRuleThickness + extra_viniculum) * size_multiplier,
    );
}

//
// // There are three kinds of delimiters, delimiters that stack when they become
// // too large
// let stackLargeDelimiters = [
//     "(", "\\lparen", ")", "\\rparen",
//     "[", "\\lbrack", "]", "\\rbrack",
//     "\\{", "\\lbrace", "\\}", "\\rbrace",
//     "\\lfloor", "\\rfloor", "\u230a", "\u230b",
//     "\\lceil", "\\rceil", "\u2308", "\u2309",
//     "\\surd",
// ];
//
// // delimiters that always stack
// let stackAlwaysDelimiters = [
//     "\\uparrow", "\\downarrow", "\\updownarrow",
//     "\\Uparrow", "\\Downarrow", "\\Updownarrow",
//     "|", "\\|", "\\vert", "\\Vert",
//     "\\lvert", "\\rvert", "\\lVert", "\\rVert",
//     "\\lgroup", "\\rgroup", "\u27ee", "\u27ef",
//     "\\lmoustache", "\\rmoustache", "\u23b0", "\u23b1",
// ];
//
// // and delimiters that never stack
// let stackNeverDelimiters = [
//     "<", ">", "\\langle", "\\rangle", "/", "\\backslash", "\\lt", "\\gt",
// ];
//

// Metrics of the different sizes. Found by looking at TeX's output of
// $\bigl| // \Bigl| \biggl| \Biggl| \showlists$
// Used to create stacked delimiters of appropriate sizes in makeSizedDelim.
const SIZE_TO_MAX_HEIGHT: [f64; 5] = [0.0, 1.2, 1.8, 2.4, 3.0];

//
// /**
//  * Used to create a delimiter of a specific size, where `size` is 1, 2, 3, or 4.
//  */
// let makeSizedDelim = function(
//     delim: String,
//     size: f64,
//     options: Options,
//     mode: Mode,
//     classes: String[],
// ): DomSpan {
// // < and > turn into \langle and \rangle in delimiters
// if (delim == "<" | | delim == "\\lt" | | delim == "\u27e8") {
// delim = "\\langle";
// } else if (delim == ">" | | delim == "\\gt" | | delim == "\u27e9") {
// delim = "\\rangle";
// }
//
// // Sized delimiters are never centered.
// if (utils.contains(stackLargeDelimiters, delim) | |
// utils.contains(stackNeverDelimiters, delim)) {
// return makeLargeDelim(delim, size, false, options, mode, classes);
// } else if (utils.contains(stackAlwaysDelimiters, delim)) {
// return makeStackedDelim(
// delim, SIZE_TO_MAX_HEIGHT[size], false, options, mode, classes);
// } else {
// throw new ParseError("Illegal delimiter: '" + delim + "'");
// }
// };
//
// /**
//  * There are three different sequences of delimiter sizes that the delimiters
//  * follow depending on the kind of delimiter. This is used when creating custom
//  * sized delimiters to decide whether to create a small, large, or stacked
//  * delimiter.
//  *
//  * In real TeX, these sequences aren't explicitly defined, but are instead
//  * defined inside the font metrics. Since there are only three sequences that
//  * are possible for the delimiters that TeX defines, it is easier to just encode
//  * them explicitly here.
//  */
//
#[derive(Clone)]
enum Delimiter {
    Small(StyleInterface),
    Large(usize),
    Stack,
}

// // Delimiters that never stack try small delimiters and large delimiters only
// let stackNeverDelimiterSequence = [
//     { type : "small", style: Style.SCRIPTSCRIPT },
//     { type : "small", style: Style.SCRIPT },
//     { type : "small", style: Style.TEXT },
//     { type : "large", size: 1 },
//     { type : "large", size: 2 },
//     { type : "large", size: 3 },
//     { type : "large", size: 4 },
// ];
//
// // Delimiters that always stack try the small delimiters first, then stack
// let stackAlwaysDelimiterSequence = [
//     { type : "small", style: Style.SCRIPTSCRIPT },
//     { type : "small", style: Style.SCRIPT },
//     { type : "small", style: Style.TEXT },
//     { type : "stack" },
// ];
//
// Delimiters that stack when large try the small and then large delimiters, and
// stack afterwards
lazy_static! {
    static ref STACK_LARGE_DELIMITER_SEQUEUE: RwLock<Vec<Delimiter>> = RwLock::new({
        let _scriptscript = crate::Style::SCRIPTSCRIPT.read().unwrap();
        let _script = crate::Style::SCRIPT.read().unwrap();
        let _text = crate::Style::TEXT.read().unwrap();
        let res = vec![
            Delimiter::Small(_scriptscript.clone()),
            Delimiter::Small(_script.clone()),
            Delimiter::Small(_text.clone()),
            Delimiter::Large(1),
            Delimiter::Large(2),
            Delimiter::Large(3),
            Delimiter::Large(4),
            Delimiter::Stack,
        ];
        res
    });
}

/**
 * Get the font used in a delimiter based on what kind of delimiter it is.
 * TODO(#963) Use more specific font family return type once that is introduced.
 */
fn delim_type_to_font(t: &Delimiter) -> String {
    return match t {
        Delimiter::Small(_) => "Main-Regular".to_string(),
        Delimiter::Large(size) => {
            format!("Size{size}-Regular")
        }
        Delimiter::Stack => "Size4-Regular".to_string(),
    };
}

/**
 * Traverse a sequence of types of delimiters to decide what kind of delimiter
 * should be used to create a delimiter of the given height+depth.
 */
fn traverse_sequence(
    delim: &str,
    height: f64,
    sequence: Vec<Delimiter>,
    options: &Options,
) -> Delimiter {
    // Here, we choose the index we should start at in the sequences. In smaller
    // sizes (which correspond to larger f64s in style.size) we start earlier
    // in the sequence. Thus, scriptscript starts at index 3-3=0, script starts
    // at index 3-2=1, text starts at 3-1=2, and display starts at min(2,3-0)=2
    let start = f64::min(2.0, 3.0 - options.get_style().size as f64) as usize;

    for s in sequence[start..].iter() {
        match s {
            Delimiter::Small(style) => {
                let metrics = get_metrics(delim, &delim_type_to_font(s), Mode::math);
                let mut heightDepth = metrics.height + metrics.depth;

                // Small delimiters are scaled down versions of the same font, so we
                // account for the style change size.

                let newOptions = options.having_base_style(style);
                heightDepth *= newOptions.sizeMultiplier;

                // Check if the delimiter at this size works for the given height.
                if heightDepth > height {
                    return s.clone();
                }
            }
            Delimiter::Large(large) => {
                let metrics = get_metrics(delim, &delim_type_to_font(s), Mode::math);
                let heightDepth = metrics.height + metrics.depth;

                // Check if the delimiter at this size works for the given height.
                if heightDepth > height {
                    return s.clone();
                }
            }
            Delimiter::Stack => {
                // This is always the last delimiter, so we just break the loop now.
                break;
            }
        }
    }

    // If we reached the end of the sequence, return the last sequence element.
    return sequence.last().unwrap().clone();
}

/**
 * Make a delimiter of a given height+depth, with optional centering. Here, we
 * traverse the sequences, and create a delimiter that the sequence tells us to.
 */
pub fn make_custom_sized_delim(
    delim: &String,
    height: f64,
    center: bool,
    options: Options,
    mode: Mode,
    classes: Vec<String>,
) -> Span {
    panic!("undefined make custom sized delim");
    // if delim == "<" || delim == "\\lt" || delim == "\u{27e8}" {
    // delim = "\\langle";
    // } else if delim == ">" || delim == "\\gt" || delim == "\u{27e9}" {
    // delim = "\\rangle";
    // }
    //
    // // Decide what sequence to use
    // let sequence;
    // if (utils.contains(stackNeverDelimiters, delim)) {
    // sequence = stackNeverDelimiterSequence;
    // } else if (utils.contains(stackLargeDelimiters, delim)) {
    // sequence = stackLargeDelimiterSequence;
    // } else {
    // sequence = stackAlwaysDelimiterSequence;
    // }
    //
    // // Look through the sequence
    // let delimType = traverseSequence(delim, height, sequence, options);
    //
    // // Get the delimiter from font glyphs.
    // // Depending on the sequence element we decided on, call the
    // // appropriate function.
    // if (delimType.type == "small") {
    // return makeSmallDelim(delim, delimType.style, center, options,
    // mode, classes);
    // } else if (delimType.type == "large") {
    // return makeLargeDelim(delim, delimType.size, center, options, mode,
    // classes);
    // } else /* if (delimType.type == "stack") */ {
    // return makeStackedDelim(delim, height, center, options, mode,
    // classes);
    // }
}
//
// /**
//  * Make a delimiter for use with `\left` and `\right`, given a height and depth
//  * of an expression that the delimiters surround.
//  */
// let makeLeftRightDelim = function(
//     delim: String,
//     height: f64,
//     depth: f64,
//     options: Options,
//     mode: Mode,
//     classes: String[],
// ): DomSpan {
// // We always center \left/\right delimiters, so the axis is always shifted
// let axisHeight =
//     options.fontMetrics().axisHeight * options.sizeMultiplier;
//
// // Taken from TeX source, tex.web, function make_left_right
// let delimiterFactor = 901;
// let delimiterExtend = 5.0 / options.fontMetrics().ptPerEm;
//
// let maxDistFromAxis = Math.max(
//     height - axisHeight, depth + axisHeight);
//
// let totalHeight = Math.max(
//     // In real TeX, calculations are done using integral values which are
//     // 65536 per pt, or 655360 per em. So, the division here truncates in
//     // TeX but doesn't here, producing different results. If we wanted to
//     // exactly match TeX's calculation, we could do
//     //   Math.floor(655360 * maxDistFromAxis / 500) *
//     //    delimiterFactor / 655360
//     // (To see the difference, compare
//     //    x^{x^{\left(\rule{0.1em}{0.68em}\right)}}
//     // in TeX and KaTeX)
//     maxDistFromAxis / 500 * delimiterFactor,
//     2 * maxDistFromAxis - delimiterExtend);
//
// // Finally, we defer to `make_custom_sized_delim` with our calculated total
// // height
// return make_custom_sized_delim(delim, totalHeight, true, options, mode, classes);
// };
//
// export default {
// sqrtImage: makeSqrtImage,
// sizedDelim: makeSizedDelim,
// SIZE_TO_MAX_HEIGHT: SIZE_TO_MAX_HEIGHT,
// customSizedDelim: make_custom_sized_delim,
// leftRightDelim: makeLeftRightDelim,
// };
