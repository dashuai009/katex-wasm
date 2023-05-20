use crate::build::common::{PositionType, VListChild, VListParam};
use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionContext2, FunctionDefSpec, FunctionPropSpec,
};
use crate::define::macros::public::MacroDefinition;
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::dom_tree::symbol_node::SymbolNode;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::check_symbol_node_type;
use crate::parse_node::types::{
    cr, ordgroup, textord, ArrayTag, ColSeparationType, ParseNodeToAny,
};
use crate::token::Token;
use crate::types::{BreakToken, StyleStr};
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::Parser::Parser;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::ops::Add;
use std::sync::Mutex;
use std::{any::Any, sync::Arc};

// Data stored in the ParseNode associated with the environment.

#[derive(struct_format::parse_node_type, Clone, Debug)]
pub struct Align {
    align: String,
    pregap: Option<String>,
    postgap: Option<String>,
}

#[derive(struct_format::parse_node_type, Clone, Debug)]
pub struct Separator {
    separator: String,
}

#[derive(Clone, Debug)]
pub enum AlignSpec {
    Align(Align),
    Separator(Separator),
}

// Helper functions
fn get_hlines(parser: &mut crate::Parser::Parser) -> Vec<bool> {
    // Return an array. The array length = number of hlines.
    // Each element in the array tells if the line is dashed.
    let mut hline_info = vec![];
    parser.consume_spaces();
    let mut nxt = parser.fetch().text;
    if nxt == "\\relax".to_string() {
        // \relax is an artifact of the \cr macro below
        parser.consume();
        parser.consume_spaces();
        nxt = parser.fetch().text;
    }
    while nxt == "\\hline" || nxt == "\\hdashline".to_string() {
        parser.consume();
        hline_info.push(nxt == "\\hdashline".to_string());
        parser.consume_spaces();
        nxt = parser.fetch().text;
    }
    return hline_info;
}

fn validate_ams_environment_context(context: &mut FunctionContext2) {
    let settings = context.parser.settings;
    if !settings.get_display_mode() {
        panic!("{} can be used only in display mode.", context.func_name);
    }
}

//
// // auto_tag (an argument to parse_array) can be one of three values:
// // * undefined: Regular (not-top-level) array; no tags on each row
// // * true: Automatic equation numbering, overridable by \tag
// // * false: Tags allowed on each row, but no automatic numbering
// // This function *doesn't* work with the "split" environment name.
fn get_auto_tag(name: &str) -> bool {
    if name.contains("ed") {
        return false;
    }

    if name.contains("*") {
        return false;
    }

    return true;
}

struct ParseArrayArgs {
    hskip_before_and_after: bool,
    add_jot: bool,
    cols: Vec<AlignSpec>,
    array_stretch: Option<f64>,
    col_separation_type: Option<parse_node::types::ColSeparationType>,
    auto_tag: bool,
    single_row: bool,
    empty_single_row: bool,
    max_num_cols: Option<usize>,
    leqno: bool,
}

/**
 * Parse the body of the environment, with rows delimited by \\ and
 * columns delimited by &, and create a nested list in row-major order
 * with one group per cell.  If given an optional argument style
 * ("text".to_string(), "display".to_string(), etc.), then each cell is cast into that style.
 */
