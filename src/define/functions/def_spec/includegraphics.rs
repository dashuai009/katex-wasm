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
use crate::settings::TrustContext;
use crate::utils::is_character_box;
use crate::Options::Options;
use crate::{parse_node, types::ArgType, AnyParseNode, HtmlDomNode, Measurement};
use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;

fn size_data(s: &str) -> crate::Measurement {
    lazy_static! {
        static ref test1: Regex = Regex::new(r"^[-+]? *(\d+(\.\d*)?|\.\d+)$").unwrap();
        static ref test2: Regex =
            Regex::new(r"([-+]?) *(\d+(?:\.\d*)?|\.\d+) *([a-z]{2})").unwrap();
    }
    if test1.is_match(s) {
        // str is a number with no unit specified.
        // default unit is bp, per graphix package.
        return Measurement {
            number: s.parse().unwrap(),
            unit: "bp".to_string(),
        };
    } else {
        if let Some(cap) = test2.captures_iter(s).next() {
            let data = Measurement {
                number: format!("{}{}", &cap[1], &cap[2]).parse().unwrap(), // sign + magnitude, cast to number
                unit: (&cap[3]).to_string(),
            };
            if !data.validUnit() {
                panic!("Invalid unit: '{}' in \\includegraphics.", data.unit);
            }
            return data;
        } else {
            panic!("Invalid size: '{s}' in \\includegraphics");
        }
    }
}

fn includegraphics_handler_fn(
    context: FunctionContext,
    args: Vec<Box<dyn AnyParseNode>>,
    opt_args: Vec<Option<Box<dyn AnyParseNode>>>,
) -> Box<dyn AnyParseNode> {
    let mut width = Measurement {
        number: 0.0,
        unit: "em".to_string(),
    };
    let mut height = Measurement {
        number: 0.9,
        unit: "em".to_string(),
    }; // sorta character sized.
    let mut totalheight = Measurement {
        number: 0.0,
        unit: "em".to_string(),
    };
    let mut alt = "".to_string();

    if let Some(opt_args_0) = &opt_args[0] {
        let raw_arg = opt_args_0
            .as_any()
            .downcast_ref::<parse_node::types::raw>()
            .unwrap();
        // Parser.js does not parse key/value pairs. We get a string.
        for key_val in raw_arg
            .string
            .split(",")
            .map(|s| s.split("=").collect::<Vec<&str>>())
        {
            if key_val.len() == 2 {
                let key = key_val[0].trim();
                let val = key_val[1].trim();
                match key {
                    "alt" => {
                        alt = val.to_string();
                    }
                    "width" => {
                        width = size_data(val);
                    }
                    "height" => {
                        height = size_data(val);
                    }
                    "totalheight" => {
                        totalheight = size_data(val);
                    }
                    _ => {
                        panic!("Invalid key: '{}' in \\includegraphics.", key);
                    }
                }
            }
        }
    }
    let url_node = args[0]
        .as_any()
        .downcast_ref::<parse_node::types::url>()
        .unwrap();

    let src = &url_node.url;

    lazy_static! {
        static ref suf: Regex = Regex::new(r"^.*[\\/]").unwrap();
    }
    if alt == ""  {
        // No alt given. Use the file name. Strip away the path.
        alt = suf.replace_all(src, "").to_string();
        alt = alt[0..alt.rfind('.').unwrap_or(alt.len())].to_string();
    }

    let trust_context = TrustContext {
        command: "\\includegraphics".to_string(),
        context: HashMap::from([("url".to_string(), "src".to_string())]),
    };
    if !context.parser.settings.is_trusted(&trust_context) {
        let res = context.parser.format_unsupported_cmd("\\includegraphics");
        return Box::new(res) as Box<dyn AnyParseNode>;
    }

    let res = parse_node::types::includegraphics {
        mode: context.parser.mode,
        loc: None,
        alt: alt,
        width: width,
        height: height,
        totalheight: totalheight,
        src: src.clone(),
    };
    return Box::new(res) as Box<dyn AnyParseNode>;
}

pub fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::includegraphics>()
        .unwrap();
    let height = crate::units::calculate_size(&group.height, &options);
    let mut depth = 0.0;

    if group.totalheight.number > 0.0 {
        depth = crate::units::calculate_size(&group.totalheight, &options) - height;
    }

    let mut width = 0.0;
    if group.width.number > 0.0 {
        width = crate::units::calculate_size(&group.width, &options);
    }

    let mut style: CssStyle = CssStyle {
        height: Some(crate::units::make_em(height + depth)),
        ..Default::default()
    };
    if width > 0.0 {
        style.width = Some(crate::units::make_em(width));
    }
    if depth > 0.0 {
        style.vertical_align = Some(crate::units::make_em(-depth));
    }

    let mut node = crate::dom_tree::img::Img::new(group.src.clone(), group.alt.clone(), style);
    node.set_height(height);
    node.set_depth( depth);

    return Box::new(node) as Box<dyn HtmlDomNode>;
}

pub fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::includegraphics>()
        .unwrap();
    let mut node = MathNode::new(MathNodeType::Mglyph, vec![], vec![]);
    node.set_attribute("alt".to_string(), group.alt.clone());

    let height = crate::units::calculate_size(&group.height, &options);
    let mut depth = 0.0;
    if group.totalheight.number > 0.0 {
        depth = crate::units::calculate_size(&group.totalheight, &options) - height;
        node.set_attribute("valign".to_string(), crate::units::make_em(-depth));
    }
    node.set_attribute("height".to_string(), crate::units::make_em(height + depth));

    if group.width.number > 0.0 {
        let width = crate::units::calculate_size(&group.width, &options);
        node.set_attribute("width".to_string(), crate::units::make_em(width));
    }
    node.set_attribute("src".to_string(), group.src.clone());
    return Box::new(node) as Box<dyn MathDomNode>;
}

lazy_static! {
    pub static ref INCLUDE_GRAPHICS: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();
        props.set_num_args(1);
        props.set_num_optional_args(1);
        props.set_arg_types(vec![ArgType::raw, ArgType::url]);
        props.set_allowed_in_text(false);

        FunctionDefSpec {
            def_type: "includegraphics".to_string(),
            names: vec!["\\includegraphics".to_string()],
            props,
            handler: includegraphics_handler_fn,
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
