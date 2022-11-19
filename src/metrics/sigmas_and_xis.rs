use std::collections::HashMap;
use wasm_bindgen::prelude::*;


// const sigmasAndXis = {
//     slant: [0.250, 0.250, 0.250],       // sigma1
//     space: [0.000, 0.000, 0.000],       // sigma2
//     stretch: [0.000, 0.000, 0.000],     // sigma3
//     shrink: [0.000, 0.000, 0.000],      // sigma4
//     xHeight: [0.431, 0.431, 0.431],     // sigma5
//     quad: [1.000, 1.171, 1.472],        // sigma6
//     extraSpace: [0.000, 0.000, 0.000],  // sigma7
//     num1: [0.677, 0.732, 0.925],        // sigma8
//     num2: [0.394, 0.384, 0.387],        // sigma9
//     num3: [0.444, 0.471, 0.504],        // sigma10
//     denom1: [0.686, 0.752, 1.025],      // sigma11
//     denom2: [0.345, 0.344, 0.532],      // sigma12
//     sup1: [0.413, 0.503, 0.504],        // sigma13
//     sup2: [0.363, 0.431, 0.404],        // sigma14
//     sup3: [0.289, 0.286, 0.294],        // sigma15
//     sub1: [0.150, 0.143, 0.200],        // sigma16
//     sub2: [0.247, 0.286, 0.400],        // sigma17
//     supDrop: [0.386, 0.353, 0.494],     // sigma18
//     subDrop: [0.050, 0.071, 0.100],     // sigma19
//     delim1: [2.390, 1.700, 1.980],      // sigma20
//     delim2: [1.010, 1.157, 1.420],      // sigma21
//     axisHeight: [0.250, 0.250, 0.250],  // sigma22
//
//     // These font metrics are extracted from TeX by using tftopl on cmex10.tfm;
//     // they correspond to the font parameters of the extension fonts (family 3).
//     // See the TeXbook, page 441. In AMSTeX, the extension fonts scale; to
//     // match cmex7, we'd use cmex7.tfm values for script and scriptscript
//     // values.
//     defaultRuleThickness: [0.04, 0.049, 0.049], // xi8; cmex7: 0.049
//     bigOpSpacing1: [0.111, 0.111, 0.111],       // xi9
//     bigOpSpacing2: [0.166, 0.166, 0.166],       // xi10
//     bigOpSpacing3: [0.2, 0.2, 0.2],             // xi11
//     bigOpSpacing4: [0.6, 0.611, 0.611],         // xi12; cmex7: 0.611
//     bigOpSpacing5: [0.1, 0.143, 0.143],         // xi13; cmex7: 0.143
//
//     // The \sqrt rule width is taken from the height of the surd character.
//     // Since we use the same font at all sizes, this thickness doesn't scale.
//     sqrtRuleThickness: [0.04, 0.04, 0.04],
//
//     // This value determines how large a pt is, for metrics which are defined
//     // in terms of pts.
//     // This value is also used in katex.less; if you change it make sure the
//     // values match.
//     ptPerEm: [10.0, 10.0, 10.0],
//
//     // The space between adjacent `|` columns in an array definition. From
//     // `\showthe\doublerulesep` in LaTeX. Equals 2.0 / ptPerEm.
//     doubleRuleSep: [0.2, 0.2, 0.2],
//
//     // The width of separator lines in {array} environments. From
//     // `\showthe\arrayrulewidth` in LaTeX. Equals 0.4 / ptPerEm.
//     arrayRuleWidth: [0.04, 0.04, 0.04],
//
//     // Two values from LaTeX source2e:
//     fboxsep: [0.3, 0.3, 0.3], //        3 pt / ptPerEm
//     fboxrule: [0.04, 0.04, 0.04], // 0.4 pt / ptPerEm
// };

