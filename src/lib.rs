#[macro_use]
extern crate lazy_static;

mod Namespace;
mod Options;
mod ParseError;
mod build;
mod dom_tree;
mod metrics;
//mod ParseError;
//mod Setting;
// mod Lexer;
mod Style;
mod settings;
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
use Namespace::*;
use Options::*;
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
