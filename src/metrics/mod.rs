mod extra_character_map;
mod fontMetricsData;
pub(crate) mod public;
pub(crate) mod sigmas_and_xis;

use crate::{
    metrics::{extra_character_map::*, fontMetricsData::*, sigmas_and_xis::*},
    types::Mode,
    unicodeScripts::supportedCodepoint,
    utils::console_log,
    utils::log,
};
use public::*;
use std::{collections::HashMap, str::FromStr};
use wasm_bindgen::prelude::*;

// macro_rules! get_metrics_map {
//     ($font:expr) => {
//         match $font.as_str(){
//             "AMS-Regular" => AMS_Regular_map,
//             "Caligraphic-Regular" => Caligraphic_Regular_map,
//             "Fraktur-Regular" => Fraktur_Regular_map,
//             "Main-Bold" => Main_Bold_map,
//             "Main-BoldItalic" => Main_BoldItalic_map,
//             "Main-Italic" => Main_Italic_map,
//             "Main-Regular" => Main_Regular_map,
//             "Math-BoldItalic" => Math_BoldItalic_map,
//             "Math-Italic" => Math_Italic_map,
//             "SansSerif-Bold" => SansSerif_Bold_map,
//             "SansSerif-Italic" => SansSerif_Italic_map,
//             "SansSerif-Regular" => SansSerif_Regular_map,
//             "Script-Regular" => Script_Regular_map,
//             "Size1-Regular" => Size1_Regular_map,
//             "Size2-Regular" => Size2_Regular_map,
//             "Size3-Regular" => Size3_Regular_map,
//             "Size4-Regular" => Size4_Regular_map,
//             "Typewriter-Regular" => Typewriter_Regular_map,
//             _=> AMS_Regular_map
//         }
//     };
// }

// pub(crate) use get_metrics_map;

#[wasm_bindgen]
pub fn _get_font_char(font: String, character: String) -> Option<CharacterMetrics> {
    //console_log!("_get_font_char: font = {} character = {}",font,character);
    get_char_metrics(&font, character)
}

/**
 * This function is a convenience function for looking up information in the
 * metricMap table. It takes a character as a string, and a font.
 *
 * Note: the `width` property may be undefined if fontMetricsData.js wasn't
 * built using `Make extended_metrics`.
 */
pub fn get_character_metrics(
    character: &str,
    font: &str,
    mode: Mode,
) -> Option<CharacterMetrics> {
    let mut ch = character.chars().next().unwrap();
    let mut metrics = get_char_metrics(font, (ch as u32).to_string());
    let _extra_char = extra_character_map.lock().unwrap();
    let tmp_ch = _extra_char.get(&ch);
    if metrics.is_none() && tmp_ch.is_some() {
        metrics = get_char_metrics(font, (tmp_ch.unwrap().to_owned() as u32).to_string());
        ch = tmp_ch.unwrap().clone();
    }

    if metrics.is_none() && mode == Mode::text {
        // We don't typically have font metrics for Asian scripts.
        // But since we support them in text mode, we need to return
        // some sort of metrics.
        // So if the character is in a script we support but we
        // don't have metrics for it, just use the metrics for
        // the Latin capital letter M. This is close enough because
        // we (currently) only care about the height of the glpyh
        // not its width.
        if supportedCodepoint(ch as u32 as f64) {
            metrics = get_char_metrics(&font, String::from("77")); // 77 is the charcode for 'M'
        }
    }

    metrics
}

#[wasm_bindgen]
pub fn wasm_getCharacterMetrics(c: String, f: String, m: String) -> Option<CharacterMetrics> {
    get_character_metrics(&c, &f, Mode::from_str(m.as_str()).unwrap())
}

#[wasm_bindgen]
pub fn getGlobalMetrics(size: f64) -> FontMetrics {
    // console_log!("getGlobalMetrics size = {}",size);
    let sizeIndex = if (size >= 5.0) {
        0
    } else if (size >= 3.0) {
        1
    } else {
        2
    };
    return sigmasAndXis[sizeIndex].clone();
}

pub fn get_global_metrics(size: f64) -> &'static FontMetrics {
    // console_log!("getGlobalMetrics size = {}",size);
    let size_index = if size >= 5.0 {
        0
    } else if size >= 3.0 {
        1
    } else {
        2
    };
    return &sigmasAndXis[size_index];
}
