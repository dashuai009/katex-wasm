/**
 * This file does conversion between units.  In particular, it provides
 * calculateSize to convert other units into ems.
 */
use std::collections::HashMap;
// This table gives the number of TeX pts in one of each *absolute* TeX unit.
// Thus, multiplying a length by this number converts the length from units
// into pts.  Dividing the result by ptPerEm gives the number of ems
// *assuming* a font size of ptPerEm (normal size, normal style).

lazy_static! {
    static ref PT_PER_UNIT: HashMap<&'static  str, f64> = {
        let mut m = HashMap::from([
            // https://en.wikibooks.org/wiki/LaTeX/Lengths and
            // https://tex.stackexchange.com/a/8263
            ("pt", 1.0),            // TeX point
            ("mm",7227.0 / 2540.0),// millimeter
            ("cm",7227.0 / 254.0), // centiimeter
            ("in",72.27),      // inch
            ("bp",803.0 / 800.0),  // big (PostScript) points
            ("pc",12.0),         // pica
            ("dd",1238.0 / 1157.0),// didot
            ("cc",14856.0 / 1157.0), // cicero (12 didot)
            ("nd",685.0 / 642.0),  // new didot
            ("nc",1370.0 / 107.0), // new cicero (12 new didot)
            ("sp",1.0 / 65536.0),  // scaled point (TeX's internal smallest unit)
            // https://tex.stackexchange.com/a/41371
            ("px",803.0 / 800.0),  // \pdfpxdimen defaults to 1 bp in pdfTeX and LuaTeX
        ]);
        return m;
    };
}

#[derive(Debug, Clone)]
pub struct Measurement {
    pub number: f64,
    pub unit: String,
}

impl Measurement {
    /**
     * Determine whether the specified unit (either a string defining the unit
     * or a "size" parse node containing a unit field) is valid.
     */
    pub fn unit_is_valid(&self) -> bool {
        return match self.unit.as_str() {
            "ex" | "em" | "mu" => {
                // relative unit
                true
            }
            _ => PT_PER_UNIT.get(self.unit.as_str()).is_some(),
        };
    }

    pub fn new(number: f64, unit: String) -> Measurement {
        Measurement { number, unit }
    }
}

/*
 * Convert a "size" parse node (with numeric "number" and string "unit" fields,
 * as parsed by functions.js argType "size") into a CSS em value for the
 * current style/scale.  `options` gives the current options.
 */
pub fn calculate_size(size_value: &Measurement, options: &crate::Options::Options) -> f64 {
    let mut scale = 1.0;
    if let Some(u) = PT_PER_UNIT.get(size_value.unit.as_str()) {
        // Absolute units
        scale = u  // Convert unit to pt
           / options.get_font_metrics().ptPerEm  // Convert pt to CSS em
           / options.sizeMultiplier; // Unscale to make absolute units
    } else if size_value.unit == "mu" {
        // `mu` units scale with scriptstyle/scriptscriptstyle.
        scale = options.clone().get_font_metrics().cssEmPerMu;
    } else {
        // Other relative units always refer to the *textstyle* font
        // in the current size.
        let unit_options = if options.get_style().isTight() {
            // isTight() means current style is script/scriptscript.
            options.having_style(&options.get_style().text())
        } else {
            options.clone()
        };
        // TODO: In TeX these units are relative to the quad of the current
        // *text* font, e.g. cmr10. KaTeX instead uses values from the
        // comparably-sized *Computer Modern symbol* font. At 10pt, these
        // match. At 7pt and 5pt, they differ: cmr7=1.138894, cmsy7=1.170641;
        // cmr5=1.361133, cmsy5=1.472241. Consider $\scriptsize a\kern1emb$.
        // TeX \showlists shows a kern of 1.13889 * fontsize;
        // KaTeX shows a kern of 1.171 * fontsize.
        scale = match size_value.unit.as_str() {
            "ex" => unit_options.get_font_metrics().xHeight,
            "em" => unit_options.get_font_metrics().quad,
            _ => {
                panic!("Invalid unit: '{}'", size_value.unit)
            }
        };
        if &unit_options != options {
            scale *= unit_options.sizeMultiplier / options.sizeMultiplier;
        }
        // console.log(`scale2 = ${scale}`)
    }
    return f64::min(size_value.number * scale, options.maxSize);
}

/**
 * Round `n` to 4 decimal places, or to the nearest 1/10,000th em. See
 * https://github.com/KaTeX/KaTeX/pull/2460.
 */
pub fn make_em(n: f64) -> String {
    format!("{:.4}em", n)
}
