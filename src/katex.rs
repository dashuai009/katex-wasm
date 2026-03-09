use crate::build::common::make_span;
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::dom_tree::symbol_node::SymbolNode;
use crate::parse_node::types::AnyParseNode;
use crate::parse::parse_tree_with_error;
use crate::parse_error::ParseError;
use crate::settings::Settings;
use crate::tree::HtmlDomNode;
use crate::utils::escape_to;
use crate::VirtualNode;
use std::panic::{catch_unwind, AssertUnwindSafe};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

fn format_parse_error(error: &ParseError) -> String {
    format!("ParseError: {}", error)
}

fn render_error_dom(error: &ParseError, expression: &str, settings: &Settings) -> Span {
    let mut node = make_span(
        vec!["katex-error".to_string()],
        vec![Box::new(SymbolNode::new(expression.to_string())) as Box<dyn HtmlDomNode>],
        None,
        CssStyle::default(),
    );
    node.set_attribute("title".to_string(), format_parse_error(error));
    node.set_attribute(
        "style".to_string(),
        format!("color:{}", settings.get_error_color()),
    );
    node
}

fn render_error_markup(error: &ParseError, expression: &str, settings: &Settings) -> String {
    let mut markup = String::new();
    markup.push_str("<span class=\"katex-error\" title=\"");
    escape_to(&mut markup, &format_parse_error(error));
    markup.push_str("\" style=\"color:");
    escape_to(&mut markup, &settings.get_error_color());
    markup.push_str("\">");
    escape_to(&mut markup, expression);
    markup.push_str("</span>");
    return markup;
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = payload.downcast_ref::<&str>() {
        message.to_string()
    } else {
        "internal render error".to_string()
    }
}

fn contains_infix_nodes(nodes: &[Box<dyn AnyParseNode>]) -> bool {
    nodes.iter().any(contains_infix_node)
}

