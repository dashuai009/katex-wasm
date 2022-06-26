use crate::VirtualNode;
use std::str::FromStr;

pub enum MathNodeType {
    Math,
    Annotation,
}

impl FromStr for MathNodeType {
    type Err = ();

    fn from_str(input: &str) -> Result<MathNodeType, Self::Err> {
        match input {
            "math" => Ok(MathNodeType::Math),
            "annotation" => Ok(MathNodeType::Annotation),
            _ => Err(()),
        }
    }
}

impl MathNodeType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            MathNodeType::Math => "math",
            MathNodeType::Annotation => "annotation",
        }
    }
}

// "math" | "annotation" | "semantics" |
// "mtext" | "mn" | "mo" | "mi" | "mspace" |
// "mover" | "munder" | "munderover" | "msup" | "msub" | "msubsup" |
// "mfrac" | "mroot" | "msqrt" |
// "mtable" | "mtr" | "mtd" | "mlabeledtr" |
// "mrow" | "menclose" |
// "mstyle" | "mpadded" | "mphantom" | "mglyph";
pub trait ToText {
    fn to_text(&self) -> String;
}
pub trait MathDomNode: ToText + VirtualNode {}
