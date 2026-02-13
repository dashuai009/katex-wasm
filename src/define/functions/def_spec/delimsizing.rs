use crate::build::HTML::{DomType, IsRealGroup};
use crate::build::{common, mathML, HTML};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use crate::define::macros::public::MacroDefinition;
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::mathML_tree::math_node::MathNode;
use crate::mathML_tree::public::{MathDomNode, MathNodeType};
use crate::parse_node::check_symbol_node_type;
use crate::parse_node::types::ParseNodeToAny;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::ops::Deref;
use std::sync::Mutex;
// @flow

// Extra data needed for the delimiter handler down below
struct DelimSize {
    mclass: String,
    size: i32,
}

lazy_static! {
    static ref DELIMITER_SIZES: std::collections::HashMap<&'static str, DelimSize> = {
        let res = std::collections::HashMap::from([
            (
                "\\bigl",
                DelimSize {
                    mclass: "mopen".to_string(),
                    size: 1,
                },
            ),
            (
                "\\Bigl",
                DelimSize {
                    mclass: "(.*)".to_string(),
                    size: 2,
                },
            ),
            (
                "\\biggl",
                DelimSize {
                    mclass: "mopen".to_string(),
                    size: 3,
                },
            ),
            (
                "\\Biggl",
                DelimSize {
                    mclass: "mopen".to_string(),
                    size: 4,
                },
            ),
            (
                "\\bigr",
                DelimSize {
                    mclass: "(.*)".to_string(),
                    size: 1,
                },
            ),
            (
                "\\Bigr",
                DelimSize {
                    mclass: "(.*)".to_string(),
                    size: 2,
                },
            ),
            (
                "\\biggr",
                DelimSize {
                    mclass: "mclose".to_string(),
                    size: 3,
                },
            ),
            (
                "\\Biggr",
                DelimSize {
                    mclass: "mclose".to_string(),
                    size: 4,
                },
            ),
            (
                "\\bigm",
                DelimSize {
                    mclass: "(.*)".to_string(),
                    size: 1,
                },
            ),
            (
                "\\Bigm",
                DelimSize {
                    mclass: "(.*)".to_string(),
                    size: 2,
                },
            ),
            (
                "\\biggm",
                DelimSize {
                    mclass: "mrel".to_string(),
                    size: 3,
                },
            ),
            (
                "\\Biggm",
                DelimSize {
                    mclass: "mrel".to_string(),
                    size: 4,
                },
            ),
            (
                "\\big",
                DelimSize {
                    mclass: "(.*)".to_string(),
                    size: 1,
                },
            ),
            (
                "\\Big",
                DelimSize {
                    mclass: "(.*)".to_string(),
                    size: 2,
                },
            ),
            (
                "\\bigg",
                DelimSize {
                    mclass: "(.*)".to_string(),
                    size: 3,
                },
            ),
            (
                "\\Bigg",
                DelimSize {
                    mclass: "(.*)".to_string(),
                    size: 4,
                },
            ),
        ]);
        res
    };
    static ref DELIMITERS: Vec<String> = {
        vec![
            "(".to_string(),
            "\\lparen".to_string(),
            ")".to_string(),
            "\\rparen".to_string(),
            "[".to_string(),
            "\\lbrack".to_string(),
            "]".to_string(),
            "\\rbrack".to_string(),
            "\\{".to_string(),
            "\\lbrace".to_string(),
            "\\}".to_string(),
            "\\rbrace".to_string(),
            "\\lfloor".to_string(),
            "\\rfloor".to_string(),
            "\u{230a}".to_string(),
            "\u{230b}".to_string(),
            "\\lceil".to_string(),
            "\\rceil".to_string(),
            "\u{2308}".to_string(),
            "\u{2309}".to_string(),
            "<".to_string(),
            ">".to_string(),
            "\\langle".to_string(),
            "\u{27e8}".to_string(),
            "\\rangle".to_string(),
            "\u{27e9}".to_string(),
            "\\lt".to_string(),
            "\\gt".to_string(),
            "\\lvert".to_string(),
            "\\rvert".to_string(),
            "\\lVert".to_string(),
            "\\rVert".to_string(),
            "\\lgroup".to_string(),
            "\\rgroup".to_string(),
            "\u{27ee}".to_string(),
            "\u{27ef}".to_string(),
            "\\lmoustache".to_string(),
            "\\rmoustache".to_string(),
            "\u{23b0}".to_string(),
            "\u{23b1}".to_string(),
            "/".to_string(),
            "\\backslash".to_string(),
            "|".to_string(),
            "\\vert".to_string(),
            "\\|".to_string(),
            "\\Vert".to_string(),
            "\\uparrow".to_string(),
            "\\Uparrow".to_string(),
            "\\downarrow".to_string(),
            "\\Downarrow".to_string(),
            "\\updownarrow".to_string(),
            "\\Updownarrow".to_string(),
            ".".to_string(),
        ]
    };
}

