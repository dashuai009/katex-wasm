
#[macro_use]
extern crate lazy_static;

mod domTree;
mod metrics;
mod Namespace;
mod Options;
//mod ParseError;
//mod Setting;
mod sourceLocation;
mod spacingData;
mod stretchy;
mod Style;
mod svgGeometry;
mod symbols;
mod wideCharacter;
mod token;
mod tree;
mod types;
mod unicodeAccents;
mod unicodeScripts;
mod unicodeSysmbols;
mod units;
mod utils;
mod settings;
mod ParseError;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;




use domTree::*;
use metrics::*;
use Namespace::*;
use Options::*;
//use ParseError::*;
//use Setting::*;
use sourceLocation::*;
use spacingData::*;
use stretchy::*;
use Style::*;
use token::*;
use tree::*;
use unicodeAccents::*;
use unicodeScripts::*;
use unicodeSysmbols::*;
use units::*;
use svgGeometry::*;
use symbols::*;
use wideCharacter::*;