fn parse_array(
    parser: &mut crate::Parser::Parser,
    args: ParseArrayArgs,
    style: crate::types::StyleStr,
) -> parse_node::types::array {
    let ParseArrayArgs {
        hskip_before_and_after,
        add_jot,
        cols,
        mut array_stretch,
        col_separation_type,
        auto_tag,
        single_row,
        empty_single_row,
        max_num_cols,
        leqno,
    } = args;

    parser.gullet.begin_group();
    if !single_row {
        // \cr is equivalent to \\ without the optional size argument (see below)
        // TODO: provide helpful error when \cr is used outside array environment
        parser.gullet.macros.set(
            &"\\cr".to_string(),
            Some(MacroDefinition::Str("\\\\\\relax".to_string())),
            false,
        );
    }

    // Get current arraystretch if it's not set by the environment
    if array_stretch.is_none() {
        let tmp_array_stretch: f64 = if let Some(stretch) = parser
            .gullet
            .expand_macro_as_text(&"\\arraystretch".to_string())
        {
            let ttmp_array_stretch: f64 =
                stretch.parse().expect("Invalid \\arraystretch: {stretch}");
            assert!(ttmp_array_stretch >= 0.0);
            ttmp_array_stretch
        } else {
            // Default \arraystretch from lttab.dtx
            1.0
        };
        array_stretch = Some(tmp_array_stretch);
    }

    // Start group for first cell
    parser.gullet.begin_group();

    let mut body = vec![vec![]];
    let mut row_gaps = vec![];
    let mut h_lines_before_row = vec![];

    let mut tags = if auto_tag { Some(vec![]) } else { None };

    // amsmath uses \global\@eqnswtrue and \global\@eqnswfalse to represent
    // whether this row should have an equation number.  Simulate this with
    // a \@eqnsw macro set to 1 or 0.
    let mut begin_row = |auto_tag: bool, parser: &mut Parser| {
        if auto_tag {
            parser.gullet.macros.set(
                &"\\@eqnsw".to_string(),
                Some(MacroDefinition::Str("1".to_string())),
                true,
            );
        }
    };
    let mut end_row = |auto_tag: bool, parser: &mut Parser, tags: &mut Option<Vec<ArrayTag>>| {
        if let Some(t) = tags {
            if parser.gullet.macros.get(&"\\df@tag".to_string()).is_some() {
                t.push(crate::parse_node::types::ArrayTag::B(
                    parser.subparse(vec![Token::new("\\df@tag".to_string(), None)]),
                ));
                parser
                    .gullet
                    .macros
                    .set(&"\\df@tag".to_string(), None, true);
            } else if auto_tag {
                let tmp = parser.gullet.macros.get(&"\\@eqnsw".to_string()).unwrap();
                let ttt = if let MacroDefinition::Str(s) = tmp {
                    s == "1"
                } else {
                    false
                };
                t.push(crate::parse_node::types::ArrayTag::A(ttt));
            } else {
                t.push(ArrayTag::A(false));
            }
        }
    };
    begin_row(auto_tag, parser);

    // Test for \hline at the top of the array.
    h_lines_before_row.push(get_hlines(parser));

    loop {
        let row = body.last_mut().unwrap();
        // eslint-disable-line no-constant-condition
        // Parse each cell in its own group (namespace)
        let cell_body = parser.parse_expression(
            false,
            Some(if single_row {
                BreakToken::End
            } else {
                BreakToken::DoubleSlash
            }),
        );
        parser.gullet.end_group();
        parser.gullet.begin_group();

        let cell = parse_node::types::styling {
            mode: parser.mode,
            loc: None,
            style: style.clone(),
            body: vec![Box::new(parse_node::types::ordgroup {
                mode: parser.mode,
                loc: None,
                body: cell_body.clone(),
                semisimple: false,
            }) as Box<dyn AnyParseNode>],
        };
        row.push(Box::new(cell) as Box<dyn AnyParseNode>);
        match parser.fetch().text.as_str() {
            "&" => {
                if Some(row.len()) == max_num_cols {
                    if single_row || col_separation_type.is_some() {
                        // {equation} or {split}
                        panic!("Too many tab characters: &{:#?}", parser.next_token);
                    } else {
                        // {array} environment
                        parser.settings.report_nonstrict(
                            "textEnv",
                            "Too few columns specified in the {array} column argument.",
                            None,
                        );
                    }
                }
                parser.consume();
            }
            r"\end" => {
                end_row(auto_tag, parser, &mut tags);
                // Arrays terminate newlines with `\crcr` which consumes a `\cr` if
                // the last line is empty.  However, AMS environments keep the
                // empty row if it's the only one.
                // NOTE: Currently, `cell` is the last item added into `row`.
                if row.len() == 1 && cell_body.len() == 0 && (body.len() > 1 || !empty_single_row) {
                    body.pop();
                }
                if h_lines_before_row.len() < body.len() + 1 {
                    h_lines_before_row.push(vec![]);
                }
                break;
            }
            r"\\" => {
                parser.consume();
                let mut size = None;
                // \def\Let@{\let\\\math@cr}
                // \def\math@cr{...\math@cr@}
                // \def\math@cr@{\new@ifnextchar[\math@cr@@{\math@cr@@[\z@]}}
                // \def\math@cr@@[#1]{...\math@cr@@@...}
                // \def\math@cr@@@{\cr}
                if parser.gullet.future().text != " ".to_string() {
                    size = parser.parse_size_group(true);
                }
                row_gaps.push(if let Some(i) = size {
                    i.as_any()
                        .downcast_ref::<parse_node::types::size>()
                        .map(|j| j.value.clone())
                } else {
                    None
                });
                end_row(auto_tag, parser, &mut tags);

                // check for \hline(s) following the row separator
                h_lines_before_row.push(get_hlines(parser));

                body.push(vec![]);
                begin_row(auto_tag, parser);
            }
            _ => {
                panic!(
                    "Expected & or \\\\ or \\cr or \\end {:#?}",
                    parser.next_token
                );
            }
        }
    }

    // End cell group
    parser.gullet.end_group();
    // End array group defining \cr
    parser.gullet.end_group();

    return parse_node::types::array {
        mode: parser.mode,
        add_jot,
        array_stretch: array_stretch.unwrap(),
        body,
        cols,
        row_gaps,
        hskip_before_and_after,
        h_lines_before_row,
        col_separation_type,
        tags,
        leqno,
        loc: None,
        is_cd: false,
    };
}

// Decides on a style for cells in an array according to whether the given
// environment name starts with the letter 'd'.
fn dCellStyle(env_name: &String) -> StyleStr {
    if env_name.starts_with("d") {
        return StyleStr::display;
    } else {
        return StyleStr::text;
    }
}

//
// type Outrow = {
// [idx: number]: *,
// height: number,
// depth: number,
// pos: number,
// };
struct Hline {
    pub pos: f64,
    pub is_dashed: bool,
}

