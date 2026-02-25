use crate::VirtualNode;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum MathNodeType {
    Math,
    Annotation,
    Mo,
    Mrow,
    Semantics,
    Mpadded,
    Mi,
    Mtext,
    Mn,
    Munder,
    Munderover,
    Mover,
    Mstyle,
    Mspace,
    Menclose,
    Mroot,
    Msqrt,
    Mglyph,
    Mtable,
    Mtr,
    Mtd,
    Mfrac,
    Mmultiscripts,
    Mphantom,
    Ms,
    Msub,
    Msubsup,
    Msup,
}

impl FromStr for MathNodeType {
    type Err = ();

    fn from_str(input: &str) -> Result<MathNodeType, Self::Err> {
        match input {
            "math" => Ok(MathNodeType::Math),
            "annotation" => Ok(MathNodeType::Annotation),
            "mo" => Ok(MathNodeType::Mo),
            "mrow" => Ok(MathNodeType::Mrow),
            "semantics" => Ok(MathNodeType::Semantics),
            "mpadded" => Ok(MathNodeType::Mpadded),
            "mi" => Ok(MathNodeType::Mi),
            "mtext" => Ok(MathNodeType::Mtext),
            "mn" => Ok(MathNodeType::Mn),
            "munder" => Ok(MathNodeType::Munder),
            "munderover" => Ok(MathNodeType::Munderover),
            "mover" => Ok(MathNodeType::Mover),
            "mstyle" => Ok(MathNodeType::Mstyle),
            "mspace" => Ok(MathNodeType::Mspace),
            "menclose" => Ok(MathNodeType::Menclose),
            "mroot" => Ok(MathNodeType::Mroot),
            "msqrt" => Ok(MathNodeType::Msqrt),
            "mglyph" => Ok(MathNodeType::Mglyph),
            "mtable" => Ok(MathNodeType::Mtable),
            "mtr" => Ok(MathNodeType::Mtr),
            "mtd" => Ok(MathNodeType::Mtd),
            "mfrac" => Ok(MathNodeType::Mfrac),
            "mmultiscripts" => Ok(MathNodeType::Mmultiscripts),
            "mphantom" => Ok(MathNodeType::Mphantom),
            "ms" => Ok(MathNodeType::Ms),
            "msub" => Ok(MathNodeType::Msub),
            "msubsup" => Ok(MathNodeType::Msubsup),
            "msup" => Ok(MathNodeType::Msup),
            _ => Err(()),
        }
    }
}

impl MathNodeType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            MathNodeType::Math => "math",
            MathNodeType::Annotation => "annotation",
            MathNodeType::Mo => "mo",
            MathNodeType::Mrow => "mrow",
            MathNodeType::Semantics => "semantics",
            MathNodeType::Mpadded => "mpadded",
            MathNodeType::Mi => "mi",
            MathNodeType::Mtext => "mtext",
            MathNodeType::Mn => "mn",
            MathNodeType::Munder => "munder",
            MathNodeType::Munderover => "munderover",
            MathNodeType::Mover => "mover",
            MathNodeType::Mstyle => "mstyle",
            MathNodeType::Mspace => "mspace",
            MathNodeType::Menclose => "menclose",
            MathNodeType::Mroot => "mroot",
            MathNodeType::Msqrt => "msqrt",
            MathNodeType::Mglyph => "mglyph",
            MathNodeType::Mtable => "mtable",
            MathNodeType::Mtr => "mtr",
            MathNodeType::Mtd => "mtd",
            MathNodeType::Mfrac => "mfrac",
            MathNodeType::Mmultiscripts => "mmultiscripts",
            MathNodeType::Mphantom => "mphantom",
            MathNodeType::Ms => "ms",
            MathNodeType::Msub => "msub",
            MathNodeType::Msubsup => "msubsup",
            MathNodeType::Msup => "msup",
        }
    }
}

// "semantics" |
// "mtext" | "mn" | "mo" | "mi" | "mspace" |
// "mover" | "munder" | "munderover" | "msup" | "msub" | "msubsup" |
// "mfrac" | "mroot" | "msqrt" |
// "mtable" | "mtr" | "mtd" | "mlabeledtr" |
// "mrow" | "menclose" |
// "mstyle" | "mpadded" | "mphantom" | "mglyph";

pub trait MathDomNodeClone {
    fn clone_math_dom_node(&self) -> Box<dyn MathDomNode>;
}

impl<T> MathDomNodeClone for T
where
    T: 'static + MathDomNode + Clone,
{
    fn clone_math_dom_node(&self) -> Box<dyn MathDomNode> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn MathDomNode> {
    fn clone(&self) -> Box<dyn MathDomNode> {
        self.clone_math_dom_node()
    }
}

pub trait MathDomNode: VirtualNode + MathDomNodeClone {
    fn to_text(&self) -> String;
}
