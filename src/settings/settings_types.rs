use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, Default)]
pub(crate) enum StrictType {
    Ignore,
    #[default]
    Warn,
    Error,
}

impl FromStr for StrictType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ignore" => Ok(StrictType::Ignore),
            "warn" => Ok(StrictType::Warn),
            "error" => Ok(StrictType::Error),
            _ => Err(()),
        }
    }
}

impl StrictType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StrictType::Ignore => "ignore",
            StrictType::Warn => "warn",
            StrictType::Error => "error",
        }
    }
}

/// Output type from KaTeX.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum OutputType {
    /// Outputs KaTeX in HTML only.
    #[default]
    Html,
    /// Outputs KaTeX in MathML only.
    Mathml,
    /// Outputs HTML for visual rendering and includes MathML for accessibility.
    HtmlAndMathml,
}

impl FromStr for OutputType {
    type Err = ();

    fn from_str(input: &str) -> Result<OutputType, Self::Err> {
        match input {
            "html" => Ok(OutputType::Html),
            "mathml" => Ok(OutputType::Mathml),
            "htmlAndMathml" => Ok(OutputType::HtmlAndMathml),
            _ => Err(()),
        }
    }
}
impl OutputType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            OutputType::Html => "html",
            OutputType::Mathml => "mathml",
            OutputType::HtmlAndMathml => "htmlAndMathml",
        }
    }
}


pub struct TrustContext{
    pub command :String,
    pub context :HashMap<String,String>
}