fn array_html_builder(mut _group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let mut group = _group
        .as_mut_any()
        .downcast_mut::<crate::parse_node::types::array>()
        .unwrap();
    // panic!("undefined")
    let nr = group.body.len();
    let h_lines_before_row = &group.h_lines_before_row;
    let mut nc = 0;
    let mut body = Vec::new();
    // let mut body = Vec::with_capacity(nr); //   Array(nr);
    let mut hlines = Vec::new();

    let rule_thickness = f64::max(
        // From LaTeX \showthe\arrayrulewidth. Equals 0.04 em.
        (options.get_font_metrics().arrayRuleWidth),
        options.minRuleThickness, // User override.
    );

    // Horizontal spacing
    let pt = 1.0 / options.get_font_metrics().ptPerEm;
    let mut arraycolsep = 5.0 * pt; // default value, i.e. \arraycolsep in article.cls
    if group.col_separation_type == Some(ColSeparationType::Small) {
        // We're in a {smallmatrix}. Default column space is \thickspace,
        // i.e. 5/18em = 0.2778em, per amsmath.dtx for {smallmatrix}.
        // But that needs adjustment because LaTeX applies \scriptstyle to the
        // entire array, including the colspace, but this function applies
        // \scriptstyle only inside each element.
        let _script = crate::Style::SCRIPT.read().unwrap();
        let local_multiplier = options.having_style(&_script).sizeMultiplier;
        arraycolsep = 0.2778 * (local_multiplier / options.sizeMultiplier);
    }

    // Vertical spacing
    let baselineskip = if group.col_separation_type == Some(ColSeparationType::CD) {
        crate::units::calculate_size(
            &crate::units::Measurement::new(3.0, "ex".to_string()),
            &options,
        )
    } else {
        12.0 * pt // see size10.clo
    };
    // Default \jot from ltmath.dtx
    // TODO(edemaine): allow overriding \jot via \setlength (#687)
    let jot = 3.0 * pt;
    let arrayskip = group.array_stretch * baselineskip;
    let arstrut_height = 0.7 * arrayskip; // \strutbox in ltfsstrc.dtx and
    let arstrut_depth = 0.3 * arrayskip; // \@arstrutbox in lttab.dtx

    // Set a position for \hline(s) at the top of the array, if any.
    let mut set_hline_pos = |hlines_in_gap: &Vec<bool>, total_height: &mut f64| {
        for (i, h) in hlines_in_gap.iter().enumerate() {
            if i > 0 { *total_height += 0.25; }
            hlines.push(Hline {
                pos: total_height.clone(),
                is_dashed: *h,
            });
        }
    };

    let mut total_height = 0.0;
    set_hline_pos(&h_lines_before_row[0], &mut total_height);

    for (r, inrow) in group.body.iter_mut().enumerate() {
        let mut height = arstrut_height; // \@array adds an \@arstrut
        let mut depth = arstrut_depth; // to each tow (via the template)

        if nc < inrow.len() {
            nc = inrow.len();
        }

        let mut outrow = Vec::new();
        for inrow_c in inrow.into_iter() {
            let elt = HTML::build_group(Some(inrow_c.clone()), options.clone(), None);
            if depth < elt.get_depth() {
                depth = elt.get_depth();
            }
            if height < elt.get_height() {
                height = elt.get_height();
            }
            outrow.push(elt);
        }

        let mut gap = 0.0;
        if r < group.row_gaps.len() {
            if let Some(m) = &group.row_gaps[r] {
                gap = crate::units::calculate_size(m, &options);
                if gap > 0.0 {
                    // \@argarraycr
                    gap += arstrut_depth;
                    if depth < gap {
                        depth = gap; // \@xargarraycr
                    }
                    gap = 0.0;
                }
            }
        }
        // In AMS multiline environments such as aligned and gathered, rows
        // correspond to lines that have additional \jot added to the
        // \baselineskip via \openup.
        if group.add_jot {
            depth += jot;
        }

        total_height += height;
        body.push((outrow, height, depth, total_height));
        total_height += depth + gap; // \@yargarraycr
        // Set a position for \hline(s), if any.
        set_hline_pos(&h_lines_before_row[r + 1], &mut total_height);
    }

    let offset = total_height / 2.0 + options.get_font_metrics().axisHeight;
    let col_descriptions = &group.cols;
    let mut cols = vec![];
    let mut col_sep;

    let mut tag_spans = vec![];
    if let Some(tags) = &group.tags {
        if tags.iter().any(|i| -> bool {
            return match i {
                ArrayTag::A(a) => *a,
                ArrayTag::B(b) => true/* b.len() > 0 */,
            };
        }) {
            // An environment with manual tags and/or automatic equation numbers.
            // Create node(s), the latter of which trigger CSS counter increment.
            for ((rw, rw_height, rw_depth, rw_pos), tag) in body.iter().zip(tags.iter()) {
                let shift = rw_pos - offset;
                let mut tag_span = match tag {
                    ArrayTag::A(a) => {
                        common::make_span(
                            if *a {
                                // automatic numbering
                                vec!["eqn-num".to_string()]
                            } else {
                                // \nonumber/\notag or starred environment
                                vec![]
                            },
                            vec![],
                            Some(&options),
                            Default::default(),
                        )
                    }
                    ArrayTag::B(b) => {
                        // manual \tag
                        common::make_span(
                            vec![],
                            HTML::build_expression(
                                b.clone(),
                                options.clone(),
                                IsRealGroup::T,
                                (None, None),
                            ),
                            Some(&options),
                            Default::default(),
                        )
                    }
                };
                tag_span.set_depth(*rw_depth);
                tag_span.set_height(*rw_height);
                tag_spans.push(VListChild::Elem {
                    elem: Box::new(tag_span) as Box<dyn HtmlDomNode>,
                    margin_left: None,
                    margin_right: None,
                    wrapper_classes: None,
                    wrapper_style: None,
                    shift: Some(shift),
                });
            }
        }
    }

    let mut c = 0;
    let mut col_descr_num = 0;

    while c < nc || col_descr_num < col_descriptions.len() {
        // Continue while either there are more columns or more column
        // descriptions, so trailing separators don't get lost.

        let mut first_separator = true;
        while col_descr_num < col_descriptions.len() {
            if let AlignSpec::Separator(col_des) = &col_descriptions[col_descr_num] {
                // If there is more than one separator in a row, add a space
                // between them.
                if !first_separator {
                    col_sep = common::make_span(
                        vec!["arraycolsep".to_string()],
                        vec![],
                        None,
                        Default::default(),
                    );
                    col_sep.get_mut_style().width = Some(crate::units::make_em(
                        options.get_font_metrics().doubleRuleSep,
                    ));
                    cols.push(col_sep);
                }

                if col_des.separator == "|" || col_des.separator == ":".to_string() {
                    let line_type = if col_des.separator == "|".to_string() {
                        "solid"
                    } else {
                        "dashed"
                    };
                    let mut separator = common::make_span(
                        vec!["vertical-separator".to_string()],
                        vec![],
                        Some(&options),
                        Default::default(),
                    );
                    separator.get_mut_style().height = Some(crate::units::make_em(total_height));
                    separator.get_mut_style().border_right_width =
                        Some(crate::units::make_em(rule_thickness));
                    separator.get_mut_style().border_right_style = Some(line_type.to_string());
                    separator.get_mut_style().margin = Some(format!(
                        "0 {}",
                        crate::units::make_em(-rule_thickness / 2.0)
                    ));
                    let shift = total_height - offset;
                    if shift != 0.0 {
                        separator.get_mut_style().vertical_align =
                            Some(crate::units::make_em(-shift));
                    }

                    cols.push(separator);
                } else {
                    panic!("Invalid separator type: {}", col_des.separator);
                }

                col_descr_num += 1;
                first_separator = false;
            } else {
                break;
            }
        }

        if c >= nc {
            continue;
        }

        if c > 0 || group.hskip_before_and_after {
            let sepwidth =
                if col_descr_num >= col_descriptions.len() {
                    None
                } else if let AlignSpec::Align(col_align) = &col_descriptions[col_descr_num] {
                    col_align.clone().pregap
                } else {
                    None
                }
                    .unwrap_or_else(|| arraycolsep.to_string())
                    .parse()
                    .unwrap();
            if sepwidth != 0.0 {
                col_sep = common::make_span(
                    vec!["arraycolsep".to_string()],
                    vec![],
                    None,
                    Default::default(),
                );
                col_sep.get_mut_style().width = Some(crate::units::make_em(sepwidth));
                cols.push(col_sep);
            }
        }

        let mut col_body = vec![];
        for (row, row_height, row_depth, row_pos) in body.iter_mut() {
            if c >= row.len() {
                continue;
            }
            let mut elem = &mut row[c];
            let shift = *row_pos - offset;
            elem.set_depth(*row_depth);
            elem.set_height(*row_height);
            col_body.push(VListChild::Elem {
                elem: elem.clone(),
                margin_left: None,
                margin_right: None,
                wrapper_classes: None,
                wrapper_style: None,
                shift: Some(shift),
            });
        }
        let col = common::make_span(
            vec![format!("col-align-{}", if col_descr_num >= col_descriptions.len() {
                "c"
            } else if let AlignSpec::Align(col_align) = &col_descriptions[col_descr_num] {
                &col_align.align
            } else {
                "c"
            })],
            vec![Box::new(common::make_vlist(VListParam {
                position_type: PositionType::IndividualShift,
                children: col_body,
                position_data: None,
            })) as Box<dyn HtmlDomNode>],
            None,
            Default::default(),
        );
        cols.push(col);

        if c < nc - 1 || group.hskip_before_and_after {
            let sepwidth = if col_descr_num >= col_descriptions.len() {
                None
            } else if let AlignSpec::Align(col_align) = &col_descriptions[col_descr_num] {
                col_align.clone().pregap
            } else {
                None
            }
                .unwrap_or(arraycolsep.to_string())
                .parse()
                .unwrap();
            if sepwidth != 0.0 {
                col_sep = common::make_span(
                    vec!["arraycolsep".to_string()],
                    vec![],
                    None,
                    Default::default(),
                );
                col_sep.get_mut_style().width = Some(crate::units::make_em(sepwidth));
                cols.push(col_sep);
            }
        }

        c += 1;
        col_descr_num += 1;
    }

    let mut res_body = common::make_span(
        vec!["mtable".to_string()],
        cols.into_iter()
            .map(|x| Box::new(x) as Box<dyn HtmlDomNode>)
            .collect::<Vec<_>>(),
        None,
        Default::default(),
    );

    // Add \hline(s), if any.
    if hlines.len() > 0 {
        let line = common::make_line_span("hline".to_string(), &options, Some(rule_thickness));
        let dashes =
            common::make_line_span("hdashline".to_string(), &options, Some(rule_thickness));
        let mut v_list_elems = vec![VListChild::Elem {
            elem: Box::new(res_body) as Box<dyn HtmlDomNode>,
            margin_left: None,
            margin_right: None,
            wrapper_classes: None,
            wrapper_style: None,
            shift: Some(0.0),
        }];
        while hlines.len() > 0 {
            let hline = hlines.pop().unwrap();
            let line_shift = hline.pos - offset;
            v_list_elems.push(VListChild::Elem {
                elem: Box::new(if hline.is_dashed {
                    dashes.clone()
                } else {
                    line.clone()
                }) as Box<dyn HtmlDomNode>,
                margin_left: None,
                margin_right: None,
                wrapper_classes: None,
                wrapper_style: None,
                shift: Some(line_shift),
            });
        }
        res_body = common::make_vlist(VListParam {
            position_type: PositionType::IndividualShift,
            children: v_list_elems,
            position_data: None,
        });
    }

    if tag_spans.len() == 0 {
        return Box::new(common::make_span(
            vec!["mord".to_string()],
            vec![Box::new(res_body) as Box<dyn HtmlDomNode>],
            Some(&options),
            Default::default(),
        )) as Box<dyn HtmlDomNode>;
    } else {
        let mut eqn_num_col = common::make_span(
            vec!["tag".to_string()],
            vec![Box::new(common::make_vlist(VListParam {
                position_type: PositionType::IndividualShift,
                children: tag_spans,
                position_data: None,
            })) as Box<dyn HtmlDomNode>],
            Some(&options),
            Default::default(),
        );
        return Box::new(crate::build::common::make_fragment(vec![
            Box::new(res_body) as Box<dyn HtmlDomNode>,
            Box::new(eqn_num_col) as Box<dyn HtmlDomNode>,
        ])) as Box<dyn HtmlDomNode>;
    }
}

