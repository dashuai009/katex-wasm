use crate::build::common::{PositionType, VListChild, VListParam};
use crate::build::HTML::IsRealGroup;
use crate::build::{common, mathML, HTML};
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
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode};
use std::sync::Mutex;

fn handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut smash_height = false;
    let mut smash_depth = false;
    //optArgs[0] && assertNodeType(optArgs[0], "ordgroup".to_string());
    if opt_args.len() == 0 || opt_args[0].is_none() {
        // 可选参数为空
        smash_height = true;
        smash_depth = true;
    } else if let Some(tb_arg) = opt_args[0]
        .as_ref()
        .unwrap()
        .as_any()
        .downcast_ref::<parse_node::types::ordgroup>()
    {
        // Optional [tb] argument is engaged.
        // ref: amsmath: \renewcommand{\smash}[1][tb]{%
        //               def\mb@t{\ht}\def\mb@b{\dp}\def\mb@tb{\ht\z@\z@\dp}%
        for node in tb_arg.body.iter() {
            // Not every node type has a `text` property.
            //letter = node.text;
            assert_eq!(node.as_any().type_id(), std::any::TypeId::of::<parse_node::types::mathord>());
            if let Some(letter) = node.as_any().downcast_ref::<parse_node::types::mathord>().map(|x|&x.text){
                if letter == "t" {
                    smash_height = true;
                } else if letter == "b"  {
                    smash_depth = true;
                } else {
                    smash_height = false;
                    smash_depth = false;
                    break;
                }

            }else {
                smash_height = false;
                smash_depth = false;
                break;
            }
        }
    } else {
        smash_height = true;
        smash_depth = true;
    }

    let body = args[0].clone();
    let res = parse_node::types::smash {
        mode: context.parser.mode,
        loc: None,
        body,
        smash_height,
        smash_depth,
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::smash>()
        .unwrap();
    let mut node = common::make_span(
        vec![],
        vec![HTML::build_group(
            Some(group.body.clone()),
            options.clone(),
            None,
        )],
        None,
        Default::default(),
    );

    if (!group.smash_height && !group.smash_depth) {
        return Box::new(node) as Box<dyn HtmlDomNode>;
    }

    if group.smash_height {
        node.set_height(0.0);
        // In order to influence makeVList, we have to reset the children.

        for child in node.get_mut_children().unwrap().into_iter() {
            child.set_height(0.0);
        }
    }

    if group.smash_depth {
        node.set_depth(0.0);
        for child in node.get_mut_children().unwrap().into_iter() {
            child.set_depth(0.0);
        }
    }

    // At this point, we've reset the TeX-like height and depth values.
    // But the span still has an HTML line height.
    // makeVList applies "display: table-cell".to_string(), which prevents the browser
    // from acting on that line height. So we'll call makeVList now.

    let smashed_node = common::make_vlist(
        VListParam {
            position_type: PositionType::FirstBaseline,
            children: vec![VListChild::Elem {
                elem: Box::new(node) as Box<dyn HtmlDomNode>,
                margin_left: None,
                margin_right: None,
                wrapper_classes: None,
                wrapper_style: None,
                shift: None,
            }],
            position_data: None,
        },
        options.clone(),
    );

    // For spacing, TeX treats \hphantom as a math group (same spacing as ord).
    let res = common::make_span(
        vec!["mord".to_string()],
        vec![Box::new(smashed_node) as Box<dyn HtmlDomNode>],
        Some(&options),
        Default::default(),
    );
    return Box::new(res) as Box<dyn HtmlDomNode>;
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    panic!("undefined")
    // let node = MathNode::new(
    // "mpadded".to_string(), [mathML::build_group(group.body, options)]);
    //
    // if (group.smashHeight) {
    // node.set_attribute("height".to_string(), "0px".to_string());
    // }
    //
    // if (group.smashDepth) {
    // node.set_attribute("depth".to_string(), "0px".to_string());
    // }
    //
    // return node;
}

// smash, with optional [tb], as in AMS

lazy_static! {
    pub static ref SMASH: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_num_optional_args(1);
        props.set_allowed_in_text(true);

        FunctionDefSpec {
            def_type: "smash".to_string(),
            names: vec!["\\smash".to_string()],
            props,
            handler: handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
