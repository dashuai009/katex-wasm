use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::panic;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct CaseResult {
    pub expression: String,
    pub js_output: String,
    pub rust_output: String,
}

pub fn get_js_parse_tree(expression: &str, project_root: &str) -> String {
    let script_path = format!("{}/diff_harness/scripts/js_parse_tree.js", project_root);
    let child = Command::new("node")
        .arg(&script_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match child {
        Ok(c) => c,
        Err(e) => return format!("JS_ERROR: failed to spawn node: {}", e),
    };

    let payload = serde_json::json!({"expression": expression}).to_string();
    if let Some(mut stdin) = child.stdin.take() {
        if let Err(e) = stdin.write_all(payload.as_bytes()) {
            return format!("JS_ERROR: stdin write failed: {}", e);
        }
    }

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => return format!("JS_ERROR: wait failed: {}", e),
    };

    if !output.status.success() {
        return format!(
            "JS_ERROR: node exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn get_js_html(expression: &str, project_root: &str) -> String {
    let script_path = format!("{}/diff_harness/scripts/js_render_html.js", project_root);
    let child = Command::new("node")
        .arg(&script_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match child {
        Ok(c) => c,
        Err(e) => return format!("JS_ERROR: failed to spawn node: {}", e),
    };

    let payload = serde_json::json!({"expression": expression}).to_string();
    if let Some(mut stdin) = child.stdin.take() {
        if let Err(e) = stdin.write_all(payload.as_bytes()) {
            return format!("JS_ERROR: stdin write failed: {}", e);
        }
    }

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => return format!("JS_ERROR: wait failed: {}", e),
    };

    if !output.status.success() {
        return format!(
            "JS_ERROR: node exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn get_rust_html(expression: &str) -> String {
    let expr = expression.to_string();
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let mut settings = katex_wasm::settings::Settings::new();
        settings.set_display_mode(true);
        settings.set_trust(true);
        settings.set_max_expand(Some(1000));
        settings.set_max_size(Some(200000.0));
        katex_wasm::render_to_string(expr, settings)
    }));

    match result {
        Ok(html) => html,
        Err(err) => {
            let msg = if let Some(s) = err.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = err.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "unknown panic".to_string()
            };
            format!("RUST_PANIC: {}", msg)
        }
    }
}

pub fn get_rust_parse_tree(expression: &str) -> String {
    let expr = expression.to_string();
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let mut settings = katex_wasm::settings::Settings::new();
        settings.set_display_mode(true);
        settings.set_trust(true);
        settings.set_max_expand(Some(1000));
        settings.set_max_size(Some(200000.0));
        let tree = katex_wasm::parseTree(expr, settings);
        format!("{:#?}", tree)
    }));

    match result {
        Ok(debug_output) => debug_output,
        Err(err) => {
            let msg = if let Some(s) = err.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = err.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "unknown panic".to_string()
            };
            format!("RUST_PANIC: {}", msg)
        }
    }
}

pub fn load_fixtures(project_root: &str) -> Vec<String> {
    let fixture_path = format!("{}/tests/fixtures/formulas.txt", project_root);
    let content = fs::read_to_string(&fixture_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", fixture_path, e));

    let mut seen = BTreeSet::new();
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter(|line| seen.insert(line.to_string()))
        .map(String::from)
        .collect()
}

pub fn run_harness(expressions: &[String], project_root: &str) -> Vec<CaseResult> {
    let mut results = Vec::new();

    for expression in expressions {
        eprintln!("[diff_harness] testing: {}", expression);

        let js_start = std::time::Instant::now();
        let js_output = get_js_parse_tree(expression, project_root);
        eprintln!("[diff_harness]   get_js_parse_tree: {:.2?}", js_start.elapsed());

        let rust_start = std::time::Instant::now();
        let rust_output = get_rust_parse_tree(expression);
        eprintln!("[diff_harness]   get_rust_parse_tree: {:.2?}", rust_start.elapsed());

        results.push(CaseResult {
            expression: expression.clone(),
            js_output,
            rust_output,
        });
    }

    results
}
