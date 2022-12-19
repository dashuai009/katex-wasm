use crate::build::common::VListChild::Kern;
use crate::build::common::{PositionType, VListChild};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::path_node::PathNode;
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
use crate::dom_tree::symbol_node::SymbolNode;
use crate::metrics::fontMetricsData::get_char_metrics;
use crate::metrics::public::CharacterMetrics;
use crate::parse_node::types::cr;
use crate::tree::HtmlDomNode;
use crate::types::Mode;
use crate::Options::Options;
use crate::{get_character_metrics, get_symbol, make_em, sqrt_path, StyleInterface, VirtualNode};
use std::collections::HashMap;
use std::f64;
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

/**
 * Puts a delimiter span in a given style, and adds appropriate height, depth,
 * and maxFontSizes.
 */
fn style_wrap(
    delim: Box<dyn HtmlDomNode>,
    to_style: &StyleInterface,
    options: &Options,
    classes: Vec<String>,
) -> Span {
    let new_options = options.having_base_style(to_style);

    let mut span = crate::build::common::make_span(
        [classes, new_options.sizing_classes(options)].concat(),
        vec![delim],
        Some(options),
        Default::default(),
    );

    let delim_size_multiplier = new_options.sizeMultiplier / options.sizeMultiplier;
    span.set_height(span.get_height() * delim_size_multiplier);
    span.set_depth(span.get_depth() * delim_size_multiplier);
    span.set_max_font_size(new_options.sizeMultiplier);

    return span;
}

fn centerSpan(span: Span, options: &Options, style: StyleInterface) -> Span {
    panic!("undefined");
    // let newOptions = options.havingBaseStyle(style);
    // let shift =
    //     (1 - options.sizeMultiplier / newOptions.sizeMultiplier) *
    //         options.fontMetrics().axisHeight;
    //
    // span.classes.push("delimcenter");
    // span.style.top = makeEm(shift);
    // span.height -= shift;
    // span.depth += shift;
}

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
// let span = style_wrap(text, style, options, classes);
// if (center) {
// centerSpan(span, options, style);
// }
// return span;
// };
//
/**
 * Builds a symbol in the given font size (note size is an integer)
 */
fn mathrmSize(value: String, size: f64, mode: Mode, options: &Options) -> SymbolNode {
    return crate::build::common::make_symbol(
        value,
        format!("Size{size}-Regular"),
        mode,
        Some(options),
        vec![],
    );
}

//
/**
 * Makes a large delimiter. This is a delimiter that comes in the Size1, Size2,
 * Size3, or Size4 fonts. It is always rendered in textstyle.
 */
pub fn make_large_delim(
    delim: &String,
    size: f64,
    center: bool,
    options: &Options,
    mode: Mode,
    classes: Vec<String>,
) -> Span {
    let inner = mathrmSize(delim.clone(), size, mode, options);
    let tmp_span = crate::build::common::make_span(
        vec!["delimsizing".to_string(), format!("size{size}")],
        vec![Box::new(inner) as Box<dyn HtmlDomNode>],
        Some(options),
        Default::default(),
    );
    let _text = crate::Style::TEXT.read().unwrap();
    let span = style_wrap(
        Box::new(tmp_span) as Box<dyn HtmlDomNode>,
        &_text,
        options,
        classes,
    );
    return if center {
        centerSpan(span, options, _text.clone())
    } else {
        span
    };
}

/**
 * Make a span from a font glyph with the given offset and in the given font.
 * This is used in makeStackedDelim to make the stacking pieces for the delimiter.
 */
fn makeGlyphSpan(symbol: &str, font: &str, mode: Mode) -> crate::build::common::VListChild {
    let sizeClass;
    // Apply the correct CSS class to choose the right font.
    if font == "Size1-Regular" {
        sizeClass = "delim-size1";
    } else if font == "Size4-Regular" {
        sizeClass = "delim-size4";
    } else {
        panic!("");
    }

    let corner = crate::build::common::make_span(
        vec!["delimsizinginner".to_string(), sizeClass.to_string()],
        vec![Box::new(crate::build::common::make_span(
            vec![],
            vec![Box::new(crate::build::common::make_symbol(
                symbol.to_string(),
                font.to_string(),
                mode,
                None,
                vec![],
            )) as Box<dyn HtmlDomNode>],
            None,
            Default::default(),
        )) as Box<dyn HtmlDomNode>],
        None,
        Default::default(),
    );

    // Since this will be passed into `makeVList` in the end, wrap the element
    // in the appropriate tag that VList uses.
    return VListChild::Elem {
        elem: Box::new(corner) as Box<dyn HtmlDomNode>,
        margin_left: None,
        margin_right: None,
        wrapper_classes: None,
        wrapper_style: None,
        shift: None,
    };
}

