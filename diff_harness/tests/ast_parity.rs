use diff_harness::harness;

fn project_root() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(manifest_dir)
        .parent()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

#[test]
fn canonical_json_round_trip() {
    use diff_harness::canonical::canonicalize;
    use serde_json::json;

    let input = json!({
        "type": "mathord",
        "loc": {"start": 0, "end": 1, "lexer": {}},
        "mode": "math",
        "text": "x"
    });
    let result = canonicalize(&input);
    assert!(!result.as_object().unwrap().contains_key("loc"));
    let keys: Vec<&String> = result.as_object().unwrap().keys().collect();
    assert_eq!(keys, vec!["mode", "text", "type"]);
}

#[test]
fn diff_identical_trees_is_empty() {
    use diff_harness::diff::diff_ast;
    use serde_json::json;

    let tree = json!([{"type": "mathord", "mode": "math", "text": "x"}]);
    let diffs = diff_ast(&tree, &tree);
    assert!(diffs.is_empty());
}

#[test]
fn diff_detects_text_mismatch() {
    use diff_harness::diff::diff_ast;
    use serde_json::json;

    let expected = json!([{"type": "mathord", "mode": "math", "text": "x"}]);
    let actual = json!([{"type": "mathord", "mode": "math", "text": "y"}]);
    let diffs = diff_ast(&expected, &actual);
    assert_eq!(diffs.len(), 1);
    assert!(diffs[0].path.contains("text"));
}

#[test]
fn tokenizer_handles_latex_commands() {
    use diff_harness::minimize::tokenize_latex;
    let tokens = tokenize_latex(r"\frac{a}{b}");
    assert!(tokens.len() >= 3);
    assert_eq!(tokens[0], r"\frac");
}

#[test]
fn debug_parser_handles_simple_ast() {
    use diff_harness::debug_parser::parse_rust_debug;

    let debug_output = r#"[mathord { mode: math, loc: None, text: "x" }]"#;
    let result = parse_rust_debug(debug_output);
    assert!(result.is_array());
    let first = &result[0];
    assert_eq!(first["type"], "mathord");
    assert_eq!(first["text"], "x");
}

#[test]
fn js_parse_tree_script_exists() {
    let root = project_root();
    let script_path = format!("{}/diff_harness/scripts/js_parse_tree.js", root);
    assert!(
        std::path::Path::new(&script_path).exists(),
        "JS parse tree script not found at {}",
        script_path
    );
}

#[test]
fn fixtures_file_has_cases() {
    let root = project_root();
    let cases = harness::load_fixtures(&root);
    assert!(
        cases.len() >= 10,
        "expected at least 10 fixture cases, got {}",
        cases.len()
    );
}

#[test]
fn rust_parse_tree_returns_string() {
    let output = harness::get_rust_parse_tree("x");
    assert!(!output.is_empty());
    assert!(!output.starts_with("RUST_PANIC"));
}
