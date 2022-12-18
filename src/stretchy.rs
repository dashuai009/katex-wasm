use crate::build::common;
use crate::build::common::make_span;
use crate::dom_tree::path_node::PathNode;
use crate::dom_tree::span::Span;
use crate::dom_tree::svg_node::SvgNode;
use crate::{make_em, parse_node, AnyParseNode, HtmlDomNode, VirtualNode};
use std::collections::HashMap;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;

use crate::mathML_tree::{math_node::MathNode, public::MathNodeType, text_node::TextNode};
use crate::parse_node::types::underline;
use crate::Options::Options;

lazy_static! {
    static ref stretchy_codepoint: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("\\widechar", "^"),
            ("\\widecheck", "ˇ"),
            ("\\widetilde", "~"),
            ("\\utilde", "~"),
            ("\\overleftarrow", "\u{2190}"),
            ("\\underleftarrow", "\u{2190}"),
            ("\\xleftarrow", "\u{2190}"),
            ("\\overrightarrow", "\u{2192}"),
            ("\\underrightarrow", "\u{2192}"),
            ("\\xrightarrow", "\u{2192}"),
            ("\\underbrace", "\u{23df}"),
            ("\\overbrace", "\u{23de}"),
            ("\\overgroup", "\u{23e0}"),
            ("\\undergroup", "\u{23e1}"),
            ("\\overleftrightarrow", "\u{2194}"),
            ("\\underleftrightarrow", "\u{2194}"),
            ("\\xleftrightarrow", "\u{2194}"),
            ("\\Overrightarrow", "\u{21d2}"),
            ("\\xRightarrow", "\u{21d2}"),
            ("\\overleftharpoon", "\u{21bc}"),
            ("\\xleftharpoonup", "\u{21bc}"),
            ("\\overrightharpoon", "\u{21c0}"),
            ("\\xrightharpoonup", "\u{21c0}"),
            ("\\xLeftarrow", "\u{21d0}"),
            ("\\xLeftrightarrow", "\u{21d4}"),
            ("\\xhookleftarrow", "\u{21a9}"),
            ("\\xhookrightarrow", "\u{21aa}"),
            ("\\xmapsto", "\u{21a6}"),
            ("\\xrightharpoondown", "\u{21c1}"),
            ("\\xleftharpoondown", "\u{21bd}"),
            ("\\xrightleftharpoons", "\u{21cc}"),
            ("\\xleftrightharpoons", "\u{21cb}"),
            ("\\xtwoheadleftarrow", "\u{219e}"),
            ("\\xtwoheadrightarrow", "\u{21a0}"),
            ("\\xlongequal", "="),
            ("\\xtofrom", "\u{21c4}"),
            ("\\xrightleftarrows", "\u{21c4}"),
            ("\\xrightequilibrium", "\u{21cc}"),
            ("\\xleftequilibrium", "\u{21cb}"),
            ("\\\\cdrightarrow", "\u{2192}"),
            ("\\\\cdleftarrow", "\u{2190}"),
            ("\\\\cdlongequal", "="),
        ])
    };
}

// Many of the KaTeX SVG images have been adapted from glyphs in KaTeX fonts.
// Copyright (c) 2009-2010, Design Science, Inc. (<www.mathjax.org>)
// Copyright (c) 2014-2017 Khan Academy (<www.khanacademy.org>)
// Licensed under the SIL Open Font License, Version 1.1.
// See \nhttp://scripts.sil.org/OFL

// Very Long SVGs
//    Many of the KaTeX stretchy wide elements use a long SVG image and an
//    overflow: hidden tactic to achieve a stretchy image while avoiding
//    distortion of arrowheads or brace corners.

//    The SVG typically contains a very long (400 em) arrow.

//    The SVG is in a container span that has overflow: hidden, so the span
//    acts like a window that exposes only part of the  SVG.

//    The SVG always has a longer, thinner aspect ratio than the container span.
//    After the SVG fills 100% of the height of the container span,
//    there is a long arrow shaft left over. That left-over shaft is not shown.
//    Instead, it is sliced off because the span's CSS has overflow: hidden.