fn make_inner(ch: &str, height: f64, options: &Options) -> VListChild {
    // Create a span with inline SVG for the inner part of a tall stacked delimiter.
    let width = if let Some(m) =
        get_char_metrics("Size4-Regular", ch.chars().nth(0).unwrap().to_string())
    {
        m.width
    } else {
        get_char_metrics("Size1-Regular", ch.chars().nth(0).unwrap().to_string())
            .unwrap()
            .width
    };
    let path = PathNode::new(
        "inner".to_string(),
        Some(crate::svgGeometry::inner_path(
            ch,
            (1000.0 * height).round(),
        )),
    );
    let svgNode = SvgNode::new(
        vec![Box::new(path) as Box<dyn VirtualNode>],
        HashMap::from([
            ("width".to_string(), make_em(width)),
            ("height".to_string(), make_em(height)),
            // Override CSS rule `.katex svg { width: 100% }`
            ("style".to_string(), format!("width:{}", make_em(width))),
            (
                "viewBox".to_string(),
                format!("0 0 {} {}", 1000.0 * width, (1000.0 * height).round()),
            ),
            ("preserveAspectRatio".to_string(), "xMinYMin".to_string()),
        ]),
    );
    let mut span = Span::new(
        vec![],
        vec![Box::new(svgNode) as Box<dyn HtmlDomNode>],
        Some(options.clone()),
        Default::default(),
    );
    span.set_height(height);
    span.get_mut_style().height = Some(make_em(height));
    span.get_mut_style().width = Some(make_em(width));
    return VListChild::Elem {
        elem: Box::new(span) as Box<dyn HtmlDomNode>,
        margin_left: None,
        margin_right: None,
        wrapper_classes: None,
        wrapper_style: None,
        shift: None,
    };
}

//
// Helpers for makeStackedDelim
const lapInEms: f64 = 0.008;
const verts: [&str; 4] = ["|", "\\lvert", "\\rvert", "\\vert"];
const doubleVerts: [&str; 4] = ["\\|", "\\lVert", "\\rVert", "\\Vert"];

//
/**
 * Make a stacked delimiter out of a given delimiter, with the total height at
 * least `heightTotal`. This routine is mentioned on page 442 of the TeXbook.
 */
