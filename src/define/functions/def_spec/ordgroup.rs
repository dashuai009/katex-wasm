use crate::build::common::make_span;
use crate::build::mathML;
use crate::build::HTML::IsRealGroup;
use crate::mathML_tree::public::MathDomNode;
use crate::Options::Options;
use crate::{parse_node, AnyParseNode, HtmlDomNode};
use crate::define::functions::public::{
    ord_argument, FunctionContext, FunctionDefSpec, FunctionPropSpec,
};
use std::sync::Mutex;

fn html_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn HtmlDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::ordgroup>()
        .unwrap();
    if group.semisimple {
        return Box::new(crate::build::common::make_fragment(crate::build::HTML::build_expression(
            group.body.clone(),
            options.clone(),
            IsRealGroup::F,
            (None, None),
        ))) as Box<dyn HtmlDomNode>;
    }
    return Box::new(make_span(vec!["mord".to_string()], crate::build::HTML::build_expression(
        group.body.clone(),
        options.clone(),
        IsRealGroup::T,
        (None, None),
    ), Some(&options), Default::default())) as Box<dyn HtmlDomNode>;
}

fn mathml_builder(_group: Box<dyn AnyParseNode>, options: Options) -> Box<dyn MathDomNode> {
    let group = _group
        .as_any()
        .downcast_ref::<parse_node::types::ordgroup>()
        .unwrap();
    return mathML::build_expression_row(group.body.clone(), options, true);
}

lazy_static! {
    pub static ref ORDGROUP: Mutex<FunctionDefSpec> = Mutex::new({
        let mut props = FunctionPropSpec::new();

        FunctionDefSpec {
            def_type: "ordgroup".to_string(),
            names: vec![],
            props,
            handler: |a, b, c| panic!("error"),
            html_builder: Some(html_builder),
            mathml_builder: Some(mathml_builder),
        }
    });
}
