use serde_json::Value;
use std::collections::BTreeMap;

const NON_SEMANTIC_FIELDS: &[&str] = &["loc"];

pub fn canonicalize(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted = BTreeMap::new();
            for (key, val) in map {
                if NON_SEMANTIC_FIELDS.contains(&key.as_str()) {
                    continue;
                }
                sorted.insert(key.clone(), canonicalize(val));
            }
            Value::Object(sorted.into_iter().collect())
        }
        Value::Array(arr) => Value::Array(arr.iter().map(canonicalize).collect()),
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                let stabilized = (f * 1e8).round() / 1e8;
                if stabilized == stabilized.floor() && stabilized.abs() < i64::MAX as f64 {
                    Value::Number(serde_json::Number::from(stabilized as i64))
                } else {
                    serde_json::to_value(stabilized).unwrap_or_else(|_| value.clone())
                }
            } else {
                value.clone()
            }
        }
        _ => value.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn strips_loc_field() {
        let input = json!({"type": "mathord", "mode": "math", "loc": {"start": 0, "end": 1}, "text": "x"});
        let result = canonicalize(&input);
        assert_eq!(result, json!({"mode": "math", "text": "x", "type": "mathord"}));
    }

    #[test]
    fn sorts_keys() {
        let input = json!({"z": 1, "a": 2, "m": 3});
        let result = canonicalize(&input);
        let keys: Vec<&String> = result.as_object().unwrap().keys().collect();
        assert_eq!(keys, vec!["a", "m", "z"]);
    }

    #[test]
    fn stabilizes_floats() {
        let input = json!(0.30000000000000004);
        let result = canonicalize(&input);
        assert_eq!(result, json!(0.3));
    }

    #[test]
    fn integer_floats_become_integers() {
        let input = json!(2.0);
        let result = canonicalize(&input);
        assert_eq!(result, json!(2));
    }

    #[test]
    fn recursively_processes_arrays_and_objects() {
        let input = json!([{"loc": {}, "type": "a", "body": [{"loc": {}, "text": "b"}]}]);
        let result = canonicalize(&input);
        assert_eq!(result, json!([{"body": [{"text": "b"}], "type": "a"}]));
    }
}
