use std::str::FromStr;

// In these types, "" (empty string) means "no change".
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
    Textbf,
    Textmd,
    NoChange,
}

impl FromStr for FontWeight {
    type Err = ();

    fn from_str(input: &str) -> Result<FontWeight, Self::Err> {
        match input {
            "textbf" => Ok(FontWeight::Textbf),
            "textmd" => Ok(FontWeight::Textmd),
            "" => Ok(FontWeight::NoChange),
            _ => Err(()),
        }
    }
}

impl FontWeight {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            FontWeight::Textbf => "textbf",
            FontWeight::Textmd => "textmd",
            FontWeight::NoChange => ""
        }
    }
}

#[derive(Debug, Clone, Copy,PartialEq)]
pub enum FontShape {
    Textit,
    Textup,
    NoChange,
}


impl FromStr for FontShape {
    type Err = ();

    fn from_str(input: &str) -> Result<FontShape, Self::Err> {
        match input {
            "textit" => Ok(FontShape::Textit),
            "textup" => Ok(FontShape::Textup),
            "" => Ok(FontShape::NoChange),
            _ => Err(()),
        }
    }
}

impl FontShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            FontShape::Textit => "textit",
            FontShape::Textup => "textup",
            FontShape::NoChange => ""
        }
    }
}
