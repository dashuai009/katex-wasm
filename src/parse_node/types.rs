use std::{any::Any, sync::Arc};

use struct_format::parse_node_type;
use wasm_bindgen::prelude::*;

use crate::{
    sourceLocation::SourceLocation,
    symbols::public::{Group, Mode},
    token::Token,
    types::StyleStr,
    units::Measurement,
};

pub trait ParseNodeToAny{
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub trait NodeClone {
    fn clone_box(&self) -> Box<dyn AnyParseNode>;
}

impl<T> NodeClone for T
where
    T: 'static + AnyParseNode + Clone,
{
    fn clone_box(&self) -> Box<dyn AnyParseNode> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn AnyParseNode> {
    fn clone(&self) -> Box<dyn AnyParseNode> {
        self.clone_box()
    }
}

pub trait AnyParseNode: ParseNodeToAny + NodeClone {
    fn get_type(&self) -> &str;
}

pub trait GetMode {
    fn get_mode(&self) -> Mode;
}
pub trait GetText {
    fn get_text(&self) -> String;
}

// impl ParseNodeToAny for raw{
//     fn as_any(&self) -> &dyn Any{
//         self
//     }
// }

impl GetMode for raw {
    fn get_mode(&self) -> Mode {
        return self.mode;
    }
}
// impl GetText for raw{
//     fn get_text(&self)->String{
//         return self.text;
//     }
// }

// #[derive(parse_node_type, Clone)]
// pub struct array{
//         mode: Mode,
//         loc:Option<SourceLocation>,
//         colSeparationType: Option<ColSeparationType>,
//         hskipBeforeAndAfter: bool,
//         addJot: bool,
//         cols: Option<AlignSpec[]>,
//         arraystretch: number,
//         body: Vec<Box<dyn AnyParseNode>>[], // List of rows in the (2D) array.
//         rowGaps: (?Measurement)[],
//         hLinesBeforeRow: Array<bool[]>,
//         // Whether each row should be automatically numbered, or an explicit tag
//         tags?: (bool | Vec<Box<dyn AnyParseNode>>)[],
//         leqno?: bool,
//         isCD?: bool,
// }
#[derive(parse_node_type, Clone)]
pub struct cdlabel {
    mode: Mode,
    loc: Option<SourceLocation>,
    side: String,
    label: Box<dyn AnyParseNode>,
}
#[derive(parse_node_type, Clone)]
pub struct cdlabelparent {
    mode: Mode,
    loc: Option<SourceLocation>,
    fragment: Box<dyn AnyParseNode>,
}
#[derive(parse_node_type, Clone)]
pub struct color {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub color: String,
    pub body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct color_token {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub color: String,
}
// To avoid requiring run-time type assertions, this more carefully captures
// the requirements on the fields per the op.js htmlBuilder logic:
// - `body` and `value` are NEVER set simultanouesly.
// - When `symbol` is true, `body` is set.
#[derive(parse_node_type, Clone)]
pub struct op {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub limits: bool,
    pub alwaysHandleSupSub: bool,
    pub suppressBaseShift: bool,
    pub parentIsSupSub: bool,
    pub symbol: bool,
    pub name: Option<String>,
    pub body: Option<Vec<Box<dyn AnyParseNode>>>,
}

#[derive(parse_node_type, Clone)]
pub struct ordgroup {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub(crate) body: Vec<Box<dyn AnyParseNode>>,
    pub semisimple: bool,
}

#[derive(parse_node_type, Clone)]
pub struct raw {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub string: String,
}

#[derive(parse_node_type, Clone)]
pub struct size {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub value: Measurement,
    pub isBlank: bool,
}

#[derive(parse_node_type, Clone)]
pub struct styling {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub style: StyleStr,
    pub(crate) body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct supsub {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub base: Option<Box<dyn AnyParseNode>>,
    pub sup: Option<Box<dyn AnyParseNode>>,
    pub sub: Option<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct tag {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Vec<Box<dyn AnyParseNode>>,
    pub tag: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct text {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Vec<Box<dyn AnyParseNode>>,
    pub font: Option<String>,
}

#[derive(parse_node_type, Clone)]
pub struct url {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub url: String,
}

#[derive(parse_node_type, Clone)]
pub struct verb {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: String,
    pub star: bool,
}

#[derive(Clone,PartialEq)]
pub enum Atom {
    bin,
    close,
    inner,
    open,
    punct,
    rel,
}

impl Atom{
    pub fn as_str(&self)->&str{
        match self{
            Atom::bin => {"bin"}
            Atom::close => {"close"}
            Atom::inner => {"inner"}
            Atom::open => {"open"}
            Atom::punct => {"punct"}
            Atom::rel => {"rel"}
        }
    }
}

impl Atom {
    pub fn from_group(g: Group) -> Atom {
        match g {
            Group::accent => panic!("can't xxxxx"),
            Group::bin => Atom::bin,
            Group::close => Atom::close,
            Group::inner => Atom::inner,
            Group::mathord => panic!("can't xxxxx"),
            Group::op => panic!("can't xxxxx"),
            Group::open => Atom::open,
            Group::punct => Atom::punct,
            Group::rel => Atom::rel,
            Group::spacing => panic!("can't xxxxx"),
            Group::textord => panic!("can't xxxxx"),
        }
    }
}
// From symbol groups, constructed in Parser.js via `symbols` lookup.
// (Some of these have "-token" suffix to distinguish them from existing
// `ParseNode` types.)
#[derive(parse_node_type, Clone)]
pub struct atom {
    pub family: Atom,
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(parse_node_type, Clone)]
pub struct mathord {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(parse_node_type, Clone)]
pub struct spacing {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(parse_node_type, Clone)]
pub struct textord {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}
// These "-token" types don't have corresponding HTML/MathML builders.
#[derive(parse_node_type, Clone)]
pub struct accent_token {
    pub mode: Mode,
    loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(parse_node_type, Clone)]
pub struct op_token {
    pub mode: Mode,
    loc: Option<SourceLocation>,
    pub text: String,
}
// From functions.js and functions/*.js. See also "color", "op", "styling",
// and "text" above.
#[derive(parse_node_type, Clone)]
pub struct accent {
    pub mode: Mode,
    pub(crate) loc: Option<SourceLocation>,
    pub label: String,
    pub isStretchy: bool,
    pub isShifty: bool,
    pub base: Option<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct accentUnder {
    mode: Mode,
    loc: Option<SourceLocation>,
    label: String,
    isStretchy: bool,
    isShifty: bool,
    base: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct cr {
    mode: Mode,
    loc: Option<SourceLocation>,
    newLine: bool,
    size: Option<Measurement>,
}

#[derive(parse_node_type, Clone)]
pub struct delimsizing {
    mode: Mode,
    loc: Option<SourceLocation>,
    size: usize,    // 1 | 2 | 3 | 4,
    mclass: String, //"mopen" | "mclose" | "mrel" | "mord",
    delim: String,
}

#[derive(parse_node_type, Clone)]
pub struct enclose {
    mode: Mode,
    loc: Option<SourceLocation>,
    label: String,
    backgroundColor: Option<String>,
    borderColor: Option<String>,
    body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct environment {
    mode: Mode,
    loc: Option<SourceLocation>,
    name: String,
    nameGroup: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct font {
    mode: Mode,
    loc: Option<SourceLocation>,
    font: String,
    pub(crate) body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct genfrac {
    mode: Mode,
    loc: Option<SourceLocation>,
    continued: bool,
    numer: Box<dyn AnyParseNode>,
    denom: Box<dyn AnyParseNode>,
    hasBarLine: bool,
    leftDelim: Option<String>,
    rightDelim: Option<String>,
    size: String, //StyleStr | "auto",
    barSize: Option<Measurement>,
}

#[derive(parse_node_type, Clone)]
pub struct hbox {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct horizBrace {
    mode: Mode,
    loc: Option<SourceLocation>,
    label: String,
    isOver: bool,
    base: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct href {
    mode: Mode,
    loc: Option<SourceLocation>,
    href: String,
    body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct html {
    mode: Mode,
    loc: Option<SourceLocation>,
    // attributes: {[String]: String},
    body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct htmlmathml {
    mode: Mode,
    loc: Option<SourceLocation>,
    html: Vec<Box<dyn AnyParseNode>>,
    mathml: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct includegraphics {
    mode: Mode,
    loc: Option<SourceLocation>,
    alt: String,
    width: Measurement,
    height: Measurement,
    totalheight: Measurement,
    src: String,
}

#[derive(parse_node_type, Clone)]
pub struct infix {
    mode: Mode,
    loc: Option<SourceLocation>,
    replace_with: String,
    size: Option<Measurement>,
    token: Option<Token>,
}

impl infix {
    pub fn get_replace_with(&self) -> String {
        self.replace_with.clone()
    }
}

#[derive(parse_node_type, Clone)]
pub struct internal {
    mode: Mode,
    loc: Option<SourceLocation>,
}

#[derive(parse_node_type, Clone)]
pub struct kern {
    mode: Mode,
    loc: Option<SourceLocation>,
    dimension: Measurement,
}

#[derive(parse_node_type, Clone)]
pub struct lap {
    mode: Mode,
    loc: Option<SourceLocation>,
    alignment: String,
    body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct leftright {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Vec<Box<dyn AnyParseNode>>,
    left: String,
    right: String,
    rightColor: Option<String>, // undefined means "inherit"
}

#[derive(parse_node_type, Clone)]
pub struct leftright_right {
    mode: Mode,
    loc: Option<SourceLocation>,
    delim: String,
    color: Option<String>, // undefined means "inherit"
}

#[derive(parse_node_type, Clone)]
pub struct mathchoice {
    mode: Mode,
    loc: Option<SourceLocation>,
    display: Vec<Box<dyn AnyParseNode>>,
    text: Vec<Box<dyn AnyParseNode>>,
    script: Vec<Box<dyn AnyParseNode>>,
    scriptscript: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct middle {
    mode: Mode,
    loc: Option<SourceLocation>,
    delim: String,
}

#[derive(parse_node_type, Clone)]
pub struct mclass {
    pub(crate) mode: Mode,
    pub(crate) loc: Option<SourceLocation>,
    pub mclass: String,
    pub body: Vec<Box<dyn AnyParseNode>>,
    pub is_character_box: bool,
}

#[derive(parse_node_type, Clone)]
pub struct operatorname {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Vec<Box<dyn AnyParseNode>>,
    pub alwaysHandleSupSub: bool,
    pub limits: bool,
    parentIsSupSub: bool,
}

#[derive(parse_node_type, Clone)]
pub struct overline {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct phantom {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct hphantom {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct vphantom {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct pmb {
    mode: Mode,
    loc: Option<SourceLocation>,
    mclass: String,
    body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct raisebox {
    mode: Mode,
    loc: Option<SourceLocation>,
    dy: Measurement,
    body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct rule {
    mode: Mode,
    loc: Option<SourceLocation>,
    shift: Option<Measurement>,
    width: Measurement,
    height: Measurement,
}

#[derive(parse_node_type, Clone)]
pub struct sizing {
    mode: Mode,
    loc: Option<SourceLocation>,
    pub size: usize,
    pub body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct smash {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Box<dyn AnyParseNode>,
    smashHeight: bool,
    smashDepth: bool,
}

#[derive(parse_node_type, Clone)]
pub struct sqrt {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Box<dyn AnyParseNode>,
    index: Option<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone)]
pub struct underline {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct vcenter {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone)]
pub struct xArrow {
    mode: Mode,
    loc: Option<SourceLocation>,
    label: String,
    body: Box<dyn AnyParseNode>,
    below: Option<Box<dyn AnyParseNode>>,
}
