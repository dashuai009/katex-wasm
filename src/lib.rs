#[macro_use]
extern crate lazy_static;

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
pub mod parse_node;
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
mod wide_character;
mod delimiter;

pub use crate::katex::render_to_string;
pub use crate::parse::parseTree;
pub use crate::parse_node::types::AnyParseNode;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Use the WASM allocator only for single-threaded WASM. Native and atomics
// builds retain their allocator; an explicit wee_alloc selection still wins.
#[cfg(all(target_family = "wasm", not(target_feature = "atomics"), not(feature = "wee_alloc")))]
#[global_allocator]
static TALC: wasm_allocator::SmallBlockCache<talc::wasm::WasmDynamicTalc> =
    wasm_allocator::SmallBlockCache::new(talc::wasm::new_wasm_dynamic_allocator());

#[cfg(any(test, all(target_family = "wasm", not(target_feature = "atomics"))))]
mod wasm_allocator;

use metrics::*;
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
use wide_character::*;
use Style::*;