//
// let alignMap = {
// c: "center ".to_string(),
// l: "left ".to_string(),
// r: "right ".to_string(),
// };
//
fn array_mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::array>()
        .unwrap();
    panic!("undefined")
    // let tbl = [];
    // let glue = MathNode::new(MathNodeType::Mtd, vec![], vec!["mtr-glue".to_string()]);
    // let tag = MathNode::new(MathNodeType::Mtd, vec![], vec!["mml-eqn-num".to_string()]);
    //     for rw in group.body{
    //         let row = [];
    //         for rw_item in rw{
    //             row.push(MathNode::new(MathNodeType::Mtd, vec![mathML::build_group(Some(rw_item), options.clone())], vec![]));
    //
    //         }
    //         if (group.tags & & group.tags[i]) {
    //             row.unshift(glue);
    //             row.push(glue);
    //             if (group.leqno) {
    //                 row.unshift(tag);
    //             } else {
    //                 row.push(tag);
    //             }
    //         }
    //         tbl.push(MathNode::new("mtr".to_string(), row));
    //     }
    // let table = MathNode::new("mtable".to_string(), tbl);
    //
    // // Set column alignment, row spacing, column spacing, and
    // // array lines by setting attributes on the table element.
    //
    // // Set the row spacing. In MathML, we specify a gap distance.
    // // We do not use rowGap[] because MathML automatically increases
    // // cell height with the height/depth of the element content.
    //
    // // LaTeX \arraystretch multiplies the row baseline-to-baseline distance.
    // // We simulate this by adding (arraystretch - 1)em to the gap. This
    // // does a reasonable job of adjusting arrays containing 1 em tall content.
    //
    // // The 0.16 and 0.09 values are found emprically. They produce an array
    // // similar to LaTeX and in which content does not interfere with \hines.
    // let gap = (group.arraystretch == 0.5)
    // ? 0.1  // {smallmatrix}, {subarray}
    // : 0.16 + group.arraystretch - 1 + (group.add_jot ? 0.09: 0);
    // table.set_attribute("rowspacing".to_string(), crate::units::make_em(gap));
    //
    // // MathML table lines go only between cells.
    // // To place a line on an edge we'll use <menclose>, if necessary.
    // let menclose = "";
    // let align = "";
    //
    // if (group.cols & & group.cols.length > 0) {
    // // Find column alignment, column spacing, and  vertical lines.
    // let cols = group.cols;
    // let columnLines = "";
    // let prevTypeWasAlign = false;
    // let iStart = 0;
    // let iEnd = cols.length;
    //
    // if (cols[0].type == "separator".to_string()) {
    // menclose += "top ";
    // iStart = 1;
    // }
    // if (cols[cols.length - 1].type == "separator".to_string()) {
    // menclose += "bottom ";
    // iEnd -= 1;
    // }
    //
    // for ( let i = iStart; i < iEnd; i + + ) {
    // if (cols[i].type == "align".to_string()) {
    // align += alignMap[cols[i].align];
    //
    // if (prevTypeWasAlign) {
    // columnLines += "none ";
    // }
    // prevTypeWasAlign = true;
    // } else if (cols[i].type == "separator".to_string()) {
    // // MathML accepts only single lines between cells.
    // // So we read only the first of consecutive separators.
    // if (prevTypeWasAlign) {
    // columnLines += cols[i].separator == "|"
    // ? "solid "
    // : "dashed ";
    // prevTypeWasAlign = false;
    // }
    // }
    // }
    //
    // table.set_attribute("columnalign".to_string(), align.trim());
    //
    // if ( /[sd] /.test(columnLines)) {
    // table.set_attribute("columnlines".to_string(), columnLines.trim());
    // }
    // }
    //
    // // Set column spacing.
    // if (group.col_separation_type == "align".to_string()) {
    // let cols = group.cols | | [];
    // let spacing = "";
    // for ( let i = 1; i < cols.length; i+ + ) {
    // spacing += i % 2 ? "0em ": "1em ";
    // }
    // table.set_attribute("columnspacing".to_string(), spacing.trim());
    // } else if (group.col_separation_type == "alignat" | |
    // group.col_separation_type == "gather".to_string()) {
    // table.set_attribute("columnspacing".to_string(), "0em".to_string());
    // } else if (group.col_separation_type == "small".to_string()) {
    // table.set_attribute("columnspacing".to_string(), "0.2778em".to_string());
    // } else if (group.col_separation_type == "CD".to_string()) {
    // table.set_attribute("columnspacing".to_string(), "0.5em".to_string());
    // } else {
    // table.set_attribute("columnspacing".to_string(), "1em".to_string());
    // }
    //
    // // Address \hline and \hdashline
    // let rowLines = "";
    // let hlines = group.hLinesBeforeRow;
    //
    // menclose += hlines[0].length > 0 ? "left ": "";
    // menclose += hlines[hlines.length - 1].length > 0 ? "right ": "";
    //
    // for ( let i = 1; i < hlines.length - 1; i + + ) {
    // rowLines += (hlines[i].length == 0)
    // ? "none "
    // // MathML accepts only a single line between rows. Read one element.
    // : hlines[i][0] ? "dashed ": "solid ";
    // }
    // if ( / [sd] /.test(rowLines)) {
    // table.set_attribute("rowlines".to_string(), rowLines.trim());
    // }
    //
    // if (menclose != "".to_string()) {
    // table = MathNode::new("menclose".to_string(), [table]);
    // table.set_attribute("notation".to_string(), menclose.trim());
    // }
    //
    // if (group.arraystretch & & group.arraystretch < 1) {
    // // A small array. Wrap in scriptstyle so row gap is not too large.
    // table = MathNode::new("mstyle".to_string(), [table]);
    // table.set_attribute("scriptlevel".to_string(), "1".to_string());
    // }
    //
    // return table;
}