#[derive(Clone,Debug)]
struct IsMiddle {
    delim: String,
    options: Options,
}

//////////////// special IsMiddleSpan

use crate::dom_tree::line_node::LineNode;
use crate::dom_tree::utils::{this_init_node, this_to_markup, this_to_node};
use crate::units::make_em;
use crate::utils::escape;
use crate::{path_get, scriptFromCodepoint,  VirtualNode};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use struct_format::html_dom_node;

#[derive( Clone, Debug)]
struct IsMiddleSpan {
    span:Span,
    pub is_middle:IsMiddle
}

impl IsMiddleSpan {
    pub fn new(span:Span,is_middle:IsMiddle)->IsMiddleSpan{
        IsMiddleSpan{
            span,
            is_middle
        }
    }
    pub fn set_attribute(&mut self, attribute: String, value: String) {
        self.span.set_attribute(attribute, value);
    }
}

impl VirtualNode for IsMiddleSpan {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn to_node(&self) -> web_sys::Node {
        panic!("undefined")
        // this_to_node!(self.span, "span")
    }

    fn to_markup(&self) -> String {
        panic!("undefined")
        // this_to_markup!(self, "span")
    }
}

impl HtmlDomNode for IsMiddleSpan{
    fn get_classes(&self) -> &Vec<String> {
        return &self.span.get_classes();
    }
    fn get_mut_classes(&mut self) -> &mut Vec<String> {
        return self.span.get_mut_classes();
    }


    fn set_classes(&mut self, _classes: Vec<String>) {
        self.span.set_classes( _classes);
    }
    fn get_height(&self) -> f64 {
        return self.span.get_height();
    }
    fn set_height(&mut self, _height: f64) {
        self.span.set_height( _height);
    }

    fn get_depth(&self) -> f64 {
        return self.span.get_depth();
    }
    fn set_depth(&mut self, _depth: f64) {
        self.span.set_depth(_depth);
    }

    fn get_max_font_size(&self) -> f64 {
        return self.span.get_max_font_size();
    }

    fn set_max_font_size(&mut self, _max_font_size: f64) {
        self.span.set_max_font_size(_max_font_size);
    }

    fn get_style(&self) -> &CssStyle {
        return self.span.get_style();
    }
    fn get_mut_style(&mut self) -> &mut CssStyle {
        return self.get_mut_style();
    }

    fn set_style(&mut self, _style: CssStyle) {
        self.span.set_style(_style);
    }
    fn has_class(&self, class_name: &String) -> bool {
        return self.span.has_class(class_name);
    }
    fn get_children(&self) -> Option<&Vec<Box<dyn HtmlDomNode>>>{
        return self.span.get_children();
    }

    fn get_mut_children(&mut self) -> Option<&mut Vec<Box<dyn HtmlDomNode>>>{
        return self.get_mut_children();
    }
}

//////////////// special IsMiddleSpan




// Delimiter functions
fn check_delimiter(delim: &Box<dyn AnyParseNode>, func_name: &String) -> String {
    // let symDelim = check_symbol_node_type(delim);
    //["accent-token", "mathord", "op-token", "spacing", "textord"]; || "atom"
    let t = if let Some(at) = delim
        .as_any()
        .downcast_ref::<parse_node::types::accent_token>()
    {
        &at.text
    } else if let Some(mo) = delim.as_any().downcast_ref::<parse_node::types::mathord>() {
        &mo.text
    } else if let Some(ot) = delim.as_any().downcast_ref::<parse_node::types::op_token>() {
        &ot.text
    } else if let Some(s) = delim.as_any().downcast_ref::<parse_node::types::spacing>() {
        &s.text
    } else if let Some(t) = delim.as_any().downcast_ref::<parse_node::types::textord>() {
        &t.text
    } else if let Some(a) = delim.as_any().downcast_ref::<parse_node::types::atom>() {
        &a.text
    } else {
        panic!("Invalid delimiter type '{}' {:#?}", delim.get_type(), delim);
    };
    if DELIMITERS.contains(t) {
        return t.clone();
    } else {
        panic!("Invalid delimiter '{}' after '{}', {:#?}", t, func_name, delim);
    }
}

