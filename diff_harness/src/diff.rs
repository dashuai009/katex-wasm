use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AstDiff {
    pub path: String,
    pub node_type: Option<String>,
    pub description: String,
    pub expected: Option<String>,
    pub actual: Option<String>,
}

impl std::fmt::Display for AstDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "path={}", self.path)?;
        if let Some(ref node_type) = self.node_type {
            write!(f, " node_type={}", node_type)?;
        }
        write!(f, " {}", self.description)?;
        if let Some(ref expected) = self.expected {
            write!(f, "\n  expected: {}", expected)?;
        }
        if let Some(ref actual) = self.actual {
            write!(f, "\n  actual:   {}", actual)?;
        }
        Ok(())
    }
}

pub fn diff_ast(expected: &Value, actual: &Value) -> Vec<AstDiff> {
    let mut diffs = Vec::new();
    diff_recursive(expected, actual, "$".to_string(), &mut diffs);
    diffs
}

fn diff_recursive(expected: &Value, actual: &Value, path: String, diffs: &mut Vec<AstDiff>) {
    match (expected, actual) {
        (Value::Object(expected_map), Value::Object(actual_map)) => {
            let node_type = expected.as_object()
                .and_then(|o| o.get("type"))
                .and_then(|v| v.as_str())
                .map(String::from);

            let mut all_keys: Vec<&String> = expected_map.keys().chain(actual_map.keys()).collect();
            all_keys.sort();
            all_keys.dedup();

            for key in all_keys {
                let child_path = format!("{}.{}", path, key);
                match (expected_map.get(key), actual_map.get(key)) {
                    (Some(expected_val), Some(actual_val)) => {
                        diff_recursive(expected_val, actual_val, child_path, diffs);
                    }
                    (Some(expected_val), None) => {
                        diffs.push(AstDiff {
                            path: child_path,
                            node_type: node_type.clone(),
                            description: "missing field in actual".to_string(),
                            expected: Some(truncate_json(expected_val)),
                            actual: None,
                        });
                    }
                    (None, Some(actual_val)) => {
                        diffs.push(AstDiff {
                            path: child_path,
                            node_type: node_type.clone(),
                            description: "extra field in actual".to_string(),
                            expected: None,
                            actual: Some(truncate_json(actual_val)),
                        });
                    }
                    (None, None) => unreachable!(),
                }
            }
        }
        (Value::Array(expected_arr), Value::Array(actual_arr)) => {
            if expected_arr.len() != actual_arr.len() {
                diffs.push(AstDiff {
                    path: path.clone(),
                    node_type: None,
                    description: format!(
                        "array length mismatch: expected {} vs actual {}",
                        expected_arr.len(),
                        actual_arr.len()
                    ),
                    expected: Some(format!("length {}", expected_arr.len())),
                    actual: Some(format!("length {}", actual_arr.len())),
                });
            }
            let min_len = expected_arr.len().min(actual_arr.len());
            for i in 0..min_len {
                let child_path = format!("{}[{}]", path, i);
                diff_recursive(&expected_arr[i], &actual_arr[i], child_path, diffs);
            }
        }
        _ => {
            if expected != actual {
                diffs.push(AstDiff {
                    path,
                    node_type: None,
                    description: "value mismatch".to_string(),
                    expected: Some(truncate_json(expected)),
                    actual: Some(truncate_json(actual)),
                });
            }
        }
    }
}

fn truncate_json(value: &Value) -> String {
    let serialized = serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value));
    if serialized.len() > 200 {
        format!("{}...", &serialized[..200])
    } else {
        serialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn identical_values_produce_no_diffs() {
        let value = json!({"type": "mathord", "mode": "math", "text": "x"});
        let diffs = diff_ast(&value, &value);
        assert!(diffs.is_empty());
    }

    #[test]
    fn detects_value_mismatch() {
        let expected = json!({"type": "mathord", "text": "x"});
        let actual = json!({"type": "mathord", "text": "y"});
        let diffs = diff_ast(&expected, &actual);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].path, "$.text");
    }

    #[test]
    fn detects_missing_field() {
        let expected = json!({"type": "mathord", "text": "x"});
        let actual = json!({"type": "mathord"});
        let diffs = diff_ast(&expected, &actual);
        assert_eq!(diffs.len(), 1);
        assert!(diffs[0].description.contains("missing"));
    }

    #[test]
    fn detects_array_length_mismatch() {
        let expected = json!([1, 2, 3]);
        let actual = json!([1, 2]);
        let diffs = diff_ast(&expected, &actual);
        assert!(!diffs.is_empty());
        assert!(diffs[0].description.contains("length"));
    }

    #[test]
    fn reports_node_type_in_diff() {
        let expected = json!({"type": "supsub", "base": {"type": "mathord", "text": "x"}});
        let actual = json!({"type": "supsub", "base": {"type": "mathord", "text": "y"}});
        let diffs = diff_ast(&expected, &actual);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].path, "$.base.text");
    }
}