pub fn aligned_handler(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut context = ctx.borrow_mut();
    if !context.func_name.contains("ed") {
        validate_ams_environment_context(&mut context);
    }

    let cols = Vec::new();
    let separation_type = if context.func_name.contains("at") {
        ColSeparationType::AlignAt
    } else {
        ColSeparationType::Align
    };
    let is_split = context.func_name == "split";
    let parse_array_args = ParseArrayArgs {
        hskip_before_and_after: false,
        add_jot: true,
        cols: cols.clone(),
        array_stretch: None,
        col_separation_type: Some(separation_type),
        auto_tag: if is_split {
            false
        } else {
            get_auto_tag(&context.func_name)
        },
        single_row: false,
        empty_single_row: true,
        max_num_cols: if is_split { Some(2) } else { None },
        leqno: context.parser.settings.get_leqno(),
    };
    let mut res = parse_array(
        context.parser,
        parse_array_args,
        crate::types::StyleStr::display,
    );

    // Determining number of columns.
    // 1. If the first argument is given, we use it as a number of columns,
    //    and makes sure that each row doesn't exceed that number.
    // 2. Otherwise, just count number of columns = maximum number
    //    of cells in each row ("aligned" mode -- isAligned will be true).
    //
    // At the same time, prepend empty group {} at beginning of every second
    // cell in each row (starting with second cell) so that operators become
    // binary.  This behavior is implemented in amsmath's \start@aligned.
    let mut num_maths: usize = 0;
    let mut num_cols = 0;
    let empty_group = parse_node::types::ordgroup {
        mode: context.parser.mode,
        loc: None,
        body: Vec::new(),
        semisimple: false,
    };

    if args.len() > 0 {
        if let Some(ord) = args[0]
            .as_any()
            .downcast_ref::<parse_node::types::ordgroup>()
        {
            let mut arg0 = String::new();
            for body_item in ord.body.iter() {
                let textord = body_item
                    .as_ref()
                    .as_any()
                    .downcast_ref::<parse_node::types::textord>()
                    .unwrap();
                arg0.push_str(&textord.text);
            }
            num_maths = arg0.parse().unwrap();
            num_cols = num_maths * 2;
        }
    }
    let is_aligned = (num_cols == 0);
    res.body.iter_mut().for_each(|row| {
        let mut i = 1;
        while i < row.len() {
            // Modify ordgroup node within styling node
            let styling = row[i]
                .as_mut_any()
                .downcast_mut::<parse_node::types::styling>()
                .unwrap();
            let ordgroup = styling.body[0]
                .as_mut_any()
                .downcast_mut::<parse_node::types::ordgroup>()
                .unwrap(); // assertNodeType(styling.body[0], "ordgroup");
            ordgroup
                .body
                .insert(0, Box::new(empty_group.clone()) as Box<dyn AnyParseNode>);
            i += 2;
        }
        if !is_aligned {
            // Case 1
            let cur_maths = row.len() / 2;
            if num_maths < cur_maths {
                panic!(
                    "Too many math in a row: {} expected {}, but got {}",
                    "row[0]", num_maths, cur_maths
                );
            }
        } else if num_cols < row.len() {
            // Case 2
            num_cols = row.len();
        }
    });

    // Adjusting alignment.
    // In aligned mode, we add one \qquad between columns;
    // otherwise we add nothing.
    for i in 0..num_cols {
        let mut align = "r";
        let mut pregap = 0;
        if i % 2 == 1 {
            align = "l";
        } else if i > 0 && is_aligned {
            // "aligned" mode.
            pregap = 1; // add one \quad
        }
        if i < res.cols.len() {
            res.cols[i] = (AlignSpec::Align(Align {
                align: align.to_string(),
                pregap: Some(pregap.to_string()),
                postgap: Some("0".to_string()),
            }));
        } else {
            res.cols.push(AlignSpec::Align(Align {
                align: align.to_string(),
                pregap: Some(pregap.to_string()),
                postgap: Some("0".to_string()),
            }));
        }
    }
    res.col_separation_type = if is_aligned {
        Some(ColSeparationType::Align)
    } else {
        Some(ColSeparationType::AlignAt)
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

// // Convenience function for align, align*, aligned, alignat, alignat*, alignedat.
// let alignedHandler = function(context, args) {
// if (context.envName.indexOf("ed".to_string()) == -1) {
// validateAmsEnvironmentContext(context);
// }
// let cols = [];
// let separationType = context.envName.indexOf("at".to_string()) > -1 ? "alignat" : "align";
// let isSplit = context.envName == "split";
// let res = parse_array(context.parser,
// {
// cols,
// add_jot: true,
// auto_tag: isSplit ? undefined : getAutoTag(context.envName),
// empty_single_row: true,
// col_separation_type: separationType,
// max_num_cols: isSplit ? 2 : undefined,
// leqno: context.parser.settings.leqno,
// },
// "display"
// );
//
// // Determining number of columns.
// // 1. If the first argument is given, we use it as a number of columns,
// //    and makes sure that each row doesn't exceed that number.
// // 2. Otherwise, just count number of columns = maximum number
// //    of cells in each row ("aligned" mode -- isAligned will be true).
// //
// // At the same time, prepend empty group {} at beginning of every second
// // cell in each row (starting with second cell) so that operators become
// // binary.  This behavior is implemented in amsmath's \start@aligned.
// let numMaths;
// let numCols = 0;
// let emptyGroup = {
// type: "ordgroup".to_string(),
// mode: context.mode,
// body: [],
// };
// if (args[0] && args[0].type == "ordgroup".to_string()) {
// let arg0 = "";
// for (let i = 0; i < args[0].body.length; i++) {
// let textord = assertNodeType(args[0].body[i], "textord".to_string());
// arg0 += textord.text;
// }
// numMaths = Number(arg0);
// numCols = numMaths * 2;
// }
// let isAligned = !numCols;
// res.body.forEach(function(row) {
// for (let i = 1; i < row.length; i += 2) {
// // Modify ordgroup node within styling node
// let styling = assertNodeType(row[i], "styling".to_string());
// let ordgroup = assertNodeType(styling.body[0], "ordgroup".to_string());
// ordgroup.body.unshift(emptyGroup);
// }
// if (!isAligned) { // Case 1
// let curMaths = row.length / 2;
// if (numMaths < curMaths) {
// panic!(
// "Too many math in a row: " +
// `expected ${numMaths}, but got ${curMaths}`,
// row[0]);
// }
// } else if (numCols < row.length) { // Case 2
// numCols = row.length;
// }
// });
//
// // Adjusting alignment.
// // In aligned mode, we add one \qquad between columns;
// // otherwise we add nothing.
// for (let i = 0; i < numCols; ++i) {
// let align = "r";
// let pregap = 0;
// if (i % 2 == 1) {
// align = "l";
// } else if (i > 0 && isAligned) { // "aligned" mode.
// pregap = 1; // add one \quad
// }
// cols[i] = {
// type: "align".to_string(),
// align: align,
// pregap: pregap,
// postgap: 0,
// };
// }
// res.col_separation_type = isAligned ? "align" : "alignat";
// return res;
// };

pub fn array_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut context = ctx.borrow_mut();
    // Since no types are specified above, the two possibilities are
    // - The argument is wrapped in {} or [], in which case Parser's
    //   parseGroup() returns an "ordgroup" wrapping some symbol node.
    // - The argument is a bare symbol node.
    let sym_node = check_symbol_node_type(&args[0]);
    let colalign = if sym_node {
        vec![args[0].clone()]
    } else {
        args[0]
            .as_ref()
            .as_any()
            .downcast_ref::<ordgroup>()
            .unwrap()
            .body
            .clone()
    };
    let cols: Vec<_> = colalign
        .into_iter()
        .map(|nde| {
            let ca = crate::parse_node::check_symbol_node_type_text(&nde);
            if "lcr".contains(&ca) {
                let res = Align {
                    align: ca.clone(),
                    pregap: None,
                    postgap: None,
                };
                return AlignSpec::Align(res);
            } else if ca == "|".to_string() {
                return AlignSpec::Separator(Separator {
                    separator: "|".to_string(),
                });
            } else if ca == ":".to_string() {
                return AlignSpec::Separator(Separator {
                    separator: ":".to_string(),
                });
            }
            panic!("Unknown column alignment: {} {:#?}", ca, nde);
        })
        .collect();
    let res = ParseArrayArgs {
        hskip_before_and_after: true, // \@preamble in lttab.dtx
        add_jot: false,
        max_num_cols: Some(cols.len()),
        cols,
        array_stretch: None,
        col_separation_type: None,
        auto_tag: false,
        single_row: false,
        empty_single_row: false,
        leqno: false,
    };
    let f_name = { context.func_name.clone() };
    return Box::new(parse_array(context.parser, res, dCellStyle(&f_name)))
        as Box<dyn AnyParseNode>;
}

// Arrays are part of LaTeX, defined in lttab.dtx so its documentation
// is part of the source2e.pdf file of LaTeX2e source documentation.
// {darray} is an {array} environment where cells are set in \displaystyle,
// as defined in nccmath.sty.
lazy_static! {
    pub static ref ARRAY: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);

        FunctionDefSpec {
            def_type: "array".to_string(),
            names: vec!["array".to_string(), "darray".to_string()],
            props,
            handler: array_handler_fn,
            html_builder: Some(array_html_builder),
            mathml_builder: Some(array_mathml_builder),
        }
    });

    // In the align environment, one uses ampersands, &, to specify number of
    // columns in each row, and to locate spacing between each column.
    // align gets automatic numbering. align* and aligned do not.
    // The alignedat environment can be used in math mode.
    // Note that we assume \nomallineskiplimit to be zero,
    // so that \strut@ is the same as \strut.
    pub static ref ARRAY_ALIGN: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);

        FunctionDefSpec{
            def_type: "array".to_string(),
            names: vec!["align".to_string(), "align*".to_string(), "aligned".to_string(), "split".to_string()],
            props,
            handler: aligned_handler,
            html_builder: Some(array_html_builder),
            mathml_builder: Some(array_mathml_builder),

        }
});

}