//    Thus, the reader sees an arrow that matches the subject matter width
//    without distortion.

//    Some functions, such as \cancel, need to vary their aspect ratio. These
//    functions do not get the overflow SVG treatment.

// Second Brush Stroke
//    Low resolution monitors struggle to display images in fine detail.
//    So browsers apply anti-aliasing. A long straight arrow shaft therefore
//    will sometimes appear as if it has a blurred edge.

//    To mitigate this, these SVG files contain a second "brush-stroke" on the
//    arrow shafts. That is, a second long thin rectangular SVG path has been
//    written directly on top of each arrow shaft. This reinforcement causes
//    some of the screen pixels to display as black instead of the anti-aliased
//    gray pixel that a  single path would generate. So we get arrow shafts
//    whose edges appear to be sharper.

// In the katexImagesData object just below, the dimensions all
// correspond to path geometry inside the relevant SVG.
// For example, \overrightarrow uses the same arrowhead as glyph U+2192
// from the KaTeX Main font. The scaling factor is 1000.
// That is, inside the font, that arrowhead is 522 units tall, which
// corresponds to 0.522 em inside the document.
lazy_static! {
    static ref KATEX_IMAGES_DATA :RwLock<HashMap<&'static str,(Vec<&'static str>,f64,i32,Option<&'static str>)>> = RwLock::new({
        let res = HashMap::< &str,(Vec<& str>,f64,i32,Option<&str>)>::from([
                              //   path(s), minWidth, height, align
            ("overrightarrow", (vec!["rightarrow"], 0.888, 522, Some("xMaxYMin"))),
            ("overleftarrow", (vec!["leftarrow"], 0.888, 522, Some("xMinYMin"))),
            ("underrightarrow", (vec!["rightarrow"], 0.888, 522, Some("xMaxYMin"))),

            ("underleftarrow", (vec!["leftarrow"], 0.888, 522, Some("xMinYMin"))),
            ("xrightarrow", (vec!["rightarrow"], 1.469, 522, Some("xMaxYMin"))),
            ("\\cdrightarrow", (vec!["rightarrow"], 3.0, 522, Some("xMaxYMin"))), // CD minwwidth2.5pc
            ("xleftarrow", (vec!["leftarrow"], 1.469, 522, Some("xMinYMin"))),
            ("\\cdleftarrow", (vec!["leftarrow"], 3.0, 522, Some("xMinYMin"))),
            ("Overrightarrow", (vec!["doublerightarrow"], 0.888, 560, Some("xMaxYMin"))),
            ("xRightarrow", (vec!["doublerightarrow"], 1.526, 560, Some("xMaxYMin"))),
            ("xLeftarrow", (vec!["doubleleftarrow"], 1.526, 560, Some("xMinYMin"))),
            ("overleftharpoon", (vec!["leftharpoon"], 0.888, 522, Some("xMinYMin"))),
            ("xleftharpoonup", (vec!["leftharpoon"], 0.888, 522, Some("xMinYMin"))),
            ("xleftharpoondown", (vec!["leftharpoondown"], 0.888, 522, Some("xMinYMin"))),
("overrightharpoon", (vec!["rightharpoon"], 0.888, 522, Some("xMaxYMin"))),
("xrightharpoonup", (vec!["rightharpoon"], 0.888, 522, Some("xMaxYMin"))),
("xrightharpoondown", (vec!["rightharpoondown"], 0.888, 522, Some("xMaxYMin"))),
("xlongequal", (vec!["longequal"], 0.888, 334, Some("xMinYMin"))),
("\\cdlongequal", (vec!["longequal"], 3.0, 334, Some("xMinYMin"))),
("xtwoheadleftarrow", (vec!["twoheadleftarrow"], 0.888, 334, Some("xMinYMin"))),
("xtwoheadrightarrow", (vec!["twoheadrightarrow"], 0.888, 334, Some("xMaxYMin"))),

("overleftrightarrow", (vec!["leftarrow", "rightarrow"], 0.888, 522,None)),
("overbrace", (vec!["leftbrace", "midbrace", "rightbrace"], 1.6, 548,None)),
("underbrace", (vec!["leftbraceunder", "midbraceunder", "rightbraceunder"],1.6, 548,None)),
("underleftrightarrow", (vec!["leftarrow", "rightarrow"], 0.888, 522, None)),
("xleftrightarrow", (vec!["leftarrow", "rightarrow"], 1.75, 522, None)),
("xLeftrightarrow", (vec!["doubleleftarrow", "doublerightarrow"], 1.75, 560, None)),
("xrightleftharpoons", (vec!["leftharpoondownplus", "rightharpoonplus"], 1.75, 716, None)),
("xleftrightharpoons", (vec!["leftharpoonplus","rightharpoondownplus"],1.75, 716, None)),
("xhookleftarrow", (vec!["leftarrow", "righthook"], 1.08, 522, None)),
("xhookrightarrow", (vec!["lefthook", "rightarrow"], 1.08, 522, None)),
("overlinesegment", (vec!["leftlinesegment", "rightlinesegment"], 0.888, 522, None)),
("underlinesegment", (vec!["leftlinesegment", "rightlinesegment"], 0.888, 522, None)),
("overgroup", (vec!["leftgroup", "rightgroup"], 0.888, 342, None)),
("undergroup", (vec!["leftgroupunder", "rightgroupunder"], 0.888, 342, None)),
("xmapsto", (vec!["leftmapsto", "rightarrow"], 1.5, 522, None)),
("xtofrom", (vec!["leftToFrom", "rightToFrom"], 1.75, 528, None)),
// The next three arrows are from the mhchem package.
// In mhchem.sty, min-length is 2.0em. But these arrows might appear in the
// document as \xrightarrow or \xrightleftharpoons. Those have
// min-length = 1.75em, so we set min-length on these next three to match.
("xrightleftarrows", (vec!["baraboveleftarrow", "rightarrowabovebar"], 1.75, 901, None)),
("xrightequilibrium", (vec!["baraboveshortleftharpoon","rightharpoonaboveshortbar"], 1.75, 716, None)),
("xleftequilibrium", (vec!["shortbaraboveleftharpoon","shortrightharpoonabovebar"], 1.75, 716, None))
       ]);
        res
    });
}

