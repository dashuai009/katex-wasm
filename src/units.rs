use wasm_bindgen::prelude::*;

use std::collections::HashMap;
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
    pub fn validUnit(&self) -> bool {
        return ptPerUnit.get(self.unit.as_str()).is_some()
            || relativeUnit.get(self.unit.as_str()).is_some();
    }
}

#[wasm_bindgen]
pub fn validUnit(unit: &js_sys::Object) -> bool {
    let q = js_sys::JsString::from("unit");
    let f = js_sys::Reflect::get(unit, &q).unwrap();
    let t = js_sys::JsString::try_from(&f).unwrap();
    return ptPerUnit.get(String::from(t).as_str()).is_some()
        || relativeUnit.get(String::from(t).as_str()).is_some();
}
