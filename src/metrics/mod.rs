mod extra_character_map;
pub mod fontMetricsData;
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
    let mut ch = match character.chars().next() {
        Some(c) => c,
        None => return None,
    };
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
