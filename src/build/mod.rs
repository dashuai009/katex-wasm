pub(crate) mod HTML;
pub(crate) mod common;
pub(crate) mod mathML;


use crate::{
    dom_tree::{css_style::CssStyle, span::Span},
    parse_node::types::AnyParseNode,
    settings::Settings,
    tree::HtmlDomNode,
    Options::Options,
};

pub fn display_wrap(node: Span, settings: Settings) -> Span {
    if settings.get_display_mode() {
        let mut classes = vec!["katex-display".to_string()];
        if settings.leqno {
            classes.push("leqno".to_string());
        }
        if settings.fleqn {
            classes.push("fleqn".to_string());
        }
        return common::make_span(
            classes,
            vec![Box::new(node) as Box<dyn HtmlDomNode>],
            None,
            CssStyle::new(),
        );
    }
    return node;
}
pub fn build_tree(
    tree: Vec<Box<dyn AnyParseNode>>,
    expression: String,
    settings: Settings,
) -> Span {
    let options = Options::from_settings(&settings);
    println!("build::build_tree options = {:#?}", options);
    let katex_node;
    if settings.get_output() == "mathml" {
        return mathML::build_math_ml(
            tree,
            expression,
            options,
            settings.get_display_mode(),
            true,
        );
    } else if settings.get_output() == "html" {
        let html_node = HTML::build_html(tree, options);
        katex_node = common::make_span(
            vec!["katex".to_string()],
            vec![Box::new(html_node) as Box<dyn HtmlDomNode>],
            None,
            CssStyle::new(),
        );
    } else {
        let math_mlnode = mathML::build_math_ml(
            tree.clone(),
            expression,
            options.clone(),
            settings.get_display_mode(),
            false,
        );
        let html_node = HTML::build_html(tree, options);
        katex_node = common::make_span(
            vec!["katex".to_string()],
            vec![
                Box::new(math_mlnode) as Box<dyn HtmlDomNode>,
                Box::new(html_node) as Box<dyn HtmlDomNode>,
            ],
            None,
            CssStyle::new(),
        );
    }

    return display_wrap(katex_node, settings);
}

pub fn build_html_tree(
    tree: Vec<Box<dyn AnyParseNode>>,
    expression: String,
    settings: Settings,
) -> Span {
    let options = Options::from_settings(&settings);
    let html_node = HTML::build_html(tree, options);
    let katex_node =  common::make_span(
        vec!["katex".to_string()],
        vec![Box::new(html_node) as Box<dyn HtmlDomNode>],
        None,
        CssStyle::new(),
    );
    return display_wrap(katex_node, settings);
}