pub(crate) fn math_ml_node(label: &String) -> MathNode {
    let mut node = MathNode::new(
        MathNodeType::Mo,
        vec![Box::new(TextNode::new(
            stretchy_codepoint.get(label.as_str()).unwrap().to_string(),
        ))],
        vec![],
    );
    node.set_attribute("stretchy".to_string(), "true".to_string());
    return node;
}
fn group_length(arg: &Box<dyn AnyParseNode>) -> usize {
    if let Some(a) = arg.as_any().downcast_ref::<parse_node::types::ordgroup>() {
        return a.body.len();
    } else {
        return 1;
    }
}

fn build_svg_span(group: Box<dyn AnyParseNode>, options: Options) -> (Span, f64, f64) {
    let mut view_box_width = 400000; // default
    let label = if let Some(g) = group.as_any().downcast_ref::<parse_node::types::accent>() {
        &g.label[1..]
    } else if let Some(g) = group
        .as_any()
        .downcast_ref::<parse_node::types::accentUnder>()
    {
        &g.label[1..]
    } else if let Some(g) = group.as_any().downcast_ref::<parse_node::types::xArrow>() {
        &g.label[1..]
    } else if let Some(g) = group
        .as_any()
        .downcast_ref::<parse_node::types::horizBrace>()
    {
        &g.label[1..]
    } else {
        panic!("unsupported type {:#?}", group);
    };
    if ["widehat", "widecheck", "widetilde", "utilde"].contains(&label) {
        // Each type in the `if` statement corresponds to one of the ParseNode
        // types below. This narrowing is required to access `grp.base`.
        let numChars = if let Some(grp) = group.as_any().downcast_ref::<parse_node::types::accent>()
        {
            group_length(&(grp.base.as_ref().unwrap()))
        } else if let Some(grp) = group
            .as_any()
            .downcast_ref::<parse_node::types::accentUnder>()
        {
            group_length(&grp.base)
        } else {
            panic!("unsupported type = {} {:#?}", group.get_type(), group);
        };
        // There are four SVG images available for each function.
        // Choose a taller image when there are more characters.
        let viewBoxHeight;
        let pathName;
        let height;

        if numChars > 5 {
            if (label == "widehat" || label == "widecheck") {
                viewBoxHeight = 420;
                view_box_width = 2364;
                height = 0.42;
                pathName = format!("{label}4");
            } else {
                viewBoxHeight = 312;
                view_box_width = 2340;
                height = 0.34;
                pathName = "tilde4".to_string();
            }
        } else {
            let imgIndex = [1, 1, 2, 2, 3, 3][numChars];
            if label == "widehat" || label == "widecheck" {
                view_box_width = [0, 1062, 2364, 2364, 2364][imgIndex];
                viewBoxHeight = [0, 239, 300, 360, 420][imgIndex];
                height = [0.0, 0.24, 0.3, 0.3, 0.36, 0.42][imgIndex];
                pathName = format!("{label}{}", imgIndex);
            } else {
                view_box_width = [0, 600, 1033, 2339, 2340][imgIndex];
                viewBoxHeight = [0, 260, 286, 306, 312][imgIndex];
                height = [0.0, 0.26, 0.286, 0.3, 0.306, 0.34][imgIndex];
                pathName = format!("tilde{}", imgIndex);
            }
        }
        let path = PathNode::new(pathName, None);
        let svg_node_attr = HashMap::from([
            ("width".to_string(), "100%".to_string()),
            ("height".to_string(), make_em(height)),
            (
                "viewBox".to_string(),
                format!("0 0 {view_box_width} {viewBoxHeight}"),
            ),
            ("preserveAspectRatio".to_string(), "none".to_string()),
        ]);
        let mut svg_node =
            SvgNode::new(vec![Box::new(path) as Box<dyn VirtualNode>], svg_node_attr);

        return (
            Span::new(
                vec![],
                vec![Box::new(svg_node) as Box<dyn HtmlDomNode>],
                Some(options.clone()),
                Default::default(),
            ),
            0.0,
            height,
        );
    } else {
        let mut spans = vec![];
        let katex_image_data = KATEX_IMAGES_DATA.read().unwrap();
        let data = katex_image_data.get(label).unwrap();
        let (paths, minWidth, viewBoxHeight, _) = data;
        let height = *viewBoxHeight as f64 / 1000.0;

        let numSvgChildren = paths.len();
        let widthClasses: Vec<String>;
        let aligns: Vec<String>;
        if numSvgChildren == 1 {
            //  All these cases must be of the 4-tuple type. 4th is some string.
            let align1: String = data.3.unwrap().to_string();
            widthClasses = vec!["hide-tail".to_string()];
            aligns = vec![align1];
        } else if numSvgChildren == 2 {
            widthClasses = vec!["halfarrow-left".to_string(), "halfarrow-right".to_string()];
            aligns = vec!["xMinYMin".to_string(), "xMaxYMin".to_string()];
        } else if numSvgChildren == 3 {
            widthClasses = vec![
                "brace-left".to_string(),
                "brace-center".to_string(),
                "brace-right".to_string(),
            ];
            aligns = vec![
                "xMinYMin".to_string(),
                "xMidYMin".to_string(),
                "xMaxYMin".to_string(),
            ];
        } else {
            panic!(
                "Correct katexImagesData or update code here to support {numSvgChildren} children."
            );
        }

        for i in 0..numSvgChildren {
            let path = PathNode::new(paths[i].to_string(), None);
            let svg_node_attr = HashMap::from([
                ("width".to_string(), "400em".to_string()),
                ("height".to_string(), make_em(height as f64)),
                (
                    "viewBox".to_string(),
                    format!("0 0 {view_box_width} {viewBoxHeight}"),
                ),
                (
                    "preserveAspectRatio".to_string(),
                    format!("{} slice", aligns[i]).to_string(),
                ),
            ]);
            let mut svg_node =
                SvgNode::new(vec![Box::new(path) as Box<dyn VirtualNode>], svg_node_attr);

            let mut span = Span::new(
                vec![widthClasses[i].clone()],
                vec![Box::new(svg_node) as Box<dyn HtmlDomNode>],
                Some(options.clone()),
                Default::default(),
            );
            if numSvgChildren == 1 {
                return (span, *minWidth, height as f64);
            } else {
                span.get_mut_style().height = Some(make_em(height as f64));
                spans.push(Box::new(span) as Box<dyn HtmlDomNode>);
            }
        }

        return (
            make_span(
                vec!["stretchy".to_string()],
                spans,
                Some(&options),
                Default::default(),
            ),
            *minWidth,
            height as f64,
        );
    }
} // buildSvgSpan_()
  // ParseNode<"accent"> | ParseNode<"accentUnder"> | ParseNode<"xArrow">| ParseNode<"horizBrace">,