pub fn big_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let context = ctx.borrow();
    let delim_text = check_delimiter(&args[0], &context.func_name);

    let res = parse_node::types::delimsizing {
        mode: context.parser.mode,
        loc: None,
        size: DELIMITER_SIZES.get(&context.func_name.as_str()).unwrap().size as usize,
        mclass: DELIMITER_SIZES.get(&context.func_name.as_str()).unwrap().mclass.clone(),
        delim: delim_text,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

pub fn big_html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::delimsizing>()
        .unwrap();
    if group.delim == ".".to_string() {
        // Empty delimiters still count as elements, even though they don't
        // show anything.
        let res = common::make_span(vec![group.mclass.clone()], vec![], None, Default::default());
        return Box::new(res) as Box<dyn HtmlDomNode>;
    }

    // Use delimiter.sizedDelim to generate the delimiter.
    let res = crate::delimiter::make_sized_delim(
        &group.delim,
        group.size,
        &options,
        group.mode,
        vec![group.mclass.clone()],
    );
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn big_mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined");
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::delimsizing>()
        .unwrap();
    // let children = [];
    //
    // if (group.delim != ".".to_string()) {
    // children.push(mml.makeText(group.delim, group.mode));
    // }
    //
    // let mut node = MathNode::new("mo".to_string(), children);
    //
    // if (group.mclass == "mopen" ||
    // group.mclass == "mclose".to_string()) {
    // // Only some of the delimsizing functions act as fences, and they
    // // return "mopen" or "mclose" mclass.
    // node.set_attribute("fence".to_string(), "true".to_string());
    // } else {
    // // Explicitly disable fencing if it's not a fence, to override the
    // // defaults.
    // node.set_attribute("fence".to_string(), "false".to_string());
    // }
    //
    // node.set_attribute("stretchy".to_string(), "true".to_string());
    // let size = crate::units::make_em(delimiter.sizeToMaxHeight[group.size]);
    // node.set_attribute("minsize".to_string(), size);
    // node.set_attribute("maxsize".to_string(), size);
    //
    // return node;
}

lazy_static! {
    pub static ref BIG: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_arg_types(vec![ArgType::primitive]);

        FunctionDefSpec {
            def_type: "delimsizing".to_string(),
            names: vec![
                "\\bigl".to_string(),
                "\\Bigl".to_string(),
                "\\biggl".to_string(),
                "\\Biggl".to_string(),
                "\\bigr".to_string(),
                "\\Bigr".to_string(),
                "\\biggr".to_string(),
                "\\Biggr".to_string(),
                "\\bigm".to_string(),
                "\\Bigm".to_string(),
                "\\biggm".to_string(),
                "\\Biggm".to_string(),
                "\\big".to_string(),
                "\\Big".to_string(),
                "\\bigg".to_string(),
                "\\Bigg".to_string(),
            ],
            props,
            handler: big_handler_fn,
            html_builder: Some(big_html_builder),
            mathml_builder: Some(big_mathml_builder),
        }
    });
}

fn assert_parsed(group: &parse_node::types::leftright) {
    if group.body.len() > 0 {
        panic!("Bug: The leftright ParseNode wasn't fully parsed.");
    }
}

