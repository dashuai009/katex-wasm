/**
 * This file does conversion between units.  In particular, it provides
 * calculateSize to convert other units into ems.
 */
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use std::{cmp::min, collections::HashMap};

use crate::Options::Options;
// This table gives the number of TeX pts in one of each *absolute* TeX unit.
// Thus, multiplying a length by this number converts the length from units
// into pts.  Dividing the result by ptPerEm gives the number of ems
// *assuming* a font size of ptPerEm (normal size, normal style).

lazy_static! {
    static ref ptPerUnit: HashMap<&'static  str, f64> = {
        let mut m = HashMap::new();
        // https://en.wikibooks.org/wiki/LaTeX/Lengths and
        // https://tex.stackexchange.com/a/8263
        m.insert("pt", 1.0);            // TeX point
        m.insert("mm",7227.0 / 2540.0);// millimeter
        m.insert("cm",7227.0 / 254.0); // centiimeter
        m.insert("in",72.27);      // inch
        m.insert("bp",803.0 / 800.0);  // big (PostScript) points
        m.insert("pc",12.0);         // pica
        m.insert("dd",1238.0 / 1157.0);// didot
        m.insert("cc",14856.0 / 1157.0); // cicero (12 didot)
        m.insert("nd",685.0 / 642.0);  // new didot
        m.insert("nc",1370.0 / 107.0); // new cicero (12 new didot)
        m.insert("sp",1.0 / 65536.0);  // scaled point (TeX's internal smallest unit)
          // https://tex.stackexchange.com/a/41371
        m.insert("px",803.0 / 800.0);  // \pdfpxdimen defaults to 1 bp in pdfTeX and LuaTeX
        return m;
    };
}

// Dictionary of relative units, for fast validity testing.
lazy_static! {
    static ref relativeUnit: HashMap<&'static str, bool> = {
        let mut m = HashMap::new();
        m.insert("ex", true);
        m.insert("em", true);
        m.insert("mu", true);
        m
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Measurement {
    pub number: f64,
    pub unit: String,
}

#[wasm_bindgen]
impl Measurement {
    /**
     * Determine whether the specified unit (either a string defining the unit
     * or a "size" parse node containing a unit field) is valid.
     */
    pub fn validUnit(&self) -> bool {
        return ptPerUnit.get(self.unit.as_str()).is_some()
            || relativeUnit.get(self.unit.as_str()).is_some();
    }

    #[wasm_bindgen(constructor)]
    pub fn new(number: f64, unit: String) -> Measurement {
        Measurement { number, unit }
    }
}
/**
 * Determine whether the specified unit (either a string defining the unit
 * or a "size" parse node containing a unit field) is valid.
 */
#[wasm_bindgen]
pub fn validUnit(unit: &js_sys::Object) -> bool {
    let q = js_sys::JsString::from("unit");
    let f = js_sys::Reflect::get(unit, &q).unwrap();
    let t = js_sys::JsString::try_from(&f).unwrap();
    return ptPerUnit.get(String::from(t).as_str()).is_some()
        || relativeUnit.get(String::from(t).as_str()).is_some();
}

/*
 * Convert a "size" parse node (with numeric "number" and string "unit" fields,
 * as parsed by functions.js argType "size") into a CSS em value for the
 * current style/scale.  `options` gives the current options.
 */
pub fn calculate_size(sizeValue: &Measurement, options: &Options) -> f64 {
    let mut scale = 1.0;
    if let Some(u) = ptPerUnit.get(sizeValue.unit.as_str()) {
        // Absolute units
        scale = u  // Convert unit to pt
           / options.clone().fontMetrics().ptPerEm  // Convert pt to CSS em
           / options.sizeMultiplier; // Unscale to make absolute units
    } else if sizeValue.unit == "mu" {
        // `mu` units scale with scriptstyle/scriptscriptstyle.
        scale = options.clone().fontMetrics().cssEmPerMu;
    } else {
        // Other relative units always refer to the *textstyle* font
        // in the current size.
        let unitOptions;
        if options.get_style().isTight() {
            // isTight() means current style is script/scriptscript.
            unitOptions = options.havingStyle(&options.get_style().text());
        } else {
            unitOptions = options.clone();
        }
        // TODO: In TeX these units are relative to the quad of the current
        // *text* font, e.g. cmr10. KaTeX instead uses values from the
        // comparably-sized *Computer Modern symbol* font. At 10pt, these
        // match. At 7pt and 5pt, they differ: cmr7=1.138894, cmsy7=1.170641;
        // cmr5=1.361133, cmsy5=1.472241. Consider $\scriptsize a\kern1emb$.
        // TeX \showlists shows a kern of 1.13889 * fontsize;
        // KaTeX shows a kern of 1.171 * fontsize.
        if (sizeValue.unit == "ex") {
            scale = unitOptions.clone().fontMetrics().xHeight;
        } else if (sizeValue.unit == "em") {
            scale = unitOptions.clone().fontMetrics().quad;
        } else {
            //throw new ParseError("Invalid unit: '" + sizeValue.unit + "'");
        }
        // Todo
        // if (unitOptions != options) {
        // scale *= unitOptions.sizeMultiplier / options.sizeMultiplier;
        // }
        // console.log(`scale2 = ${scale}`)
    }
    return f64::min(sizeValue.number * scale, options.maxSize);
}

/**
 * Round `n` to 4 decimal places, or to the nearest 1/10,000th em. See
 * https://github.com/KaTeX/KaTeX/pull/2460.
 */
#[wasm_bindgen(js_name = makeEm)]
pub fn make_em(n: f64) -> String {
    format!("{:.4}em", n)
}