fn makeStackedDelim(
    delim: &str,
    heightTotal: f64,
    center: bool,
    options: &Options,
    mode: Mode,
    classes: Vec<String>,
) -> Span {
    // There are four parts, the top, an optional middle, a repeated part, and a
    // bottom.
    let mut top = "";
    let mut middle = "";
    let mut repeat = "";
    let mut bottom = "";
    let mut svgLabel = "";
    let mut viewBoxWidth = 0;
    // Also keep track of what font the delimiters are in
    let mut font = "Size1-Regular";

    // We set the parts and font based on the symbol. Note that we use
    // '\u23d0' instead of '|' and '\u2016' instead of '\\|' for the
    // repeats of the arrows
    if (delim == "\\uparrow") {
        bottom = "\u{23d0}";
        repeat = bottom;
    } else if (delim == "\\Uparrow") {
        bottom = "\u{2016}";
        repeat = bottom;
    } else if (delim == "\\downarrow") {
        repeat = "\u{23d0}";
        top = repeat;
    } else if (delim == "\\Downarrow") {
        repeat = "\u{2016}";
        top = repeat;
    } else if (delim == "\\updownarrow") {
        top = "\\uparrow";
        repeat = "\u{23d0}";
        bottom = "\\downarrow";
    } else if (delim == "\\Updownarrow") {
        top = "\\Uparrow";
        repeat = "\u{2016}";
        bottom = "\\Downarrow";
    } else if verts.contains(&delim) {
        repeat = "\u{2223}";
        svgLabel = "vert";
        viewBoxWidth = 333;
    } else if doubleVerts.contains(&delim) {
        repeat = "\u{2225}";
        svgLabel = "doublevert";
        viewBoxWidth = 556;
    } else if (delim == "[" || delim == "\\lbrack") {
        top = "\u{23a1}";
        repeat = "\u{23a2}";
        bottom = "\u{23a3}";
        font = "Size4-Regular";
        svgLabel = "lbrack";
        viewBoxWidth = 667;
    } else if (delim == "]" || delim == "\\rbrack") {
        top = "\u{23a4}";
        repeat = "\u{23a5}";
        bottom = "\u{23a6}";
        font = "Size4-Regular";
        svgLabel = "rbrack";
        viewBoxWidth = 667;
    } else if (delim == "\\lfloor" || delim == "\u{230a}") {
        top = "\u{23a2}";
        repeat = top;
        bottom = "\u{23a3}";
        font = "Size4-Regular";
        svgLabel = "lfloor";
        viewBoxWidth = 667;
    } else if (delim == "\\lceil" || delim == "\u{2308}") {
        top = "\u{23a1}";
        bottom = "\u{23a2}";
        repeat = bottom;
        font = "Size4-Regular";
        svgLabel = "lceil";
        viewBoxWidth = 667;
    } else if (delim == "\\rfloor" || delim == "\u{230b}") {
        top = "\u{23a5}";
        repeat = top;
        bottom = "\u{23a6}";
        font = "Size4-Regular";
        svgLabel = "rfloor";
        viewBoxWidth = 667;
    } else if (delim == "\\rceil" || delim == "\u{2309}") {
        top = "\u{23a4}";
        bottom = "\u{23a5}";
        repeat = bottom;
        font = "Size4-Regular";
        svgLabel = "rceil";
        viewBoxWidth = 667;
    } else if (delim == "(" || delim == "\\lparen") {
        top = "\u{239b}";
        repeat = "\u{239c}";
        bottom = "\u{239d}";
        font = "Size4-Regular";
        svgLabel = "lparen";
        viewBoxWidth = 875;
    } else if (delim == ")" || delim == "\\rparen") {
        top = "\u{239e}";
        repeat = "\u{239f}";
        bottom = "\u{23a0}";
        font = "Size4-Regular";
        svgLabel = "rparen";
        viewBoxWidth = 875;
    } else if (delim == "\\{" || delim == "\\lbrace") {
        top = "\u{23a7}";
        middle = "\u{23a8}";
        bottom = "\u{23a9}";
        repeat = "\u{23aa}";
        font = "Size4-Regular";
    } else if (delim == "\\}" || delim == "\\rbrace") {
        top = "\u{23ab}";
        middle = "\u{23ac}";
        bottom = "\u{23ad}";
        repeat = "\u{23aa}";
        font = "Size4-Regular";
    } else if (delim == "\\lgroup" || delim == "\u{27ee}") {
        top = "\u{23a7}";
        bottom = "\u{23a9}";
        repeat = "\u{23aa}";
        font = "Size4-Regular";
    } else if (delim == "\\rgroup" || delim == "\u{27ef}") {
        top = "\u{23ab}";
        bottom = "\u{23ad}";
        repeat = "\u{23aa}";
        font = "Size4-Regular";
    } else if (delim == "\\lmoustache" || delim == "\u{23b0}") {
        top = "\u{23a7}";
        bottom = "\u{23ad}";
        repeat = "\u{23aa}";
        font = "Size4-Regular";
    } else if (delim == "\\rmoustache" || delim == "\u{23b1}") {
        top = "\u{23ab}";
        bottom = "\u{23a9}";
        repeat = "\u{23aa}";
        font = "Size4-Regular";
    }

    // Get the metrics of the four sections
    let topMetrics = get_metrics(top, font, mode);
    let topHeightTotal = topMetrics.height + topMetrics.depth;
    let repeatMetrics = get_metrics(repeat, font, mode);
    let repeatHeightTotal = repeatMetrics.height + repeatMetrics.depth;
    let bottomMetrics = get_metrics(bottom, font, mode);
    let bottomHeightTotal = bottomMetrics.height + bottomMetrics.depth;
    let mut middleHeightTotal = 0.0;
    let mut middleFactor = 1.0;
    if (middle != "") {
        let middleMetrics = get_metrics(middle, font, mode);
        middleHeightTotal = middleMetrics.height + middleMetrics.depth;
        middleFactor = 2.0; // repeat symmetrically above and below middle
    }

    // Calcuate the minimal height that the delimiter can have.
    // It is at least the size of the top, bottom, and optional middle combined.
    let minHeight = topHeightTotal + bottomHeightTotal + middleHeightTotal;

    // Compute the f64 of copies of the repeat symbol we will need
    let repeatCount = f64::max(
        0.0,
        ((heightTotal - minHeight) / (middleFactor * repeatHeightTotal).ceil()),
    );

    // Compute the total height of the delimiter including all the symbols
    let realHeightTotal = minHeight + repeatCount * middleFactor * repeatHeightTotal;

    // The center of the delimiter is placed at the center of the axis. Note
    // that in this context, "center" means that the delimiter should be
    // centered around the axis in the current style, while normally it is
    // centered around the axis in textstyle.
    let mut axisHeight = options.get_font_metrics().axisHeight;
    if center {
        axisHeight *= options.sizeMultiplier;
    }
    // Calculate the depth
    let depth = realHeightTotal / 2.0 - axisHeight;

    // Now, we start building the pieces that will go into the vlist
    // Keep a list of the pieces of the stacked delimiter
    let mut stack = vec![];

    if svgLabel.len() > 0 {
        // Instead of stacking glyphs, create a single SVG.
        // This evades browser problems with imprecise positioning of spans.
        let mid_height = realHeightTotal - topHeightTotal - bottomHeightTotal;
        let view_box_height = (realHeightTotal * 1000.0).round();
        let path_str = crate::svgGeometry::tall_delim(svgLabel, (mid_height * 1000.0).round());
        let path = PathNode::new(svgLabel.to_string(), Some(path_str));
        let width = make_em(viewBoxWidth as f64 / 1000.0);
        let height = make_em(view_box_height / 1000.0);
        let svg = SvgNode::new(
            vec![Box::new(path) as Box<dyn VirtualNode>],
            std::collections::HashMap::from([
                ("width".to_string(), width.clone()),
                ("height".to_string(), height.clone()),
                (
                    "viewBox".to_string(),
                    format!("0  0 {viewBoxWidth} {view_box_height}"),
                ),
            ]),
        );

        let mut wrapper = Span::new(
            vec![],
            vec![Box::new(svg) as Box<dyn HtmlDomNode>],
            Some(options.clone()),
            Default::default(),
        );
        wrapper.set_height(view_box_height / 1000.0);
        wrapper.get_mut_style().width = Some(width.to_string());
        wrapper.get_mut_style().height = Some(height.to_string());
        stack.push(crate::build::common::VListChild::Elem {
            elem: Box::new(wrapper) as Box<dyn HtmlDomNode>,
            margin_left: None,
            margin_right: None,
            wrapper_classes: None,
            wrapper_style: None,
            shift: None,
        });
    } else {
        const lap: VListChild = Kern {
            size: -1.0 * lapInEms,
        };
        // Stack glyphs
        // Start by adding the bottom symbol
        stack.push(makeGlyphSpan(bottom, font, mode));
        stack.push(lap); // overlap

        if (middle == "") {
            // The middle section will be an SVG. Make it an extra 0.016em tall.
            // We'll overlap by 0.008em at top and bottom.
            let innerHeight = realHeightTotal - topHeightTotal - bottomHeightTotal + 2.0 * lapInEms;
            stack.push(make_inner(repeat, innerHeight, options));
        } else {
            // When there is a middle bit, we need the middle part and two repeated
            // sections
            let innerHeight =
                (realHeightTotal - topHeightTotal - bottomHeightTotal - middleHeightTotal) / 2.0
                    + 2.0 * lapInEms;
            stack.push(make_inner(repeat, innerHeight, options));
            // Now insert the middle of the brace.
            stack.push(lap);
            stack.push(makeGlyphSpan(middle, font, mode));
            stack.push(lap);
            stack.push(make_inner(repeat, innerHeight, options));
        }

        // Add the top symbol
        stack.push(lap);
        stack.push(makeGlyphSpan(top, font, mode));
    }

    // Finally, build the vlist
    let _text = crate::Style::TEXT.read().unwrap();
    let new_options = options.having_base_style(&_text);
    let inner = crate::build::common::make_vlist(
        crate::build::common::VListParam {
            position_type: PositionType::Bottom,
            position_data: Some(depth),
            children: stack,
        }
    );

    return style_wrap(
        Box::new(crate::build::common::make_span(
            vec!["delimsizing".to_string(), "mult".to_string()],
            vec![Box::new(inner) as Box<dyn HtmlDomNode>],
            Some(&new_options),
            Default::default(),
        )) as Box<dyn HtmlDomNode>,
        &_text,
        &options,
        classes,
    );
}

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

    let svg = SvgNode::new(
        vec![Box::new(pathNode) as Box<dyn VirtualNode>],
        HashMap::from([
            // Note: 1000:1 ratio of viewBox to document em width.
            ("width".to_string(), "400em".to_string()),
            ("height".to_string(), make_em(height)),
            (
                "viewBox".to_string(),
                format!("0 0 400000 {}", viewBoxHeight),
            ),
            (
                "preserveAspectRatio".to_string(),
                "xMinYMin slice".to_string(),
            ),
        ]),
    );

    return Span::new(
        vec!["hide-tail".to_string()],
        vec![Box::new(svg) as Box<dyn HtmlDomNode>],
        Some(options.clone()),
        Default::default(),
    );
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

