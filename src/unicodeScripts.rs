/*
 * This file defines the Unicode scripts and script families that we
 * support. To add new scripts or families, just add a new entry to the
 * scriptData array below. Adding scripts to the scriptData array allows
 * characters from that script to appear in \text{} environments.
 */
use wasm_bindgen::prelude::*;

type BlockType = [i64; 2];
/**
 * Each script or script family has a name and an array of blocks.
 * Each block is an array of two numbers which specify the start and
 * end points (inclusive) of a block of Unicode codepoints.
 */
pub struct Script {
    name: &'static str,
    blocks: &'static [BlockType],
}

const scriptData: [Script; 7] = [
    Script {
        // Latin characters beyond the Latin-1 characters we have metrics for.
        // Needed for Czech, Hungarian and Turkish text, for example.
        name: "latin",
        blocks: &[
            [0x0100, 0x024f], // Latin Extended-A and Latin Extended-B
            [0x0300, 0x036f], // Combining Diacritical marks
        ],
    },
    Script {
        // The Cyrillic script used by Russian and related languages.
        // A Cyrillic subset used to be supported as explicitly defined
        // symbols in symbols.js
        name: "cyrillic",
        blocks: &[[0x0400, 0x04ff]],
    },
    Script {
        // Armenian
        name: "armenian",
        blocks: &[[0x0530, 0x058F]],
    },
    Script {
        // The Brahmic scripts of South and Southeast Asia
        // Devanagari (0900–097F)
        // Bengali (0980–09FF)
        // Gurmukhi (0A00–0A7F)
        // Gujarati (0A80–0AFF)
        // Oriya (0B00–0B7F)
        // Tamil (0B80–0BFF)
        // Telugu (0C00–0C7F)
        // Kannada (0C80–0CFF)
        // Malayalam (0D00–0D7F)
        // Sinhala (0D80–0DFF)
        // Thai (0E00–0E7F)
        // Lao (0E80–0EFF)
        // Tibetan (0F00–0FFF)
        // Myanmar (1000–109F)
        name: "brahmic",
        blocks: &[[0x0900, 0x109F]],
    },
    Script {
        name: "georgian",
        blocks: &[[0x10A0, 0x10ff]],
    },
    Script {
        // Chinese and Japanese.
        // The "k" in cjk is for Korean, but we've separated Korean out
        name: "cjk",
        blocks: &[
            [0x3000, 0x30FF], // CJK symbols and punctuation, Hiragana, Katakana
            [0x4E00, 0x9FAF], // CJK ideograms
            [0xFF00, 0xFF60], // Fullwidth punctuation
                              // TODO: add halfwidth Katakana and Romanji glyphs
        ],
    },
    Script {
        // Korean
        name: "hangul",
        blocks: &[[0xAC00, 0xD7AF]],
    },
];

/**
 * Given a codepoint, return the name of the script or script family
 * it is from, or null if it is not part of a known block
 */
#[wasm_bindgen]
pub fn scriptFromCodepoint(codepoint: f64) -> Option<String> {
    for script in scriptData {
        for block in script.blocks {
            if block[0] as f64<= codepoint && codepoint <= block[1] as f64 {
                return Some(String::from(script.name));
            }
        }
    }
    return None;
}

/**
 * Given a codepoint, return true if it falls within one of the
 * scripts or script families defined above and false otherwise.
 *
 * Micro benchmarks shows that this is faster than
 * /[\u3000-\u30FF\u4E00-\u9FAF\uFF00-\uFF60\uAC00-\uD7AF\u0900-\u109F]/.test()
 * in Firefox, Chrome and Node.
 */

 
#[wasm_bindgen]
pub fn supportedCodepoint(codepoint: f64) -> bool {
    for script in scriptData {
        for block in script.blocks {
            if block[0] as f64 <= codepoint && codepoint <= block[1] as f64 {
                return true;
            }
        }
    }
    return false;
}
