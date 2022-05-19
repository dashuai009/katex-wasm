
#[macro_use]
extern crate lazy_static;

mod domTree;
mod Namespace;
mod sourceLocation;
mod spacingData;
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

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;




use domTree::*;
use Namespace::*;
use sourceLocation::*;
use spacingData::*;
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