// // The matrix environments of amsmath builds on the array environment
// // of LaTeX, which is discussed above.
// // The mathtools package adds starred versions of the same environments.
// // These have an optional argument to choose left|center|right justification.
// defineEnvironment({
// type: "array".to_string(),
// names: [
// "matrix".to_string(),
// "pmatrix".to_string(),
// "bmatrix".to_string(),
// "Bmatrix".to_string(),
// "vmatrix".to_string(),
// "Vmatrix".to_string(),
// "matrix*".to_string(),
// "pmatrix*".to_string(),
// "bmatrix*".to_string(),
// "Bmatrix*".to_string(),
// "vmatrix*".to_string(),
// "Vmatrix*".to_string(),
// ],
// props: {
// numArgs: 0,
// },
// handler(context) {
// let delimiters = {
// "matrix": null,
// "pmatrix": ["(".to_string(), ")".to_string()],
// "bmatrix": ["[".to_string(), "]".to_string()],
// "Bmatrix": ["\\{".to_string(), "\\}".to_string()],
// "vmatrix": ["|".to_string(), "|".to_string()],
// "Vmatrix": ["\\Vert".to_string(), "\\Vert".to_string()],
// }[context.envName.replace("*".to_string(), "".to_string())];
// // \hskip -\arraycolsep in amsmath
// let colAlign = "c";
// let payload = {
// hskip_before_and_after: false,
// cols: [{type: "align".to_string(), align: colAlign}],
// };
// if (context.envName.charAt(context.envName.length - 1) == "*".to_string()) {
// // It's one of the mathtools starred functions.
// // Parse the optional alignment argument.
// let parser = context.parser;
// parser.consumeSpaces();
// if (parser.fetch().text == "[".to_string()) {
// parser.consume();
// parser.consumeSpaces();
// colAlign = parser.fetch().text;
// if ("lcr".indexOf(colAlign) == -1) {
// panic!("Expected l or c or r".to_string(), parser.nextToken);
// }
// parser.consume();
// parser.consumeSpaces();
// parser.expect("]".to_string());
// parser.consume();
// payload.cols = [{type: "align".to_string(), align: colAlign}];
// }
// }
// let res: ParseNode<"array"> =
// parse_array(context.parser, payload, dCellStyle(context.envName));
// // Populate cols with the correct number of column alignment specs.
// let numCols = f64::max(0, ...res.body.map((row) => row.length));
// res.cols = new Array(numCols).fill(
// {type: "align".to_string(), align: colAlign}
// );
// return delimiters ? {
// type: "leftright".to_string(),
// mode: context.mode,
// body: [res],
// left: delimiters[0],
// right: delimiters[1],
// rightColor: undefined, // \right uninfluenced by \color in array
// } : res;
// },
// html_builder,
// mathml_builder,
// });
//
// defineEnvironment({
// type: "array".to_string(),
// names: ["smallmatrix".to_string()],
// props: {
// numArgs: 0,
// },
// handler(context) {
// let payload = {arraystretch: 0.5};
// let res = parse_array(context.parser, payload, "script".to_string());
// res.col_separation_type = "small";
// return res;
// },
// html_builder,
// mathml_builder,
// });
//
// defineEnvironment({
// type: "array".to_string(),
// names: ["subarray".to_string()],
// props: {
// numArgs: 1,
// },
// handler(context, args) {
// // Parsing of {subarray} is similar to {array}
// let symNode = checkSymbolNodeType(args[0]);
// let colalign: AnyParseNode[] =
// symNode ? [args[0]] : assertNodeType(args[0], "ordgroup".to_string()).body;
// let cols = colalign.map(function(nde) {
// let node = assertSymbolNodeType(nde);
// let ca = node.text;
// // {subarray} only recognizes "l" & "c"
// if ("lc".indexOf(ca) != -1) {
// return {
// type: "align".to_string(),
// align: ca,
// };
// }
// panic!("Unknown column alignment: " + ca, nde);
// });
// if (cols.length > 1) {
// panic!("{subarray} can contain only one column".to_string());
// }
// let res = {
// cols,
// hskip_before_and_after: false,
// arraystretch: 0.5,
// };
// res = parse_array(context.parser, res, "script".to_string());
// if (res.body.length > 0 &&  res.body[0].length > 1) {
// panic!("{subarray} can contain only one column".to_string());
// }
// return res;
// },
// html_builder,
// mathml_builder,
// });
//
// // A cases environment (in amsmath.sty) is almost equivalent to
// // \def\arraystretch{1.2}%
// // \left\{\begin{array}{@{}l@{\quad}l@{}} … \end{array}\right.
// // {dcases} is a {cases} environment where cells are set in \displaystyle,
// // as defined in mathtools.sty.
// // {rcases} is another mathtools environment. It's brace is on the right side.
// defineEnvironment({
// type: "array".to_string(),
// names: [
// "cases".to_string(),
// "dcases".to_string(),
// "rcases".to_string(),
// "drcases".to_string(),
// ],
// props: {
// numArgs: 0,
// },
// handler(context) {
// let payload = {
// arraystretch: 1.2,
// cols: [{
// type: "align".to_string(),
// align: "l".to_string(),
// pregap: 0,
// // TODO(kevinb) get the current style.
// // For now we use the metrics for TEXT style which is what we were
// // doing before.  Before attempting to get the current style we
// // should look at TeX's behavior especially for \over and matrices.
// postgap: 1.0, /* 1em quad */
// }, {
// type: "align".to_string(),
// align: "l".to_string(),
// pregap: 0,
// postgap: 0,
// }],
// };
// let res: ParseNode<"array"> =
// parse_array(context.parser, payload, dCellStyle(context.envName));
// return {
// type: "leftright".to_string(),
// mode: context.mode,
// body: [res],
// left: context.envName.indexOf("r".to_string()) > -1 ? "." : "\\{".to_string(),
// right: context.envName.indexOf("r".to_string()) > -1 ? "\\}" : ".".to_string(),
// rightColor: undefined,
// };
// },
// html_builder,
// mathml_builder,
// });

