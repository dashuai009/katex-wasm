#[macro_use]
extern crate lazy_static;
extern crate core;

// mod define;
mod Namespace;
mod Options;
mod build;
mod define;
mod dom_tree;
mod metrics;
mod parse_error;
//mod ParseError;
//mod Setting;
// mod Lexer;
mod Lexer;
mod Parser;
mod Style;
mod katex;
mod mathML_tree;
pub mod parse;
mod parse_node;
pub mod settings;
mod sourceLocation;
mod spacingData;
mod stretchy;
mod svgGeometry;
mod symbols;
mod token;
mod tree;
mod types;
mod unicodeAccents;
mod unicodeScripts;
mod unicodeSupOrSub;
mod unicodeSysmbols;
mod units;
mod utils;
mod wideCharacter;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use metrics::*;
use parse_node::types::AnyParseNode;
use Namespace::*;
//use ParseError::*;
//use Setting::*;
use sourceLocation::*;
use spacingData::*;
use stretchy::*;
use svgGeometry::*;
use symbols::*;
use token::*;
use tree::*;
use unicodeAccents::*;
use unicodeScripts::*;
use unicodeSysmbols::*;
use units::*;
use wideCharacter::*;
use Style::*;

type HtmlBuilder<NODETYPE: AnyParseNode> =
    fn(ParseNode: NODETYPE, crate::Options::Options) -> Box<dyn HtmlDomNode>;