pub fn svg_span(group: Box<dyn AnyParseNode>, options: Options) -> Span {
    // Create a span with inline SVG for the element.

    let (mut span, min_width, height) = build_svg_span(group, options);

    // Note that we are returning span.depth = 0.
    // Any adjustments relative to the baseline must be done in buildHTML.
    span.set_height(height);
    span.get_mut_style().height = Some(make_em(height));
    if min_width > 0.0 {
        span.get_mut_style().min_width = Some(make_em(min_width));
    }

    return span;
}

pub fn enclose_span(
    inner: &Box<dyn HtmlDomNode>,
    label: String,
    topPad: f64,
    bottomPad: f64,
    options: &Options,
) -> Span {
    // Return an image span for \cancel, \bcancel, \xcancel, \fbox, or \angl
    let mut img;
    let totalHeight = inner.get_height() + inner.get_depth() + topPad + bottomPad;

    if label.contains("fbox") || label.contains("color") || label.contains("angl") {
        img = common::make_span(
            vec!["stretchy".to_string(), label.to_string()],
            vec![],
            Some(options),
            Default::default(),
        );

        if (label == "fbox") {
            if let Some(c) = options.get_color() {
                img.get_mut_style().border_color = Some(c);
            }
        }
    } else {
        // \cancel, \bcancel, or \xcancel
        // Since \cancel's SVG is inline and it omits the viewBox attribute,
        // its stroke-width will not vary with span area.

        let mut lines = vec![];
        if label == "bcancel" || label == "xcancel" {
            let tmp = crate::dom_tree::line_node::LineNode::new(HashMap::from([
                ("x1".to_string(), "0".to_string()),
                ("y1".to_string(), "0".to_string()),
                ("x2".to_string(), "100%".to_string()),
                ("y2".to_string(), "100%".to_string()),
                ("stroke-width".to_string(), "0.046em".to_string()),
            ]));
            lines.push(Box::new(tmp) as Box<dyn VirtualNode>);
        }
        lazy_static! {
            static ref X_CANCEL: std::sync::Mutex<regex::Regex> =
                std::sync::Mutex::new({ regex::Regex::new("^x?cancel$").unwrap() });
        }
        let x_cancel = X_CANCEL.lock().unwrap();

        if x_cancel.is_match(&label) {
            let attr = HashMap::from([
                ("x1".to_string(), "0".to_string()),
                ("y1".to_string(), "100%".to_string()),
                ("x2".to_string(), "100%".to_string()),
                ("y2".to_string(), "0".to_string()),
                ("stroke-width".to_string(), "0.046em".to_string()),
            ]);
            let tmp = crate::dom_tree::line_node::LineNode::new(attr);
            lines.push(Box::new(tmp) as Box<dyn VirtualNode>);
        }

        let svgNode = SvgNode::new(
            lines,
            HashMap::from([
                ("width".to_string(), "100%".to_string()),
                ("height".to_string(), make_em(totalHeight)),
            ]),
        );

        img = Span::new(
            vec![],
            vec![Box::new(svgNode) as Box<dyn HtmlDomNode>],
            Some(options.clone()),
            Default::default(),
        );
    }

    img.set_height(totalHeight);
    img.get_mut_style().height = Some(make_em(totalHeight));

    return img;
}
