use crate::build::HTML::IsRealGroup;
use crate::build::{mathML, HTML, common};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::types::ParseNodeToAny;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, AnyParseNode, HtmlDomNode, types::ArgType};
use std::sync::Mutex;



/**
 * Converts verb group into body string.
 *
 * \verb* replaces each space with an open box \u2423
 * \verb replaces each space with a no-break space \xA0
 */
fn make_verb(group: &parse_node::types::verb) -> String {
    group.body.replace(' ', if group.star { "\u{2423}" } else { "U+00A0"})
}



pub fn handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
// \verb and \verb* are dealt with directly in Parser.js.
// If we end up here, it's because of a failure to match the two delimiters
// in the regex in Lexer.js.  LaTeX raises the following error when \verb is
// terminated by end of line (or file).
    panic!("\\verb ended by end of line instead of matching delimiter");
}

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::verb>()
        .unwrap();
    let text = make_verb(group);
    let mut body = vec![];
// \verb enters text mode and therefore is sized like \textstyle
    let new_options = options.having_style(&options.get_style().text());
    for c in text.chars(){

        body.push(Box::new(common::make_symbol(if c == '~' {
             "\\textasciitilde".to_string()
        } else{
            c.to_string()
        }, "Typewriter-Regular".to_string(),
                                      group.mode, Some(&new_options), vec!["mord".to_string(), "texttt".to_string()])) as Box<dyn HtmlDomNode>);
    }
    let res = common::make_span(
        [vec!["mord".to_string(), "text".to_string()], new_options.sizing_classes(&options)].concat(),
        common::try_combine_chars(body),
        Some(&new_options),
        Default::default()
    );
    return Box::new(res) as Box<dyn HtmlDomNode>;
}


fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::verb>()
        .unwrap();
    panic!("undeifne");
// let text = new mathMLTree.TextNode(makeVerb(group));
// let node = MathNode::new("mtext".to_string(), [text]);
// node.set_attribute("mathvariant".to_string(), "monospace".to_string());
// return node;
}

lazy_static! {
    pub static ref VERB: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(0);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "verb".to_string(),
            names: vec!["\\verb".to_string()],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}


