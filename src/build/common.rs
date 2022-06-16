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

//
// /**
//  * Makes a symbolNode after translation via the list of symbols in symbols.js.
//  * Correctly pulls out metrics for the character, and optionally takes a list of
//  * classes to be attached to the node.
//  *
//  * TODO: make argument order closer to makeSpan
//  * TODO: add a separate argument for math class (e.g. `mop`, `mbin`), which
//  * should if present come first in `classes`.
//  * TODO(#953): Make `options` mandatory and always pass it in.
//  */
// pub fn make_symbol(value:String, font_name:String, mode:Mode, options:Option<Options>, classes:Option<Box<JsValue>>){
//     let lookup = lookup_symbol(value, fontName, mode);
//     let metrics = lookup.metrics;
//     let value = lookup.value;
//
//     let symbolNode;
//     console.log(metrics);
//     if metrics.is_some() {
//         let italic = metrics.italic;
//         if mode == "text" || (options && options.font === "mathit") {
//             italic = 0;
//         }
//         symbolNode = new SymbolNode(
//             value, metrics.height, metrics.depth, italic, metrics.skew,
//             metrics.width, classes);
//     } else {
// // TODO(emily): Figure out a good way to only print this in development
//         typeof console !== "undefined" && console.warn("No character metrics " +
//                                                        `for '${value}' in style '${fontName}' and mode '${mode}'`);
//         symbolNode = new SymbolNode(value, 0, 0, 0, 0, 0, classes);
//     }
//
//     if (options) {
//         symbolNode.maxFontSize = options.sizeMultiplier;
//         if (options.style.isTight()) {
//             symbolNode.classes.push("mtight");
//         }
//         const color = options.getColor();
//         if (color) {
//             symbolNode.style.color = color;
//         }
//     }
//
//     return symbolNode;
// }
// const makeSymbol = function(
//     value: string,
//     fontName: string,
//     mode: Mode,
//     options?: Options,
//     classes?: string[],
// ): SymbolNode {
//
// };