pub fn lrr_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut context = ctx.borrow_mut();
    // \left case below triggers parsing of \right in
    //   `let right = parser.parseFunction();`
    // uses this return value.
    let _color = context
        .parser
        .gullet
        .macros
        .get(&"\\current@color".to_string());
    let color = if let Some(c) = _color {
        if let MacroDefinition::Str(s) = c {
            Some(s.clone())
        } else {
            None
        }
    } else {
        None
    };
    // if color && typeof color != "string".to_string()) {
    //     panic!("\\current@color set to non-string in \\right".to_string());
    // }
    let res = parse_node::types::leftright_right {
        mode: context.parser.mode,
        loc: None,
        delim: check_delimiter(&args[0], &context.func_name),
        color, // undefined if not set via \color
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

lazy_static! {
    pub static ref LEFTRIGHT_RIGHT: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_primitive(true);

        FunctionDefSpec {
            def_type: "leftright-right".to_string(),
            names: vec!["\\right".to_string()],
            props,
            handler: lrr_handler_fn,
            html_builder: Some(big_html_builder),
            mathml_builder: Some(big_mathml_builder),
        }
    });
}

pub fn lr_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut context = ctx.borrow_mut();
    let delim_text = check_delimiter(&args[0], &context.func_name);
    // Parse out the implicit body
    context.parser.left_right_depth += 1;
    // parseExpression stops before '\\right'
    let body = context.parser.parse_expression(false, None);
    context.parser.left_right_depth -= 1;
    // Check the next token
    context.parser.expect("\\right".to_string(), false);
    let r_tmp = context.parser.parse_function(None, "".to_string()).unwrap();
    let right = r_tmp
        .as_any()
        .downcast_ref::<parse_node::types::leftright_right>()
        .unwrap();
    let res = parse_node::types::leftright {
        mode: context.parser.mode,
        loc: None,
        body,
        left: delim_text.clone(),
        right: right.delim.clone(),
        right_color: right.color.clone(),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

const IS_MIDDLE: &str = "is_middle_with_an_ungly_name";
pub fn lr_html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::leftright>()
        .unwrap();
    assert_parsed(group);

    let mut innerHeight = 0.0;
    let mut innerDepth = 0.0;
    let mut hadMiddle = false;
    // Build the inner expression
    let mut inner : Vec<Box<dyn HtmlDomNode>> = HTML::build_expression(
        group.body.clone(),
        options.clone(),
        IsRealGroup::T,
        (Some(DomType::mopen), Some(DomType::mclose)),
    ).into_iter()
        .map(|inner_item|{
        if let Some(middle_span) = inner_item.as_any().downcast_ref::<IsMiddleSpan>(){
            hadMiddle = true;
            let res = crate::delimiter::make_left_right_delim(
               &middle_span.is_middle.delim,
                innerHeight,
                innerDepth,
                &middle_span.is_middle.options,
                group.mode,
                vec![],
            );
            Box::new(res ) as Box<dyn HtmlDomNode>

        }else{
            // Calculate its height and depth
            innerHeight = f64::max(inner_item.get_height(), innerHeight);
            innerDepth = f64::max(inner_item.get_depth(), innerDepth);
            inner_item
        }
    }).collect();

    // The size of delimiters is the same, regardless of what style we are
    // in. Thus, to correctly calculate the size of delimiter we need around
    // a group, we scale down the inner size based on the size.
    innerHeight *= options.sizeMultiplier;
    innerDepth *= options.sizeMultiplier;

    let left_delim= 
    if group.left == ".".to_string() {
        // Empty delimiters in \left and \right make null delimiter spaces.
         HTML::make_null_delimiter(&options, vec!["mopen".to_string()])
    } else {
        // Otherwise, use leftRightDelim to generate the correct sized
        // delimiter.
         crate::delimiter::make_left_right_delim(
            &group.left,
            innerHeight,
            innerDepth,
            &options,
            group.mode,
            vec!["mopen".to_string()],
        )
    };
    // Add it to the beginning of the expression
    inner.insert(0,Box::new(left_delim) as Box<dyn HtmlDomNode>);

    let right_delim=
    // Same for the right delimiter, but using color specified by \color
    if group.right == ".".to_string() {
        HTML::make_null_delimiter(&options, vec!["mclose".to_string()])
    } else {
        let color_options = if let Some(s) = &group.right_color {
            options.with_color(s.clone())
        } else {
            options.clone()
        };
         crate::delimiter::make_left_right_delim(
            &group.right,
            innerHeight,
            innerDepth,
            &color_options,
            group.mode,
            vec!["mclose".to_string()],
        )
    };
    // Add it to the end of the expression.
    inner.push(Box::new(right_delim) as Box<dyn HtmlDomNode>);

    let res = common::make_span(
        vec!["minner".to_string()],
        inner,
        Some(&options),
        Default::default(),
    );
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

pub fn lr_mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined");
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::leftright>()
        .unwrap();
    // assert_parsed(group);
    // let inner = mml::build_expression(group.body, options);
    //
    // if group.left != ".".to_string() {
    //     let leftNode = MathNode::new("mo".to_string(), [mml.makeText(group.left, group.mode)]);
    //
    //     leftNode.set_attribute("fence".to_string(), "true".to_string());
    //
    //     inner.unshift(leftNode);
    // }
    //
    // if (group.right != ".".to_string()) {
    //     let rightNode = MathNode::new("mo".to_string(), [mml.makeText(group.right, group.mode)]);
    //
    //     rightNode.set_attribute("fence".to_string(), "true".to_string());
    //
    //     if (group.rightColor) {
    //         rightNode.set_attribute("mathcolor".to_string(), group.rightColor);
    //     }
    //
    //     inner.push(rightNode);
    // }
    //
    // return mml.makeRow(inner);
}