//
// // A gathered environment is like an array environment with one centered
// // column, but where rows are considered lines so get \jot line spacing
// // and contents are set in \displaystyle.
// defineEnvironment({
// type: "array".to_string(),
// names: ["gathered".to_string(), "gather".to_string(), "gather*".to_string()],
// props: {
// numArgs: 0,
// },
// handler(context) {
// if (utils.contains(["gather".to_string(), "gather*".to_string()], context.envName)) {
// validateAmsEnvironmentContext(context);
// }
// let res = {
// cols: [{
// type: "align".to_string(),
// align: "c".to_string(),
// }],
// add_jot: true,
// col_separation_type: "gather".to_string(),
// auto_tag: getAutoTag(context.envName),
// empty_single_row: true,
// leqno: context.parser.settings.leqno,
// };
// return parse_array(context.parser, res, "display".to_string());
// },
// html_builder,
// mathml_builder,
// });
//
// // alignat environment is like an align environment, but one must explicitly
// // specify maximum number of columns in each row, and can adjust spacing between
// // each columns.
// defineEnvironment({
// type: "array".to_string(),
// names: ["alignat".to_string(), "alignat*".to_string(), "alignedat".to_string()],
// props: {
// numArgs: 1,
// },
// handler: alignedHandler,
// html_builder,
// mathml_builder,
// });
//
// defineEnvironment({
// type: "array".to_string(),
// names: ["equation".to_string(), "equation*".to_string()],
// props: {
// numArgs: 0,
// },
// handler(context) {
// validateAmsEnvironmentContext(context);
// let res = {
// auto_tag: getAutoTag(context.envName),
// empty_single_row: true,
// single_row: true,
// max_num_cols: 1,
// leqno: context.parser.settings.leqno,
// };
// return parse_array(context.parser, res, "display".to_string());
// },
// html_builder,
// mathml_builder,
// });
//
// defineEnvironment({
// type: "array".to_string(),
// names: ["CD".to_string()],
// props: {
// numArgs: 0,
// },
// handler(context) {
// validateAmsEnvironmentContext(context);
// return parseCD(context.parser);
// },
// html_builder,
// mathml_builder,
// });
//

//

//
// // Catch \hline outside array environment
// lazy_static!{
//     type: "text".to_string(), // Doesn't matter what this is.
//     names: ["\\hline".to_string(), "\\hdashline".to_string()],
//     props: {
//         numArgs: 0,
//         allowedInText: true,
//         allowedInMath: true,
//     },
//     handler(context, args) {
//         panic!(
//             `${context.funcName} valid only within array environment`);
//     },
// });
//
