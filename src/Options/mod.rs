mod consts;
/**
 * This file contains information about the options that the Parser carries
 * around with it while parsing. Data is held in an `Options` object, and when
 * recursing, a new `Options` object can be created with the `.with*` and
 * `.reset` functions.
 */
mod types;

use serde::{Deserialize, Serialize};

use crate::get_global_metrics;
use crate::metrics::sigmas_and_xis::FontMetrics;
use crate::settings::Settings;
use crate::utils::{console_log, log};
use crate::Style::StyleInterface;
use consts::*;
use std::str::FromStr;
use types::{FontShape, FontWeight};
use wasm_bindgen::prelude::*;

pub fn size_at_style(size: f64, style: &StyleInterface) -> f64 {
    if style.size < 2 {
        size
    } else {
        SIZE_STYLE_MAP[size as i32 as usize - 1 as usize][style.size as usize - 1] as f64
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    style: StyleInterface,
    pub color: String,
    pub size: f64,
    pub textSize: f64,
    pub phantom: bool,
    // A font family applies to a group of fonts (i.e. SansSerif), while a font
    // represents a specific font (i.e. SansSerif Bold).
    // See: https://tex.stackexchange.com/questions/22350/difference-between-textrm-and-mathrm
    pub font: String,
    pub fontFamily: String,
    font_weight: FontWeight,
    font_shape: FontShape,
    pub sizeMultiplier: f64,
    pub maxSize: f64,
    pub minRuleThickness: f64,
    _fontMetrics: Option<&'static FontMetrics>,
}

/**
 * This is the main options class. It contains the current style, size, color,
 * and font.
 *
 * Options objects should not be modified. To create a new Options with
 * different properties, call a `.having*` method.
 */
static BASESIZE: f64 = 6.0;

impl Options {
    pub fn from_settings(settings: &Settings) -> Options {
        let mut res = Options::new();
        res.style = if settings.get_display_mode() {
            let r = crate::Style::DISPLAY.lock().unwrap();
            r.clone()
        } else {
            let r = crate::Style::TEXT.lock().unwrap();
            r.clone()
        };
        // console.log(`maxSiz === ${settings.maxSize}`)
        res.maxSize = settings.get_max_size().unwrap_or(100000.0);
        res.minRuleThickness = settings.get_min_rule_thickness();
        res.log();
        return res;
    }

    // Takes font options, and returns the appropriate fontLookup name
    pub(crate) fn retrieve_text_font_name(&self, font_family: String) -> String {
        let base_font_name = match font_family.as_str() {
            "amsrm" => "AMS",
            "textrm" => "Main",
            "textsf" => "SansSerif",
            "texttt" => "Typewriter",
            _ => &font_family, // use fonts added by a plugin
        };

        let font_styles_name = if self.font_weight == FontWeight::Textbf
            && self.font_shape == FontShape::Textit
        {
            "BoldItalic"
        } else if (self.font_weight == FontWeight::Textbf) {
            "Bold"
        } else if (self.font_weight == FontWeight::Textbf) {
            "Italic"
        } else {
            "Regular"
        };

        return format!("{base_font_name}-{font_styles_name}");
    }
}

impl Options {
    pub fn fontWeight(&self) -> String {
        self.font_weight.as_str().to_string()
    }

    pub fn fontShape(&self) -> String {
        self.font_shape.as_str().to_string()
    }

    pub fn get_style(&self) -> StyleInterface {
        self.style.clone()
    }

    pub fn set_style(&mut self, style: &StyleInterface) {
        self.style = style.clone()
    }

    /**
     * The base size index.
     */
    pub fn new() -> Options {
        Options {
            style: StyleInterface {
                id: 0,
                size: 0,
                cramped: false,
            },
            color: "".to_string(),
            size: BASESIZE,
            textSize: BASESIZE,
            phantom: false,
            font: String::from(""),
            fontFamily: String::from(""),
            font_weight: FontWeight::NoChange,
            font_shape: FontShape::NoChange,
            sizeMultiplier: SIZE_MULTIPLIERS[BASESIZE as i32 as usize - 1],
            maxSize: 0.0,
            minRuleThickness: 0.0,
            _fontMetrics: None,
        }
    }

    /**
     * Return an options object with the given style. If `this.style === style`,
     * returns `this`.
     */
    pub fn havingStyle(&self, style: &StyleInterface) -> Options {
        if &self.style == style {
            // let res =  Options{
            //     style:self.style.clone(),
            //     ..self.clone()
            // };
            // console_log!("{:#?}",res);
            return self.clone();
        } else {
            let res = Options {
                style: style.clone(),
                size: size_at_style(self.textSize, style),
                ..self.clone()
            };
            return res;
        }
    }

    /**
     * Return an options object with a cramped version of the current style. If
     * the current style is cramped, returns `this`.
     */
    pub fn havingCrampedStyle(&self) -> Options {
        return self.havingStyle(&self.style.cramp());
    }

    /**
     * Return an options object with the given size and in at least `\textstyle`.
     * Returns `this` if appropriate.
     */
    pub fn havingSize(&self, size: f64) -> Options {
        if self.size == size && self.textSize == size {
            return self.clone();
        } else {
            return Options {
                style: self.style.text(),
                size: size,
                textSize: size,
                sizeMultiplier: SIZE_MULTIPLIERS[size as i32 as usize - 1],
                ..self.clone()
            };
        }
    }

    /**
     * Like `this.havingSize(BASESIZE).havingStyle(style)`. If `style` is omitted,
     * changes to at least `\textstyle`.
     */
    pub fn havingBaseStyle(&self, style: &StyleInterface) -> Options {
        //style = style | | this.style.text();TODO
        let wantSize: f64 = size_at_style(BASESIZE, style);
        if self.size == wantSize && self.textSize == BASESIZE && &self.style == style {
            return self.clone();
        } else {
            return Options {
                style: style.clone(),
                size: wantSize,
                ..self.clone()
            };
        }
    }

    /**
     * Remove the effect of sizing changes such as \Huge.
     * Keep the effect of the current style, such as \scriptstyle.
     */
    pub fn havingBaseSizing(&self) -> Options {
        let size = match self.style.id {
            4 | 5 => 3,
            6 | 7 => 1, // normalsize in scriptscriptstyle
            _ => 6,     // normalsize in textstyle or displaystyle
        };
        return Options {
            style: self.style.text(),
            size: size as f64,
            ..self.clone()
        };
    }

    /**
     * Create a new options object with the given color.
     */
    pub fn withColor(&self, color: String) -> Options {
        return Options {
            color: color,
            ..self.clone()
        };
    }

    /**
     * Create a new options object with "phantom" set to true.
     */
    pub fn withPhantom(&self) -> Options {
        return Options {
            phantom: true,
            ..self.clone()
        };
    }

    /**
     * Creates a new options object with the given math font or old text font.
     * @type {[type]}
     */
    pub fn withFont(&self, font: String) -> Options {
        return Options {
            font: font,
            ..self.clone()
        };
    }

    /**
     * Create a new options objects with the given fontFamily.
     */
    pub fn withTextFontFamily(&self, fontFamily: String) -> Options {
        return Options {
            fontFamily: fontFamily,
            font: "".to_string(),
            ..self.clone()
        };
    }

    /**
     * Creates a new options object with the given font weight
     */
    pub fn withTextFontWeight(&self, fontWeight: String) -> Options {
        return Options {
            font_weight: FontWeight::from_str(fontWeight.as_str()).unwrap(),
            font: "".to_string(),
            ..self.clone()
        };
    }

    /**
     * Creates a new options object with the given font weight
     */
    pub fn withTextFontShape(&self, fontShape: Option<String>) -> Options {
        return Options {
            font_shape: match fontShape {
                Some(p) => FontShape::from_str(p.as_str()).unwrap(),
                None => self.font_shape,
            },
            font: "".to_string(),
            ..self.clone()
        };
    }

    /**
     * Return the CSS sizing classes required to switch from enclosing options
     * `oldOptions` to `this`. Returns an array of classes.
     */
    pub fn sizingClasses(&self, oldOptions: &Options) -> Vec<String> {
        if oldOptions.size != self.size {
            return vec![
                "sizing".to_string(),
                format!("reset-size{}", oldOptions.size),
                format!("size{}", self.size),
            ];
        } else {
            return vec![];
        }
    }

    /**
     * Return the CSS sizing classes required to switch to the base size. Like
     * `this.havingSize(BASESIZE).sizingClasses(this)`.
     */
    pub fn baseSizingClasses(&self) -> JsValue {
        if self.size != BASESIZE {
            return JsValue::from_serde(&vec![
                "sizing".to_string(),
                format!("reset-size{}", self.size),
                format!("size{}", BASESIZE),
            ])
            .unwrap();
        } else {
            return JsValue::from_serde(&(vec![] as Vec<String>)).unwrap();
        }
    }

    // /**
    //  * TODO
    //  * Return the font metrics for this size.
    //  */
    pub fn fontMetrics(&self) -> FontMetrics {
        if self._fontMetrics.is_none() {
            return get_global_metrics(self.size).clone();
        }
        return self._fontMetrics.unwrap().clone();
    }

    /**
     * Gets the CSS color of the current options object
     */
    pub fn getColor(&self) -> String {
        if self.phantom == true {
            return String::from("transparent");
        } else {
            return self.color.clone();
        }
    }

    pub fn log(&self) {
        if false {
            console_log!("rust: self = {:#?}", self);
        } else {
            println!("Options = {:#?}", self);
        }
    }
}
