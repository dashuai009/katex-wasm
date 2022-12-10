use std::{any::Any, sync::Arc};
use std::collections::HashMap;
use std::fmt::Debug;
use struct_format::parse_node_type;

use crate::{
    sourceLocation::SourceLocation,
    symbols::public::{Group, Mode},
    token::Token,
    types::StyleStr,
    units::Measurement,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
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

pub trait AnyParseNode: ParseNodeToAny + NodeClone + Debug{
    fn get_type(&self) -> &str;
}
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(parse_node_type, Clone, Debug)]
pub struct cdlabel {
    mode: Mode,
    loc: Option<SourceLocation>,
    side: String,
    label: Box<dyn AnyParseNode>,
}
#[derive(parse_node_type, Clone, Debug)]
pub struct cdlabelparent {
    mode: Mode,
    loc: Option<SourceLocation>,
    fragment: Box<dyn AnyParseNode>,
}
#[derive(parse_node_type, Clone, Debug)]
pub struct color {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub color: String,
    pub body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct color_token {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub color: String,
}
// To avoid requiring run-time type assertions, this more carefully captures
// the requirements on the fields per the op.rs htmlBuilder logic:
// - `body` and `value` are NEVER set simultanouesly.
// - When `symbol` is true, `body` is set.
#[derive(parse_node_type, Clone, Debug)]
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

#[derive(parse_node_type, Clone, Debug)]
pub struct ordgroup {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub(crate) body: Vec<Box<dyn AnyParseNode>>,
    pub semisimple: bool,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct raw {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub string: String,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct size {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub value: Measurement,
    pub isBlank: bool,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct styling {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub style: StyleStr,
    pub(crate) body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct supsub {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub base: Option<Box<dyn AnyParseNode>>,
    pub sup: Option<Box<dyn AnyParseNode>>,
    pub sub: Option<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct tag {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Vec<Box<dyn AnyParseNode>>,
    pub tag: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct text {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Vec<Box<dyn AnyParseNode>>,
    pub font: Option<String>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct url {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub url: String,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct verb {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: String,
    pub star: bool,
}

#[derive(Clone,PartialEq, Debug)]
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
#[derive(parse_node_type, Clone, Debug)]
pub struct atom {
    pub family: Atom,
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct mathord {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct spacing {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct textord {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}
// These "-token" types don't have corresponding HTML/MathML builders.
#[derive(parse_node_type, Clone, Debug)]
pub struct accent_token {
    pub mode: Mode,
    loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct op_token {
    pub mode: Mode,
    loc: Option<SourceLocation>,
    pub text: String,
}
// From functions.js and functions/*.js. See also "color", "op", "styling",
// and "text" above.
#[derive(parse_node_type, Clone, Debug)]
pub struct accent {
    pub mode: Mode,
    pub(crate) loc: Option<SourceLocation>,
    pub label: String,
    pub isStretchy: bool,
    pub isShifty: bool,
    pub base: Option<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct accentUnder {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub isStretchy: bool,
    pub isShifty: bool,
    pub base: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct cr {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub newLine: bool,
    pub size: Option<Measurement>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct delimsizing {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub size: usize,    // 1 | 2 | 3 | 4,
    pub mclass: String, //"mopen" | "mclose" | "mrel" | "mord",
    pub delim: String,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct enclose {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub backgroundColor: Option<String>,
    pub borderColor: Option<String>,
    pub body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct environment {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub name: String,
    pub name_group: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct font {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub font: String,
    pub body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct genfrac {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub continued: bool,
    pub numer: Box<dyn AnyParseNode>,
    pub denom: Box<dyn AnyParseNode>,
    pub hasBarLine: bool,
    pub leftDelim: Option<String>,
    pub rightDelim: Option<String>,
    pub size: String, //StyleStr | "auto",
    pub barSize: Option<Measurement>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct hbox {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct horizBrace {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub isOver: bool,
    pub base: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct href {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub href: String,
    pub body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct html {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub attributes: HashMap<String,String>,
    pub body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct htmlmathml {
    mode: Mode,
    loc: Option<SourceLocation>,
    html: Vec<Box<dyn AnyParseNode>>,
    mathml: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct includegraphics {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub alt: String,
    pub width: Measurement,
    pub height: Measurement,
    pub totalheight: Measurement,
    pub src: String,
}

#[derive(parse_node_type, Clone, Debug)]
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

#[derive(parse_node_type, Clone, Debug)]
pub struct internal {
    mode: Mode,
    loc: Option<SourceLocation>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct kern {
    mode: Mode,
    loc: Option<SourceLocation>,
    dimension: Measurement,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct lap {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub alignment: String,
    pub body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct leftright {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Vec<Box<dyn AnyParseNode>>,
    pub left: String,
    pub right: String,
    pub right_color: Option<String>, // undefined means "inherit"
}

#[derive(parse_node_type, Clone, Debug)]
pub struct leftright_right {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub delim: String,
    pub color: Option<String>, // undefined means "inherit"
}

#[derive(parse_node_type, Clone, Debug)]
pub struct mathchoice {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub display: Vec<Box<dyn AnyParseNode>>,
    pub text: Vec<Box<dyn AnyParseNode>>,
    pub script: Vec<Box<dyn AnyParseNode>>,
    pub scriptscript: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct middle {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub delim: String,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct mclass {
    pub(crate) mode: Mode,
    pub(crate) loc: Option<SourceLocation>,
    pub mclass: String,
    pub body: Vec<Box<dyn AnyParseNode>>,
    pub is_character_box: bool,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct operatorname {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Vec<Box<dyn AnyParseNode>>,
    pub alwaysHandleSupSub: bool,
    pub limits: bool,
    parentIsSupSub: bool,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct overline {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct phantom {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct hphantom {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct vphantom {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct pmb {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub mclass: String,
    pub body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct raisebox {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub dy: Measurement,
    pub body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct rule {
    mode: Mode,
    loc: Option<SourceLocation>,
    shift: Option<Measurement>,
    width: Measurement,
    height: Measurement,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct sizing {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub size: usize,
    pub body: Vec<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct smash {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<dyn AnyParseNode>,
    pub smash_height: bool,
    pub smash_depth: bool,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct sqrt {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<dyn AnyParseNode>,
    pub index: Option<Box<dyn AnyParseNode>>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct underline {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct vcenter {
    mode: Mode,
    loc: Option<SourceLocation>,
    body: Box<dyn AnyParseNode>,
}

#[derive(parse_node_type, Clone, Debug)]
pub struct xArrow {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub body: Box<dyn AnyParseNode>,
    pub below: Option<Box<dyn AnyParseNode>>,
}