#[derive(Debug,Clone, PartialEq)]
#[wasm_bindgen]
pub struct FontMetrics {
    pub slant: f64,
    pub space: f64,
    pub stretch: f64,
    pub shrink: f64,
    pub xHeight: f64,
    pub quad: f64,
    pub cssEmPerMu: f64,
    pub extraSpace: f64,
    pub num1: f64,
    pub num2: f64,
    pub num3: f64,
    pub denom1: f64,
    pub denom2: f64,
    pub sup1: f64,
    pub sup2: f64,
    pub sup3: f64,
    pub sub1: f64,
    pub sub2: f64,
    pub supDrop: f64,
    pub subDrop: f64,
    pub delim1: f64,
    pub delim2: f64,
    pub axisHeight: f64,
    pub defaultRuleThickness: f64,
    pub bigOpSpacing1: f64,
    pub bigOpSpacing2: f64,
    pub bigOpSpacing3: f64,
    pub bigOpSpacing4: f64,
    pub bigOpSpacing5: f64,
    pub sqrtRuleThickness: f64,
    pub ptPerEm: f64,
    pub doubleRuleSep: f64,
    pub arrayRuleWidth: f64,
    pub fboxsep: f64,
    pub fboxrule: f64,
}

pub const sigmasAndXis: [FontMetrics; 3] = [
    FontMetrics {
        slant: 0.250,
        space: 0.000,
        stretch: 0.000,
        shrink: 0.000,
        xHeight: 0.431,
        quad: 1.000,
        cssEmPerMu: 1.000 / 18.0,
        extraSpace: 0.000,
        num1: 0.677,
        num2: 0.394,
        num3: 0.444,
        denom1: 0.686,
        denom2: 0.345,
        sup1: 0.413,
        sup2: 0.363,
        sup3: 0.289,
        sub1: 0.150,
        sub2: 0.247,
        supDrop: 0.386,
        subDrop: 0.050,
        delim1: 2.390,
        delim2: 1.010,
        axisHeight: 0.250,
        defaultRuleThickness: 0.040,
        bigOpSpacing1: 0.111,
        bigOpSpacing2: 0.166,
        bigOpSpacing3: 0.200,
        bigOpSpacing4: 0.600,
        bigOpSpacing5: 0.100,
        sqrtRuleThickness: 0.040,
        ptPerEm: 10.0,
        doubleRuleSep: 0.20,
        arrayRuleWidth: 0.040,
        fboxsep: 0.300,
        fboxrule: 0.040,
    },
    FontMetrics {
        slant: 0.250,
        space: 0.000,
        stretch: 0.000,
        shrink: 0.000,
        xHeight: 0.431,
        quad: 1.171,
        cssEmPerMu: 1.171 / 18.0,
        extraSpace: 0.000,
        num1: 0.732,
        num2: 0.384,
        num3: 0.471,
        denom1: 0.752,
        denom2: 0.344,
        sup1: 0.503,
        sup2: 0.431,
        sup3: 0.286,
        sub1: 0.143,
        sub2: 0.286,
        supDrop: 0.353,
        subDrop: 0.071,
        delim1: 1.700,
        delim2: 1.157,
        axisHeight: 0.250,
        defaultRuleThickness: 0.049,
        bigOpSpacing1: 0.111,
        bigOpSpacing2: 0.166,
        bigOpSpacing3: 0.200,
        bigOpSpacing4: 0.611,
        bigOpSpacing5: 0.143,
        sqrtRuleThickness: 0.040,
        ptPerEm: 10.0,
        doubleRuleSep: 0.200,
        arrayRuleWidth: 0.040,
        fboxsep: 0.300,
        fboxrule: 0.040,
    },
    FontMetrics {
        slant: 0.250,
        space: 0.000,
        stretch: 0.000,
        shrink: 0.000,
        xHeight: 0.431,
        quad: 1.472,
        cssEmPerMu: 1.472 / 18.0,
        extraSpace: 0.000,
        num1: 0.925,
        num2: 0.387,
        num3: 0.504,
        denom1: 1.025,
        denom2: 0.532,
        sup1: 0.504,
        sup2: 0.404,
        sup3: 0.294,
        sub1: 0.200,
        sub2: 0.400,
        supDrop: 0.494,
        subDrop: 0.100,
        delim1: 1.98,
        delim2: 1.42,
        axisHeight: 0.25,
        defaultRuleThickness: 0.049,
        bigOpSpacing1: 0.111,
        bigOpSpacing2: 0.166,
        bigOpSpacing3: 0.200,
        bigOpSpacing4: 0.611,
        bigOpSpacing5: 0.143,
        sqrtRuleThickness: 0.040,
        ptPerEm: 10.00,
        doubleRuleSep: 0.200,
        arrayRuleWidth: 0.040,
        fboxsep: 0.300,
        fboxrule: 0.040,
    }
];