lazy_static! {
    pub static ref LEFTRIGHT: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_primitive(true);

        FunctionDefSpec {
            def_type: "leftright".to_string(),
            names: vec!["\\left".to_string()],
            props,
            handler: lr_handler_fn,
            html_builder: Some(lr_html_builder),
            mathml_builder: Some(lr_mathml_builder),
        }
    });
}

pub fn middle_handler_fn(
    ctx: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let delim_text = check_delimiter(&args[0], &ctx.borrow().func_name);
    let context = ctx.borrow_mut();
    if context.parser.left_right_depth == 0 {
        panic!("\\middle without preceding \\left {}", delim_text);
    }

    let res = parse_node::types::middle {
        mode: context.parser.mode,
        loc: None,
        delim: delim_text,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

pub fn middle_html_builder(
    _group: Box<dyn AnyParseNode>,
    options: Options,
) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::middle>()
        .unwrap();
    if group.delim == ".".to_string() {
        let middle_delim = HTML::make_null_delimiter(&options, vec![]);
        return Box::new(middle_delim) as Box<dyn HtmlDomNode>;
    } else {
        let middle_delim = IsMiddleSpan::new(
            crate::delimiter::make_sized_delim(&group.delim, 1, &options, group.mode, vec![]),
IsMiddle {
            delim: group.delim.clone(),
            options,
        }
        );
        return Box::new(middle_delim) as Box<dyn HtmlDomNode>;
        // Property `isMiddle` not defined on `span`. It is only used in
        // this file above.
        // TODO: Fix this violation of the `span` type and possibly rename
        // things since `isMiddle` sounds like a boolean, but is a struct.
    }
}

pub fn middle_mathml_builder(
    _group: Box<dyn AnyParseNode>,
    options: Options,
) -> Box<dyn MathDomNode> {
    panic!("undefined");
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::middle>()
        .unwrap();

    // A Firefox \middle will strech a character vertically only if it
    // is in the fence part of the operator dictionary at:
    // https://www.w3.org/TR/MathML3/appendixc.html.
    // So we need to avoid U+2223 and use plain "|" instead.
    // let textNode = if (group.delim == "\\vert" || group.delim == "|".to_string()) {
    //     mml.makeText("|".to_string(), "text".to_string())
    // } else {
    //     mml.makeText(group.delim, group.mode)
    // };
    // let middleNode = MathNode::new(MathNodeType::Mo, vec![textNode]);
    // middleNode.set_attribute("fence".to_string(), "true".to_string());
    // // MathML gives 5/18em spacing to each <mo> element.
    // // \middle should get delimiter spacing instead.
    // middleNode.set_attribute("lspace".to_string(), "0.05em".to_string());
    // middleNode.set_attribute("rspace".to_string(), "0.05em".to_string());
    // return middleNode;
}
lazy_static! {
    pub static ref MIDDLE: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_primitive(true);

        FunctionDefSpec {
            def_type: "middle".to_string(),
            names: vec!["\\middle".to_string()],
            props,
            handler: middle_handler_fn,
            html_builder: Some(middle_html_builder),
            mathml_builder: Some(middle_mathml_builder),
        }
    });
}