// There are three kinds of delimiters, delimiters that stack when they become
// too large
const STACK_LARGE_DELIMITERS: [&'static str; 21] = [
    "(", "\\lparen", ")", "\\rparen", "[", "\\lbrack", "]", "\\rbrack", "\\{", "\\lbrace", "\\}",
    "\\rbrace", "\\lfloor", "\\rfloor", "\u{230a}", "\u{230b}", "\\lceil", "\\rceil", "\u{2308}",
    "\u{2309}", "\\surd",
];

// delimiters that always stack
const STACK_ALWAYS_DELIMITERS: [&'static str; 22] = [
    "\\uparrow",
    "\\downarrow",
    "\\updownarrow",
    "\\Uparrow",
    "\\Downarrow",
    "\\Updownarrow",
    "|",
    "\\|",
    "\\vert",
    "\\Vert",
    "\\lvert",
    "\\rvert",
    "\\lVert",
    "\\rVert",
    "\\lgroup",
    "\\rgroup",
    "\u{27ee}",
    "\u{27ef}",
    "\\lmoustache",
    "\\rmoustache",
    "\u{23b0}",
    "\u{23b1}",
];

// and delimiters that never stack
const STACK_NEVER_DELIMITERS: [&str; 8] = [
    "<",
    ">",
    "\\langle",
    "\\rangle",
    "/",
    "\\backslash",
    "\\lt",
    "\\gt",
];

