use crate::dom_tree::span::Span;
use crate::parse::parseTree;
use crate::settings::Settings;
use crate::VirtualNode;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

/**
 * Generates and returns the katex build tree. This is used for advanced
 * use cases (like rendering to custom output).
 */
pub fn render_to_dom_tree(expression: String, settings: Settings) -> Span {
    let tree = parseTree(expression.clone(), settings.clone());
    println!("tree parse nodes = {:#?}", tree);
    return crate::build::build_tree(tree, expression, settings);
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
    return render_to_dom_tree(expression, settings).to_markup();
}

#[wasm_bindgen(js_name = renderToString)]
pub fn render_to_string_for_js(expression:String, settings:  &JsValue)->String{
    return render_to_string(expression,Settings::new_from_js(settings));
}




const TEST_CASE: [&str; 1] = [
    // "E=mc^2",
    // "a^2+b^2=c^2",
    // "\\\"{A}",
    // "\\underleftarrow{AB} \\underrightarrow{AB} \\underleftrightarrow{AB} \\underlinesegment{AB} \\undergroup{AB} \\utilde{AB} \\xleftarrow{abc} \\xrightarrow{abc}  \\xLeftarrow{abc}  \\xRightarrow{abc} \\xleftrightarrow{abc}  \\xLeftrightarrow{abc}  \\xhookleftarrow{abc}  \\xhookrightarrow{abc}  \\xmapsto{abc}  \\xrightharpoondown{abc}  \\xrightharpoonup{abc}  \\xleftharpoondown{abc}  \\xleftharpoonup{abc} \\xrightleftharpoons{abc}  \\xleftrightharpoons{abc}  \\xlongequal{abc} \\xtwoheadrightarrow{abc}  \\xtwoheadleftarrow{abc}  \\xtofrom{abc} \\xrightleftarrows{abc}  \\xrightequilibrium{abc}  \\xleftequilibrium{abc}"
    // "\\\\cdrightarrow{abc}  \\\\cdleftarrow{abc}  \\\\cdlongequal{abc}" // untested
    // "F=ma \\\\ hahaha"
    // "\\cancel{5}"
    // r"\frac{1}{2}"
    // r"\overbrace{AB} \underbrace{AB}"
    // r"\href{https://www.dashuai009.icu}{dashuai009} \url{https:www.dashuai009.icu} \textbf{Ab0} \textit{Ab0} \textrm{Ab0} \textup{Ab0} \textnormal{Ab0} \text{Ab0} \textmd{Ab0} \textsf{Ab0}"
    // r"\hbox{a}"
    // r"\htmlId{bar}{x} \htmlClass{foo}{x} \htmlStyle{color: red;}{x} \htmlData{foo=a, bar=b}{x}"
    // r"\sqrt{a^2+b^2} = 1"
    // r"\includegraphics[height=0.8em, totalheight=100px, width=150px, alt=KA logo]{https://katex.org/img/khan-academy.png}"
    // r"{=}\mathllap{/} \mathrlap{/}{=} \mathclap{1\le i\le j\le n}"
    // r"\mathchoice{D}{T}{S}{SS}"
    // r"\sum_{i}",
    // r"\coprod^a",
    //r"\overline{A}"
    //r"b\phantom{content}a\hphantom{content}c\vphantom{content}d"
    // r"\pmb{\mu} \mu"
    // r"a\raisebox{0.25em}{b}c"
    //r" x^{\smash[a]{2}} "
    //r"\underline{AB}",
    //r"\mathrm{Ab0} \mathbf{Ab0} \mathit{Ab0}",
    //r"\mathnormal{Ab0} \mathbb{Ab} \mathcal{Ab} \mathfrak{Ab0} \mathscr{Ab} \mathsf{Ab0} \Bbb{Ab} \bold{Ab0} \frak{Ab0}",
    //r"\boldsymbol{Ab} \bm{Ab0} \rm{a} \sf{A} \tt{a} \bf{aB0} \it{Ab0} \cal{Ab0}",
    r"\big(\big) \Big(\Big)"
];



#[cfg(test)]
mod tests {
    use crate::katex::{render_to_string, TEST_CASE};
    use crate::settings::Settings;

    #[test]
    fn test_parse_tree() {
        let mut settings = Settings::new();
        settings.set_display_mode(true);
        settings.set_error_color("#cc0000".to_string());
        settings.set_trust(true);

        settings.set_max_expand(Some(1000));
        settings.set_max_size(Some(200000.0));
        println!("setting = {:#?}", settings);
        for test_string in TEST_CASE{
            println!("{} {}",test_string, render_to_string(test_string.to_string(), settings.clone()).as_str());
        }
    }
}


/*****
具有纪念意义的一行输出
<span class="katex-display"><span class="katex"><span class="katex-html" aria-hidden=true><span class="base"><span class="mspace" style="margin-right:0.2778em;"></span><span class="mord mathnormal" style="margin-right:0.05764em;">E</span><span class="mspace" style="margin-right:0.2778em;"></span><span class="mrel">=</span></span><span class="base"><span class="mord mathnormal">m</span><span class="mord"><span class="mord mathnormal">c</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span class="vlist" style="height:0.8641em;"><span style="margin-right:0.0500em;top:-3.1130em;"><span class="pstrut" style="height:2.7000em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord   mtight">2</span></span></span></span></span></span></span></span></span></span></span></span>
 */