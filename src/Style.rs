use wasm_bindgen::prelude::*;

#[derive(Debug,Clone)]
#[derive(PartialEq)]
#[wasm_bindgen]
pub struct StyleInterface {
    pub id: usize,
    pub size: i32,
    pub cramped: bool,
}


#[wasm_bindgen]
impl StyleInterface {


    #[wasm_bindgen(constructor)]
    pub fn new(id: usize, size: i32, cramped: bool) -> StyleInterface {
        StyleInterface {
            id: id as usize,
            size: size as i32,
            cramped:cramped,
        }
    }

    /**
     * Get the style of a superscript given a base in the current style.
     */
    pub fn sup(&self) -> StyleInterface {
        return styles[sup[self.id]].clone();
    }

    /**
     * Get the style of a subscript given a base in the current style.
     */
    pub fn sub(&self) -> StyleInterface {
        return styles[sub[self.id]].clone();
    }

    /**
     * Get the style of a fraction numerator given the fraction in the current
     * style.
     */
    pub fn fracNum(&self) -> StyleInterface {
        return styles[fracNum[self.id]].clone();
    }

    /**
     * Get the style of a fraction denominator given the fraction in the current
     * style.
     */
    pub fn fracDen(&self) -> StyleInterface {
        return styles[fracDen[self.id]].clone();
    }

    /**
     * Get the cramped version of a style (in particular, cramping a cramped style
     * doesn't change the style).
     */
    pub fn cramp(&self) -> StyleInterface {
        return styles[cramp[self.id]].clone();
    }

    /**
     * Get a text or display version of self.style.
     */
    pub fn text(&self) -> StyleInterface {
        return styles[text[self.id]].clone();
    }

    /**
     * Return true if self.style is tightly spaced (scriptstyle/scriptscriptstyle)
     */
    pub fn isTight(&self) -> bool {
        return self.size >= 2;
    }
}

// IDs of the different styles
const D: usize = 0;
const Dc: usize = 1;
const T: usize = 2;
const Tc: usize = 3;
const S: usize = 4;
const Sc: usize = 5;
const SS: usize = 6;
const SSc: usize = 7;

// Instances of the different styles
const styles: [StyleInterface; 8] = [
    StyleInterface {
        id: D,
        size: 0,
        cramped: false,
    },
    StyleInterface {
        id: Dc,
        size: 0,
        cramped: true,
    },
    StyleInterface {
        id: T,
        size: 1,
        cramped: false,
    },
    StyleInterface {
        id: Tc,
        size: 1,
        cramped: true,
    },
    StyleInterface {
        id: S,
        size: 2,
        cramped: false,
    },
    StyleInterface {
        id: Sc,
        size: 2,
        cramped: true,
    },
    StyleInterface {
        id: SS,
        size: 3,
        cramped: false,
    },
    StyleInterface {
        id: SSc,
        size: 3,
        cramped: true,
    },
];

const sup: [usize; 8] = [S, Sc, S, Sc, SS, SSc, SS, SSc];
const sub: [usize; 8] = [Sc, Sc, Sc, Sc, SSc, SSc, SSc, SSc];
const fracNum: [usize; 8] = [T, Tc, S, Sc, SS, SSc, SS, SSc];
const fracDen: [usize; 8] = [Tc, Tc, Sc, Sc, SSc, SSc, SSc, SSc];
const cramp: [usize; 8] = [Dc, Dc, Tc, Tc, Sc, Sc, SSc, SSc];
const text: [usize; 8] = [D, Dc, T, Tc, T, Tc, T, Tc];


use std::sync::RwLock;
lazy_static!{
    pub static ref DISPLAY:RwLock<StyleInterface> = RwLock::new(StyleInterface::new(0, 0, false));
    pub static ref TEXT:RwLock<StyleInterface> = RwLock::new(StyleInterface::new(2, 1, false));
    pub static ref SCRIPT:RwLock<StyleInterface> = RwLock::new(StyleInterface::new(4, 2, false));
    pub static ref SCRIPTSCRIPT:RwLock<StyleInterface> = RwLock::new(StyleInterface::new(6, 3, false));
}