fn contains_infix_node(node: &Box<dyn AnyParseNode>) -> bool {
    if node.get_type() == "infix" {
        return true;
    }

    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::array>() {
        return group.body.iter().any(|row| contains_infix_nodes(row));
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::color>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::op>() {
        return group
            .body
            .as_ref()
            .is_some_and(|body| contains_infix_nodes(body));
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::ordgroup>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::styling>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::supsub>() {
        return group.base.as_ref().is_some_and(contains_infix_node)
            || group.sup.as_ref().is_some_and(contains_infix_node)
            || group.sub.as_ref().is_some_and(contains_infix_node);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::tag>() {
        return contains_infix_nodes(&group.body) || contains_infix_nodes(&group.tag);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::text>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::accent>() {
        return group.base.as_ref().is_some_and(contains_infix_node);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::accentUnder>() {
        return contains_infix_node(&group.base);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::enclose>() {
        return contains_infix_node(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::font>() {
        return contains_infix_node(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::genfrac>() {
        return contains_infix_node(&group.numer) || contains_infix_node(&group.denom);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::hbox>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::horizBrace>() {
        return contains_infix_node(&group.base);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::href>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::html>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::htmlmathml>() {
        return contains_infix_nodes(&group.html) || contains_infix_nodes(&group.mathml);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::lap>() {
        return contains_infix_node(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::leftright>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::mathchoice>() {
        return contains_infix_nodes(&group.display)
            || contains_infix_nodes(&group.text)
            || contains_infix_nodes(&group.script)
            || contains_infix_nodes(&group.scriptscript);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::mclass>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::operatorname>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::overline>() {
        return contains_infix_node(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::phantom>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::hphantom>() {
        return contains_infix_node(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::vphantom>() {
        return contains_infix_node(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::pmb>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::raisebox>() {
        return contains_infix_node(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::sizing>() {
        return contains_infix_nodes(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::smash>() {
        return contains_infix_node(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::sqrt>() {
        return contains_infix_node(&group.body)
            || group.index.as_ref().is_some_and(contains_infix_node);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::underline>() {
        return contains_infix_node(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::vcenter>() {
        return contains_infix_node(&group.body);
    }
    if let Some(group) = node.as_any().downcast_ref::<crate::parse_node::types::xArrow>() {
        return contains_infix_node(&group.body)
            || group.below.as_ref().is_some_and(contains_infix_node);
    }

    false
}

fn build_dom_tree(expression: &str, settings: &Settings) -> Result<Span, ParseError> {
    match catch_unwind(AssertUnwindSafe(|| {
        let tree = parse_tree_with_error(expression.to_string(), settings.clone())?;
        if contains_infix_nodes(&tree) {
            return Err(ParseError {
                msg: "Got group of unknown type: 'infix'".to_string(),
                loc: None,
            });
        }
        Ok(crate::build::build_tree(
            tree,
            expression.to_string(),
            settings.clone(),
        ))
    })) {
        Ok(result) => result,
        Err(payload) => Err(ParseError {
            msg: panic_message(payload),
            loc: None,
        }),
    }
}

/**
 * Generates and returns the katex build tree. This is used for advanced
 * use cases (like rendering to custom output).
 */
pub fn render_to_dom_tree(expression: String, settings: Settings) -> Span {
    match build_dom_tree(&expression, &settings) {
        Ok(tree) => tree,
        Err(error) => {
            if settings.get_throw_on_error() {
                panic!("{}", format_parse_error(&error));
            }
            render_error_dom(&error, &expression, &settings)
        }
    }
}

#[wasm_bindgen]
pub fn render(expression: String, base_node: &web_sys::Node, options: &JsValue) {
    base_node.set_text_content(Some(""));
    let node = render_to_dom_tree(expression, Settings::new_from_js(options)).to_node();
    base_node.append_child(&node);
}

/**
 * Parse and build an expression, and return the markup for that.
 */
pub fn render_to_string(expression: String, settings: Settings) -> String {
    match build_dom_tree(&expression, &settings) {
        Ok(tree) => tree.to_markup(),
        Err(error) => {
            if settings.get_throw_on_error() {
                panic!("{}", format_parse_error(&error));
            }
            render_error_markup(&error, &expression, &settings)
        }
    }
}

#[wasm_bindgen(js_name = renderToString)]
pub fn render_to_string_for_js(expression: String, settings: &JsValue) -> String {
    return render_to_string(expression, Settings::new_from_js(settings));
}

const TEST_CASE: [&str; 1] = [
    // "E=mc^2",
    // "a^2+b^2=c^2",
    // "\\\"{A}",
    // "\\underleftarrow{AB} \\underrightarrow{AB} \\underleftrightarrow{AB} \\underlinesegment{AB} \\undergroup{AB} \\utilde{AB} \\xleftarrow{abc} \\xrightarrow{abc}  \\xLeftarrow{abc}  \\xRightarrow{abc} \\xleftrightarrow{abc}  \\xLeftrightarrow{abc}  \\xhookleftarrow{abc}  \\xhookrightarrow{abc}  \\xmapsto{abc}  \\xrightharpoondown{abc}  \\xrightharpoonup{abc}  \\xleftharpoondown{abc}  \\xleftharpoonup{abc} \\xrightleftharpoons{abc}  \\xleftrightharpoons{abc}  \\xlongequal{abc} \\xtwoheadrightarrow{abc}  \\xtwoheadleftarrow{abc}  \\xtofrom{abc} \\xrightleftarrows{abc}  \\xrightequilibrium{abc}  \\xleftequilibrium{abc}",
    // "\\\\cdrightarrow{abc}  \\\\cdleftarrow{abc}  \\\\cdlongequal{abc}", // untested
    // "F=ma \\\\ hahaha",
    // "\\cancel{5}",
    // r"\frac{1}{2}",
    // r"\overbrace{AB} \underbrace{AB}",
    // r"\href{https://www.dashuai009.icu}{dashuai009} \url{https:www.dashuai009.icu} \textbf{Ab0} \textit{Ab0} \textrm{Ab0} \textup{Ab0} \textnormal{Ab0} \text{Ab0} \textmd{Ab0} \textsf{Ab0}",
    // r"\hbox{a}",
    // r"\htmlId{bar}{x} \htmlClass{foo}{x} \htmlStyle{color: red;}{x} \htmlData{foo=a, bar=b}{x}",
    // r"\sqrt{a^2+b^2} = 1",
    // r"\includegraphics[height=0.8em, totalheight=100px, width=150px, alt=KA logo]{https://katex.org/img/khan-academy.png}",
    // r"{=}\mathllap{/} \mathrlap{/}{=} \mathclap{1\le i\le j\le n}",
    // r"\mathchoice{D}{T}{S}{SS}",
    // r"\sum_{i}",
    // r"\coprod^a",
    // r"\overline{A}",
    // r"b\phantom{content}a\hphantom{content}c\vphantom{content}d",
    // r"\pmb{\mu} \mu",
    // r"a\raisebox{0.25em}{b}c",
    // r" x^{\smash[a]{2}} ",
    // r"\underline{AB}",
    // r"\mathrm{Ab0} \mathbf{Ab0} \mathit{Ab0}",
    // r"\mathnormal{Ab0} \mathbb{Ab} \mathcal{Ab} \mathfrak{Ab0} \mathscr{Ab} \mathsf{Ab0} \Bbb{Ab} \bold{Ab0} \frak{Ab0}",
    // r"\boldsymbol{Ab} \bm{Ab0} \rm{a} \sf{A} \tt{a} \bf{aB0} \it{Ab0} \cal{Ab0}",
    // r"\big(\big) \Big(\Big) \tiny tiny \Huge huge",
    // r"I\kern-2.5pt R a\mkern18mu b 	a\mskip{10mu}b 	a\mskip{10mu}b",
    //r"a\sqrt{\frac{a}{b}}"
    // r"\vec{A}\vec{x}\vec x^2\vec{x}_2^2\vec{A}^2\vec{xA}^2\; \underbar{X}",
    //r"a+b-c\cdot d/e"
    //r"\dbinom{a}{b}\tbinom{a}{b}^{\binom{a}{b}+17}{\scriptscriptstyle \binom{a}{b}}"
    //r"\mathbf{A}^2+\mathbf{B}_3*\mathscr{C}'"
    //   r"\sum_{\boldsymbol{\alpha}}^{\boldsymbol{\beta}} \boldsymbol{\omega}+ \boldsymbol{\int_\alpha^\beta} \boldsymbol{\Omega + {}} \\
    // \boldsymbol{\lim_{x \to \infty} \log Ax2k\omega\Omega\imath+} \\
    // x \boldsymbol{+} y \boldsymbol{=} z"
    // r"    \begin{array}{lc}
    // a & a \\
    // b & b
    // \end{array}",
    // r"\begin{align}
    //   a&= b\\
    //   b= & v\\
    //   \end{align}",
   r"\tan\left(\frac{\pi}{4}\right)=1"
];

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::process::{Command, Stdio};
    use crate::katex::{render, render_to_string, TEST_CASE};
    use crate::settings::Settings;

    fn render_with_js_katex(expression: &str) -> String {
        let script = r#"
const fs = require('fs');
const payload = JSON.parse(fs.readFileSync(0, 'utf8'));
const katex = require('./KaTeX/dist/katex.mjs');
let options = {
  displayMode: true,
  output: 'html',
  throwOnError: false,
  trust: true,
  strict: 'ignore'
};
 var settings = new katex.Settings(options);
 process.stdout.write(JSON.stringify(settings));
  try {
    var tree = katex.parseTree(payload.expression, settings);
    process.stdout.write(JSON.stringify(tree, null, 2));
    // return buildTree(tree, payload.expression, settings);
  } catch (error) {
     process.stdout.write(JSON.stringify(error));
    // return renderError(error, payload.expression, settings);
  }

const html = katex.renderToString(payload.expression, options);
process.stdout.write(html);
"#;

        let mut child = Command::new("node")
            .arg("-e")
            .arg(script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("node must be available to run parity test");

        let payload = format!(
            r#"{{"expression":"{}"}}"#,
            expression.replace('\\', "\\\\").replace('"', "\\\"")
        );

        let mut stdin = child.stdin.take().expect("stdin should be available");
        stdin
            .write_all(payload.as_bytes())
            .expect("failed to pass formula payload to node");
        drop(stdin);

        let output = child
            .wait_with_output()
            .expect("failed to wait for node renderer");
        assert!(
            output.status.success(),
            "node renderer failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        String::from_utf8(output.stdout).expect("node output must be utf-8")
    }
    #[test]
    fn test_parse_tree() {




        let mut settings = Settings::new();
        settings.set_display_mode(true);
        settings.set_error_color("#cc0000".to_string());
        settings.set_trust(true);

        settings.set_max_expand(Some(1000));
        settings.set_max_size(Some(200000.0));
        println!("setting = {:#?}", settings);
        for test_str in TEST_CASE {
            println!("{test_str}");
        }
        for test_string in TEST_CASE {

            let js_katex = render_with_js_katex(test_string);
            println!("js_katex = {}", js_katex);

            println!(
                "{}",
                render_to_string(test_string.to_string(), settings.clone()).as_str()
            );
        }
    }
}

/*****
具有纪念意义的一行输出
<span class="katex-display"><span class="katex"><span class="katex-html" aria-hidden=true><span class="base"><span class="mspace" style="margin-right:0.2778em;"></span><span class="mord mathnormal" style="margin-right:0.05764em;">E</span><span class="mspace" style="margin-right:0.2778em;"></span><span class="mrel">=</span></span><span class="base"><span class="mord mathnormal">m</span><span class="mord"><span class="mord mathnormal">c</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span class="vlist" style="height:0.8641em;"><span style="margin-right:0.0500em;top:-3.1130em;"><span class="pstrut" style="height:2.7000em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord   mtight">2</span></span></span></span></span></span></span></span></span></span></span></span>
 */
