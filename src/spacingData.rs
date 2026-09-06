/**
 * Describes spaces between different classes of atoms.
 */
use super::units::Measurement;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// Making the type below exact with all optional fields doesn't work due to
// - https://github.com/facebook/flow/issues/4582
// - https://github.com/facebook/flow/issues/5688
// However, since *all* fields are optional, $Shape<> works as suggested in 5688
// above.

pub type __spacings = HashMap<String, HashMap<String, Measurement>>;

lazy_static! {
    pub static ref thinspace: Mutex<Measurement> = Mutex::new(Measurement{
        number: 3.0,
        unit: String::from("mu")
    });
    static ref mediumspace: Mutex<Measurement> = Mutex::new(Measurement{
        number: 4.0,
        unit:  String::from("mu")
    });
    static ref thickspace: Mutex<Measurement> = Mutex::new(Measurement{
        number: 5.0,
        unit:  String::from("mu")
    });

// Spacing relationships for display and text styles

    static ref spacings:Mutex<__spacings>= Mutex::new({
        let mut m = HashMap::new();
        let mut mord = HashMap::new();
        let thinspace_c = thinspace.lock().unwrap();
        let mediumspace_c = mediumspace.lock().unwrap();
        let thickspace_c = thickspace.lock().unwrap();
        mord.insert(String::from("mop"), thinspace_c.clone());
        mord.insert(String::from("mbin"),mediumspace_c.clone());
        mord.insert(String::from("mrel"),thickspace_c.clone());
        mord.insert(String::from("minner"),thinspace_c.clone());
        m.insert(String::from("mord"),mord);

        let mut mop = HashMap::new();
        mop.insert(String::from("mord"), thinspace_c.clone());
        mop.insert(String::from("mop"), thinspace_c.clone());
        mop.insert(String::from("mrel"), thickspace_c.clone());
        mop.insert(String::from("minner"), thinspace_c.clone());
        m.insert(String::from("mop"),mop);

        let mut mbin = HashMap::new();
        mbin.insert(String::from("mord"), mediumspace_c.clone());
        mbin.insert(String::from("mop"), mediumspace_c.clone());
        mbin.insert(String::from("mopen"), mediumspace_c.clone());
        mbin.insert(String::from("minner"), mediumspace_c.clone());
        m.insert(String::from("mbin"),mbin);


        let mut mrel = HashMap::new();
        mrel.insert(String::from("mord"), thickspace_c.clone());
        mrel.insert(String::from("mop"), thickspace_c.clone());
        mrel.insert(String::from("mopen"), thickspace_c.clone());
        mrel.insert(String::from("minner"), thickspace_c.clone());
        m.insert(String::from("mrel"),mrel);

        let mut mclose = HashMap::new();
        mclose.insert(String::from("mop"), thinspace_c.clone());
        mclose.insert(String::from("mbin"), mediumspace_c.clone());
        mclose.insert(String::from("mrel"), thickspace_c.clone());
        mclose.insert(String::from("minner"), thinspace_c.clone());
        m.insert(String::from("mclose"),mclose);

        let mut mpunct  = HashMap::new();
        mpunct.insert(String::from("mord"), thinspace_c.clone());
        mpunct.insert(String::from("mop"), thinspace_c.clone());
        mpunct.insert(String::from("mrel"), thickspace_c.clone());
        mpunct.insert(String::from("mopen"), thinspace_c.clone());
        mpunct.insert(String::from("mclose"), thinspace_c.clone());
        mpunct.insert(String::from("mpunct"), thinspace_c.clone());
        mpunct.insert(String::from("minner"), thinspace_c.clone());
        m.insert(String::from("mpunct"),mpunct);

        let mut minner= HashMap::new();
        minner.insert(String::from("mord"), thinspace_c.clone());
        minner.insert(String::from("mop"), thinspace_c.clone());
        minner.insert(String::from("mbin"), mediumspace_c.clone());
        minner.insert(String::from("mrel"), thickspace_c.clone());
        minner.insert(String::from("mopen"), thinspace_c.clone());
        minner.insert(String::from("mpunct"), thinspace_c.clone());
        minner.insert(String::from("minner"), thinspace_c.clone());
        m.insert(String::from("minner"),minner);
        m
    });
    static ref tightSpacings:Mutex<__spacings>= Mutex::new({
        let thinspace_c = thinspace.lock().unwrap();
        let mut m = HashMap::new();
        let   mut mord = HashMap::new();
        mord.insert(String::from("mop"), thinspace_c.clone());
        m.insert(String::from("mord"),mord);

        let mut mop = HashMap::new();
        mop.insert(String::from("mord"), thinspace_c.clone());
        mop.insert(String::from("mop"),thinspace_c.clone());
        m.insert(String::from("mop"),mop);

        let mut mclose= HashMap::new();
        mclose.insert(String::from("mop"),thinspace_c.clone());
        m.insert(String::from("mclose"), mclose);

        let mut minner = HashMap::new();
        minner.insert(String::from("mop"),thinspace_c.clone());
        m.insert(String::from("minner"),minner);
        m
    });
// Spacing relationships for script and scriptscript styles

}

