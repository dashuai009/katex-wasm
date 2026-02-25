use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::Value;

use crate::canonical::canonicalize;

pub struct MinimizeResult {
    pub original: String,
    pub minimized: String,
    pub original_token_count: usize,
    pub minimized_token_count: usize,
}

impl std::fmt::Display for MinimizeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "minimized {} tokens → {} tokens\n  original:  {}\n  minimized: {}",
            self.original_token_count,
            self.minimized_token_count,
            self.original,
            self.minimized
        )
    }
}

pub fn tokenize_latex(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            let start = i;
            i += 1;
            while i < chars.len() && chars[i].is_ascii_alphabetic() {
                i += 1;
            }
            if i == start + 1 && i < chars.len() {
                i += 1;
            }
            tokens.push(chars[start..i].iter().collect());
        } else if chars[i].is_whitespace() {
            let start = i;
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
            tokens.push(chars[start..i].iter().collect());
        } else if chars[i] == '{' || chars[i] == '}' || chars[i] == '^' || chars[i] == '_' {
            tokens.push(chars[i..i + 1].iter().collect());
            i += 1;
        } else {
            tokens.push(chars[i..i + 1].iter().collect());
            i += 1;
        }
    }
    tokens
}

fn reassemble(tokens: &[String], skip: &[bool]) -> String {
    tokens
        .iter()
        .zip(skip.iter())
        .filter(|(_, &s)| !s)
        .map(|(t, _)| t.as_str())
        .collect::<String>()
}

fn get_js_ast(expression: &str, project_root: &str) -> Option<Value> {
    let script_path = format!("{}/diff_harness/scripts/js_parse_tree.js", project_root);
    let mut child = Command::new("node")
        .arg(&script_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;

    let payload = serde_json::json!({"expression": expression}).to_string();
    child.stdin.take()?.write_all(payload.as_bytes()).ok()?;
    let output = child.wait_with_output().ok()?;
    if !output.status.success() {
        return None;
    }
    let raw: Value = serde_json::from_slice(&output.stdout).ok()?;
    if raw.as_object().map_or(false, |o| o.contains_key("__error")) {
        return None;
    }
    Some(canonicalize(&raw))
}

fn check_still_fails(
    expression: &str,
    reference_ast: &Value,
    project_root: &str,
) -> bool {
    if expression.trim().is_empty() {
        return false;
    }
    match get_js_ast(expression, project_root) {
        Some(ast) => {
            let candidate_canonical = canonicalize(&ast);
            let diffs = crate::diff::diff_ast(reference_ast, &candidate_canonical);
            !diffs.is_empty()
        }
        None => false,
    }
}

pub fn minimize_failing_input(
    expression: &str,
    reference_ast: &Value,
    project_root: &str,
) -> MinimizeResult {
    let tokens = tokenize_latex(expression);
    let token_count = tokens.len();
    let mut skip = vec![false; token_count];

    // ddmin-style: try removing each token
    for i in 0..token_count {
        if skip[i] {
            continue;
        }
        skip[i] = true;
        let candidate = reassemble(&tokens, &skip);
        if !check_still_fails(&candidate, reference_ast, project_root) {
            skip[i] = false;
        }
    }

    // Second pass: try removing contiguous pairs
    for i in 0..token_count.saturating_sub(1) {
        if skip[i] || skip[i + 1] {
            continue;
        }
        skip[i] = true;
        skip[i + 1] = true;
        let candidate = reassemble(&tokens, &skip);
        if !check_still_fails(&candidate, reference_ast, project_root) {
            skip[i] = false;
            skip[i + 1] = false;
        }
    }

    let minimized = reassemble(&tokens, &skip);
    let minimized_token_count = skip.iter().filter(|&&s| !s).count();

    MinimizeResult {
        original: expression.to_string(),
        minimized,
        original_token_count: token_count,
        minimized_token_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple_expression() {
        let tokens = tokenize_latex(r"x^2");
        assert_eq!(tokens, vec!["x", "^", "2"]);
    }

    #[test]
    fn tokenize_command() {
        let tokens = tokenize_latex(r"\frac{a}{b}");
        assert_eq!(tokens, vec!["\\frac", "{", "a", "}", "{", "b", "}"]);
    }

    #[test]
    fn tokenize_preserves_braces() {
        let tokens = tokenize_latex(r"{x}");
        assert_eq!(tokens, vec!["{", "x", "}"]);
    }

    #[test]
    fn reassemble_with_no_skips() {
        let tokens = vec!["x".to_string(), "^".to_string(), "2".to_string()];
        let skip = vec![false, false, false];
        assert_eq!(reassemble(&tokens, &skip), "x^2");
    }

    #[test]
    fn reassemble_with_skips() {
        let tokens = vec!["x".to_string(), "^".to_string(), "2".to_string()];
        let skip = vec![false, true, true];
        assert_eq!(reassemble(&tokens, &skip), "x");
    }
}
