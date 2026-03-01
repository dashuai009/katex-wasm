use crate::build::common::make_span;
use crate::dom_tree::css_style::CssStyle;
use crate::dom_tree::span::Span;
use crate::dom_tree::symbol_node::SymbolNode;
use crate::parse::parse_tree_with_error;
use crate::parse_error::ParseError;
use crate::settings::Settings;
use crate::tree::HtmlDomNode;
use crate::utils::escape;
use crate::VirtualNode;
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
    format!(
        "<span class=\"katex-error\" title=\"{}\" style=\"color:{}\">{}</span>",
        escape(&format_parse_error(error)),
        escape(&settings.get_error_color()),
        escape(&expression.to_string()),
    )
}

fn build_dom_tree(expression: &str, settings: &Settings) -> Result<Span, ParseError> {
    let tree = parse_tree_with_error(expression.to_string(), settings.clone())?;
    Ok(crate::build::build_tree(
        tree,
        expression.to_string(),
        settings.clone(),
    ))
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