pub fn get_spacings(k1: &str, k2: &str) -> Option<&'static Measurement> {
    const TABLE: [[u8; 8]; 8] = [
        [0,3,4,5,0,0,0,3], [3,3,0,5,0,0,0,3],
        [4,4,0,0,4,0,0,4], [5,5,0,0,5,0,0,5],
        [0,0,0,0,0,0,0,0], [0,3,4,5,0,0,0,3],
        [3,3,0,5,3,3,3,3], [3,3,4,5,3,0,3,3],
    ];
    spacing_value(TABLE[atom_index(k1)?][atom_index(k2)?])
}
fn atom_index(name: &str) -> Option<usize> {
    Some(match name { "mord"=>0, "mop"=>1, "mbin"=>2, "mrel"=>3, "mopen"=>4, "mclose"=>5, "mpunct"=>6, "minner"=>7, _=>return None })
}
fn spacing_value(value: u8) -> Option<&'static Measurement> {
    use std::sync::LazyLock;
    static THIN: LazyLock<Measurement> = LazyLock::new(|| Measurement { number: 3.0, unit: "mu".to_owned() });
    static MEDIUM: LazyLock<Measurement> = LazyLock::new(|| Measurement { number: 4.0, unit: "mu".to_owned() });
    static THICK: LazyLock<Measurement> = LazyLock::new(|| Measurement { number: 5.0, unit: "mu".to_owned() });
    match value { 3=>Some(&THIN), 4=>Some(&MEDIUM), 5=>Some(&THICK), _=>None }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules!  console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

use std::panic;

pub fn get_tightSpacings(k1: &str, k2: &str) -> Option<&'static Measurement> {
    if (matches!(k1, "mord" | "mop" | "mclose" | "minner") && k2 == "mop") || (k1 == "mop" && k2 == "mord") {
        spacing_value(3)
    } else { None }
}

#[cfg(test)]
mod regression_tests {
    use super::*;
    #[test]
    fn dense_spacing_matches_original_tables() {
        let classes = ["mord","mop","mbin","mrel","mopen","mclose","mpunct","minner","unknown"];
        for a in classes { for b in classes {
            let original = spacings.lock().unwrap();
            let expected = original.get(a).and_then(|row| row.get(b));
            assert_eq!(get_spacings(a,b).map(|m| (m.number,m.unit.as_str())), expected.map(|m| (m.number,m.unit.as_str())), "{a} {b}");
            let tight = tightSpacings.lock().unwrap();
            let expected = tight.get(a).and_then(|row| row.get(b));
            assert_eq!(get_tightSpacings(a,b).map(|m| (m.number,m.unit.as_str())), expected.map(|m| (m.number,m.unit.as_str())));
        }}
    }
}
