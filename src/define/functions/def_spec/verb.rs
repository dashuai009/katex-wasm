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
    group.body
        .replace(' ', if group.star { "\u{2423}" } else { "\u{00A0}" })
}



pub fn handler_fn(
    context: FunctionContext,
    _args: Vec<Box<dyn AnyParseNode>>,
    _opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut ctx = context.borrow_mut();
    let mut star = false;
    let mut delim_token = ctx.parser.fetch();
    if delim_token.text == "*" {
        star = true;
        ctx.parser.consume();
        delim_token = ctx.parser.fetch();
    }

    if delim_token.text == "EOF" {
        ctx.parser.report_parse_error(
            "\\verb ended by end of line instead of matching delimiter".to_string(),
            delim_token.loc.clone(),
        );
        return Box::new(parse_node::types::verb {
            mode: ctx.parser.mode,
            loc: None,
            body: String::new(),
            star,
        }) as Box<dyn AnyParseNode>;
    }

    let delimiter = delim_token.text.clone();
    ctx.parser.consume(); // consume opening delimiter
    let mut body = String::new();
    loop {
        let tok = ctx.parser.fetch();
        if tok.text == "EOF" || tok.text.contains('\n') {
            ctx.parser.report_parse_error(
                "\\verb ended by end of line instead of matching delimiter".to_string(),
                tok.loc.clone(),
            );
            break;
        }
        if tok.text == delimiter {
            ctx.parser.consume(); // consume closing delimiter
            return Box::new(parse_node::types::verb {
                mode: ctx.parser.mode,
                loc: None,
                body,
                star,
            }) as Box<dyn AnyParseNode>;
        }
        if tok.text == " " {
            if let Some(loc) = &tok.loc {
                let count = (loc.end - loc.start).max(1) as usize;
                body.push_str(&" ".repeat(count));
            } else {
                body.push(' ');
            }
        } else {
            body.push_str(&tok.text);
        }
        ctx.parser.consume();
    }

    Box::new(parse_node::types::verb {
        mode: ctx.parser.mode,
        loc: None,
        body,
        star,
    }) as Box<dyn AnyParseNode>
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


