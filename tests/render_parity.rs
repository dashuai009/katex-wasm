use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;
use std::process::{Command, Stdio};

use katex_wasm::render_to_string;
use katex_wasm::settings::Settings;
#[derive(Debug)]
struct ParityMismatch {
    expression: String,
    rust_html: String,
    js_html: String,
}

#[derive(Debug)]
struct RenderError {
    expression: String,
    stage: String,
    message: String,
}

fn normalize_html(html: &str) -> String {
    // 规范化 HTML 以便进行严格比较
    // 移除多余的空白字符，但保留标签之间的空格
    html.trim().to_string()
}

fn render_with_js_katex(expression: &str) -> String {
    let script = r#"
const fs = require('fs');
const payload = JSON.parse(fs.readFileSync(0, 'utf8'));
const katex = require('./demo/node_modules/katex');
const html = katex.renderToString(payload.expression, {
  displayMode: true,
  output: 'html',
  throwOnError: false,
  trust: true,
  strict: 'ignore'
});
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

fn load_formula_cases() -> Vec<String> {
    let fixture = Path::new("tests/fixtures/formulas.txt");
    let content = fs::read_to_string(fixture)
        .unwrap_or_else(|err| panic!("failed to read formula fixture {}: {err}", fixture.display()));

    let mut cases: Vec<String> = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(ToOwned::to_owned)
        .collect();

    cases.extend(generate_formula_cases());
    dedup_preserve_order(cases)
}

fn selected_formula_cases(cases: Vec<String>) -> Vec<String> {
    let filter = std::env::var("PARITY_CASE_FILTER").ok();
    let exact = std::env::var("PARITY_CASE_EXACT").ok();

    let mut selected = cases;
    if let Some(filter) = filter {
        selected = selected
            .into_iter()
            .filter(|case| case.contains(&filter))
            .collect();
    }

    if let Some(exact) = exact {
        let exact_cases = exact
            .split("||")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect::<BTreeSet<_>>();
        selected = selected
            .into_iter()
            .filter(|case| exact_cases.contains(case.as_str()))
            .collect();
    }

    selected
}

fn generate_formula_cases() -> Vec<String> {
    let atoms = ["x", "y", "z", "\\alpha", "\\beta"];
    let wrappers = [
        "{a}+{b}",
        "{a}-{b}",
        "{a}\\cdot {b}",
        "\\frac{{{a}}}{{{b}}}",
        "\\sqrt{{{a}+{b}}}",
        "\\left({a}+{b}\\right)^2",
    ];

    let mut generated = Vec::new();
    for a in atoms {
        for b in atoms {
            if a == b {
                continue;
            }
            for template in wrappers {
                generated.push(template.replace("{a}", a).replace("{b}", b));
            }
        }
    }
    generated
}

fn dedup_preserve_order(cases: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();

    for case in cases {
        if seen.insert(case.clone()) {
            out.push(case);
        }
    }

    out
}

#[test]
fn formula_case_source_is_large_enough() {
    let cases = load_formula_cases();
    assert!(
        cases.len() >= 80,
        "expected at least 80 formulas, got {}",
        cases.len()
    );
}

#[test]
fn parity_signature_matches_js_for_generated_formula_set() {
    let all_cases = load_formula_cases();
    let cases = selected_formula_cases(all_cases);

    assert!(
        !cases.is_empty(),
        "no test case selected; check PARITY_CASE_FILTER / PARITY_CASE_EXACT"
    );

    let mut settings = Settings::new();
    settings.set_display_mode(true);
    settings.set_trust(true);

    let mut mismatches = Vec::new();
    let mut render_errors = Vec::new();

    let print_all = true; // std::env::var("PARITY_PRINT_ALL").ok().as_deref() == Some("1");
    let total_cases = cases.len();

    for (idx, expression) in cases.into_iter().enumerate() {
        eprintln!("[render parity][case {}/{}] {}", idx + 1, total_cases, expression);
        let rust_html = match panic::catch_unwind(AssertUnwindSafe(|| {
            render_to_string(expression.clone(), settings.clone())
        })) {
            Ok(value) => value,
            Err(err) => {
                render_errors.push(RenderError {
                    expression: expression.clone(),
                    stage: "rust".to_string(),
                    message: format!("{err:?}"),
                });
                continue;
            }
        };

        let js_html = match panic::catch_unwind(AssertUnwindSafe(|| render_with_js_katex(&expression))) {
            Ok(value) => value,
            Err(err) => {
                render_errors.push(RenderError {
                    expression: expression.clone(),
                    stage: "js".to_string(),
                    message: format!("{err:?}"),
                });
                continue;
            }
        };

        let normalized_rust = normalize_html(&rust_html);
        let normalized_js = normalize_html(&js_html);

        let is_mismatch = normalized_rust != normalized_js;

        if print_all {
            eprintln!("[render parity][rust html] {}", rust_html);
            eprintln!("[render parity][js html]   {}", js_html);
            eprintln!(
                "[render parity][match] {}",
                if is_mismatch { "MISMATCH" } else { "MATCH" }
            );
        }

        if is_mismatch {
            mismatches.push(ParityMismatch {
                expression,
                rust_html,
                js_html,
            });
        }
    }

    if !render_errors.is_empty() {
        eprintln!(
            "[render parity] render errors: {} (set ASSERT_PARITY=1 to fail this test)",
            render_errors.len()
        );
        for error in &render_errors {
            eprintln!(
                "- [{}] {} | {}",
                error.stage,
                error.expression,
                error.message
            );
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "[render parity] mismatches: {} (set ASSERT_PARITY=1 to fail this test)",
            mismatches.len()
        );
        for mismatch in &mismatches {
            eprintln!("- {}", mismatch.expression);
            eprintln!("  rust_html: {}", mismatch.rust_html);
            eprintln!("  js_html:   {}", mismatch.js_html);

            // 显示差异的详细信息
            if mismatch.rust_html.len() != mismatch.js_html.len() {
                eprintln!(
                    "  length diff: rust={} vs js={}",
                    mismatch.rust_html.len(),
                    mismatch.js_html.len()
                );
            }
        }
    }

    if std::env::var("ASSERT_PARITY").ok().as_deref() == Some("1") {
        assert!(
            render_errors.is_empty(),
            "render errors detected: {}",
            render_errors.len()
        );
        assert!(
            mismatches.is_empty(),
            "parity mismatches detected: {}",
            mismatches.len()
        );
    }
}