// Metrics of the different sizes. Found by looking at TeX's output of
// $\bigl| // \Bigl| \biggl| \Biggl| \showlists$
// Used to create stacked delimiters of appropriate sizes in makeSizedDelim.
const SIZE_TO_MAX_HEIGHT: [f64; 5] = [0.0, 1.2, 1.8, 2.4, 3.0];

/**
 * Used to create a delimiter of a specific size, where `size` is 1, 2, 3, or 4.
 */
pub fn make_sized_delim(
    delim: &String,
    size: f64,
    options: &Options,
    mode: Mode,
    classes: Vec<String>,
) -> Span {
    // < and > turn into \langle and \rangle in delimiters
    let angle = if delim == "<" || delim == "\\lt" || delim == "\u{27e8}" {
        "\\langle"
    } else if delim == ">" || delim == "\\gt" || delim == "\u{27e9}" {
        "\\rangle"
    } else {
        delim
    };

    // Sized delimiters are never centered.
    if STACK_LARGE_DELIMITERS.contains(&angle) || STACK_NEVER_DELIMITERS.contains(&angle) {
        return make_large_delim(delim, size, false, options, mode, classes);
    } else if STACK_ALWAYS_DELIMITERS.contains(&angle) {
        return makeStackedDelim(
            &delim,
            SIZE_TO_MAX_HEIGHT[size as usize],
            false,
            &options,
            mode,
            classes,
        );
    } else {
        panic!("Illegal delimiter: '{delim}'");
    }
}

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
    options: &Options,
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

/**
 * Make a delimiter for use with `\left` and `\right`, given a height and depth
 * of an expression that the delimiters surround.
 */
pub fn make_left_right_delim(
    delim: &String,
    height: f64,
    depth: f64,
    options: &Options,
    mode: Mode,
    classes: Vec<String>,
) -> Span {
    // We always center \left/\right delimiters, so the axis is always shifted
    let axis_height = options.get_font_metrics().axisHeight * options.sizeMultiplier;

    // Taken from TeX source, tex.web, function make_left_right
    let delimiter_factor = 901.0;
    let delimiter_extend = 5.0 / options.get_font_metrics().ptPerEm;

    let max_dist_from_axis = f64::max(height - axis_height, depth + axis_height);

    let total_height = f64::max(
        // In real TeX, calculations are done using integral values which are
        // 65536 per pt, or 655360 per em. So, the division here truncates in
        // TeX but doesn't here, producing different results. If we wanted to
        // exactly match TeX's calculation, we could do
        //   Math.floor(655360 * max_dist_from_axis / 500) *
        //    delimiter_factor / 655360
        // (To see the difference, compare
        //    x^{x^{\left(\rule{0.1em}{0.68em}\right)}}
        // in TeX and KaTeX)
        max_dist_from_axis / 500.0 * delimiter_factor,
        2.0 * max_dist_from_axis - delimiter_extend,
    );

    // Finally, we defer to `make_custom_sized_delim` with our calculated total
    // height
    return make_custom_sized_delim(delim, total_height, true, options, mode, classes);
}
