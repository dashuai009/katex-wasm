use crate::dom_tree::symbol_node::SymbolNode;
use crate::metrics::public::CharacterMetrics;
use crate::types::Mode;
use crate::Options::Options;
use crate::{get_character_metrics, get_symbol};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct TmpSymbol {
    pub value: String,
    pub metrics: Option<CharacterMetrics>,
}

/**
 * Looks up the given symbol in fontMetrics, after applying any symbol
 * replacements defined in symbol.js
 */
pub fn lookup_symbol(
    value: String,
    // TODO(#963): Use a union type for this.
    font_name: String,
    mode: Mode,
) -> TmpSymbol {
    // Replace the value with its replaced value from symbol.js
    let tmp_metrics = get_character_metrics(&value, font_name, mode);

    // if tmp.is_some_and(|&t| t.replace.is_some()) {}
    if let Some(tmp) = get_symbol(mode, &value) {
        if let Some(tmp_replace) = tmp.replace {
            return TmpSymbol {
                value: tmp_replace,
                metrics: tmp_metrics,
            };
        }
    }

    return TmpSymbol {
        value: value,
        metrics: tmp_metrics,
    };
}

#[wasm_bindgen]
pub fn _lookup_symbol(
    value: String,
    // TODO(#963): Use a union type for this.
    font_name: String,
    mode: String,
) -> TmpSymbol {
    return lookup_symbol(value, font_name, Mode::from_str(mode.as_str()).unwrap());
}

/**
 * Makes a symbolNode after translation via the list of symbols in symbols.js.
 * Correctly pulls out metrics for the character, and optionally takes a list of
 * classes to be attached to the node.
 *
 * TODO: make argument order closer to makeSpan
 * TODO: add a separate argument for math class (e.g. `mop`, `mbin`), which
 * should if present come first in `classes`.
 * TODO(#953): Make `options` mandatory and always pass it in.
 */
pub fn make_symbol(
    value: String,
    font_name: String,
    mode: Mode,
    options: Option<&Options>,
    classes: Vec<String>,
) -> SymbolNode {
    let lookup = lookup_symbol(value, font_name, mode);
    let value = lookup.value;

    let mut symbol_node = SymbolNode::new("init_node".to_string());
    if let Some(metrics) = lookup.metrics {
        let mut italic = metrics.italic;
        if let Some(opt) = options.clone() {
            if opt.font == "mathit" {
                italic = 0.0;
            }
        }
        if mode == Mode::text {
            italic = 0.0;
        }
        symbol_node = SymbolNode::new(value);
        symbol_node.height = metrics.height;
        symbol_node.depth = metrics.depth;
        symbol_node.italic = italic;
        symbol_node.skew = metrics.skew;
        symbol_node.width = metrics.width;
        symbol_node.set_classes(classes);
    } else {
        // TODO(emily): Figure out a good way to only print this in development
        //         typeof console !== "undefined" && console.warn("No character metrics " +
        //                                                        `for '${value}' in style '${fontName}' and mode '${mode}'`);

        symbol_node = SymbolNode::new(value);
        symbol_node.height = 0.0;
        symbol_node.depth = 0.0;
        symbol_node.italic = 0.0;
        symbol_node.skew = 0.0;
        symbol_node.width = 0.0;
        symbol_node.set_classes(classes);
    }

    if let Some(opt) = options {
        symbol_node.maxFontSize = opt.sizeMultiplier;
        if (opt.style().isTight()) {
            symbol_node.push_class("mtight".to_string());
        }
        let color = opt.getColor();
        // if (color) {
        symbol_node.set_style_color(Some(color));
        // }
    }

    return symbol_node;
}

#[wasm_bindgen]
pub fn canCombine(prev: &SymbolNode, next: &SymbolNode) -> bool {
    return SymbolNode::can_combine(prev, next);
}
#[wasm_bindgen]
pub fn MakeSymbol(
    value: String,
    font_name: String,
    _mode: String,
    options: &Options,
    classes: js_sys::Array,
) -> SymbolNode {
    let mut c = vec![];
    for cl in classes.to_vec().iter() {
        if let Some(t) = cl.as_string() {
            c.push(t);
        }
    }
    let mode = Mode::from_str(_mode.as_str()).unwrap();
    make_symbol(value, font_name, mode, Some(options), c)
}

#[wasm_bindgen]
pub fn MakeSymbol_none(
    value: String,
    font_name: String,
    _mode: String,
    classes: js_sys::Array,
) -> SymbolNode {
    let mut c = vec![];
    for cl in classes.to_vec().iter() {
        //classes fontShape 可能是undefined
        if let Some(t) = cl.as_string() {
            c.push(t);
        }
    }
    let mode = Mode::from_str(_mode.as_str()).unwrap();
    make_symbol(value, font_name, mode, None, c)
}
