use serde_json::{json, Map, Value};

pub fn parse_rust_debug(input: &str) -> Value {
    let input = input.trim();
    let mut parser = DebugParser::new(input);
    parser.parse_value()
}

struct DebugParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> DebugParser<'a> {
    fn new(input: &'a str) -> Self {
        DebugParser { input, pos: 0 }
    }

    fn remaining(&self) -> &'a str {
        &self.input[self.pos..]
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.input.as_bytes()[self.pos];
            if ch == b' ' || ch == b'\n' || ch == b'\r' || ch == b'\t' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn consume_char(&mut self) -> Option<char> {
        let ch = self.input[self.pos..].chars().next()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.remaining().starts_with(prefix)
    }

    fn parse_value(&mut self) -> Value {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Value::Null;
        }

        match self.peek() {
            Some('[') => self.parse_array(),
            Some('"') => self.parse_string(),
            Some(ch) if ch.is_ascii_digit() || ch == '-' => self.parse_number(),
            _ => {
                if self.starts_with("true") {
                    self.pos += 4;
                    Value::Bool(true)
                } else if self.starts_with("false") {
                    self.pos += 5;
                    Value::Bool(false)
                } else if self.starts_with("None") {
                    self.pos += 4;
                    Value::Null
                } else if self.starts_with("Some(") {
                    self.pos += 5; // skip "Some("
                    let val = self.parse_value();
                    self.skip_whitespace();
                    if self.peek() == Some(')') {
                        self.consume_char();
                    }
                    val
                } else {
                    self.parse_struct_or_enum()
                }
            }
        }
    }

    fn parse_array(&mut self) -> Value {
        self.consume_char(); // skip '['
        let mut items = Vec::new();
        loop {
            self.skip_whitespace();
            if self.peek() == Some(']') {
                self.consume_char();
                break;
            }
            if !items.is_empty() {
                if self.peek() == Some(',') {
                    self.consume_char();
                }
            }
            self.skip_whitespace();
            if self.peek() == Some(']') {
                self.consume_char();
                break;
            }
            items.push(self.parse_value());
        }
        Value::Array(items)
    }

    fn parse_string(&mut self) -> Value {
        self.consume_char(); // skip opening '"'
        let mut result = String::new();
        loop {
            match self.consume_char() {
                Some('\\') => {
                    match self.consume_char() {
                        Some('"') => result.push('"'),
                        Some('\\') => result.push('\\'),
                        Some('n') => result.push('\n'),
                        Some('t') => result.push('\t'),
                        Some('r') => result.push('\r'),
                        Some(ch) => {
                            result.push('\\');
                            result.push(ch);
                        }
                        None => break,
                    }
                }
                Some('"') => break,
                Some(ch) => result.push(ch),
                None => break,
            }
        }
        Value::String(result)
    }

    fn parse_number(&mut self) -> Value {
        let start = self.pos;
        if self.peek() == Some('-') {
            self.consume_char();
        }
        while self.pos < self.input.len() {
            let ch = self.input.as_bytes()[self.pos];
            if ch.is_ascii_digit() || ch == b'.' || ch == b'e' || ch == b'E' || ch == b'+' || ch == b'-' {
                // Handle sign only after e/E
                if (ch == b'+' || ch == b'-') && self.pos > start {
                    let prev = self.input.as_bytes()[self.pos - 1];
                    if prev != b'e' && prev != b'E' {
                        break;
                    }
                }
                self.pos += 1;
            } else {
                break;
            }
        }
        let num_str = &self.input[start..self.pos];
        if num_str.contains('.') || num_str.contains('e') || num_str.contains('E') {
            if let Ok(f) = num_str.parse::<f64>() {
                return serde_json::to_value(f).unwrap_or(Value::Null);
            }
        }
        if let Ok(i) = num_str.parse::<i64>() {
            return Value::Number(serde_json::Number::from(i));
        }
        if let Ok(u) = num_str.parse::<u64>() {
            return Value::Number(serde_json::Number::from(u));
        }
        Value::String(num_str.to_string())
    }

    fn parse_identifier(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.input.len() {
            let ch = self.input.as_bytes()[self.pos];
            if ch.is_ascii_alphanumeric() || ch == b'_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    fn parse_struct_or_enum(&mut self) -> Value {
        let name = self.parse_identifier();
        self.skip_whitespace();

        match self.peek() {
            Some('{') => {
                // Struct with named fields
                self.consume_char(); // skip '{'
                let mut map = Map::new();
                // Add the struct name as the "type" field, converting underscores to hyphens
                // to match JS KaTeX convention
                map.insert("type".to_string(), Value::String(name.replace('_', "-")));
                loop {
                    self.skip_whitespace();
                    if self.peek() == Some('}') {
                        self.consume_char();
                        break;
                    }
                    if !map.is_empty() || map.len() > 1 {
                        if self.peek() == Some(',') {
                            self.consume_char();
                        }
                    }
                    self.skip_whitespace();
                    if self.peek() == Some('}') {
                        self.consume_char();
                        break;
                    }
                    let field_name = self.parse_identifier();
                    if field_name.is_empty() {
                        // Skip unexpected characters
                        self.consume_char();
                        continue;
                    }
                    self.skip_whitespace();
                    if self.peek() == Some(':') {
                        self.consume_char(); // skip ':'
                    }
                    self.skip_whitespace();
                    let field_value = self.parse_value();
                    map.insert(field_name, field_value);
                }
                Value::Object(map)
            }
            Some('(') => {
                // Enum variant with tuple fields
                self.consume_char(); // skip '('
                let mut items = Vec::new();
                loop {
                    self.skip_whitespace();
                    if self.peek() == Some(')') {
                        self.consume_char();
                        break;
                    }
                    if !items.is_empty() {
                        if self.peek() == Some(',') {
                            self.consume_char();
                        }
                    }
                    self.skip_whitespace();
                    if self.peek() == Some(')') {
                        self.consume_char();
                        break;
                    }
                    items.push(self.parse_value());
                }
                if items.len() == 1 {
                    // Single-value enum variant: return as tagged value
                    json!({name: items.into_iter().next().unwrap()})
                } else {
                    json!({name: items})
                }
            }
            _ => {
                // Bare identifier (enum variant without data, or a simple value)
                Value::String(name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_struct() {
        let input = r#"mathord { mode: math, text: "x" }"#;
        let result = parse_rust_debug(input);
        assert_eq!(result["type"], "mathord");
        assert_eq!(result["mode"], "math");
        assert_eq!(result["text"], "x");
    }

    #[test]
    fn parse_none_and_some() {
        let input = r#"test { a: None, b: Some("hello") }"#;
        let result = parse_rust_debug(input);
        assert_eq!(result["a"], Value::Null);
        assert_eq!(result["b"], "hello");
    }

    #[test]
    fn parse_array() {
        let input = r#"[mathord { mode: math, text: "x" }]"#;
        let result = parse_rust_debug(input);
        assert!(result.is_array());
        assert_eq!(result[0]["type"], "mathord");
    }

    #[test]
    fn parse_nested_struct() {
        let input = r#"supsub { mode: math, base: Some(mathord { mode: math, text: "x" }), sup: Some(textord { mode: math, text: "2" }), sub: None }"#;
        let result = parse_rust_debug(input);
        assert_eq!(result["type"], "supsub");
        assert_eq!(result["base"]["type"], "mathord");
        assert_eq!(result["base"]["text"], "x");
        assert_eq!(result["sup"]["type"], "textord");
        assert_eq!(result["sup"]["text"], "2");
        assert_eq!(result["sub"], Value::Null);
    }

    #[test]
    fn parse_bool_and_number() {
        let input = r#"test { flag: true, count: 42, ratio: 1.5 }"#;
        let result = parse_rust_debug(input);
        assert_eq!(result["flag"], true);
        assert_eq!(result["count"], 42);
        assert_eq!(result["ratio"], 1.5);
    }

    #[test]
    fn underscore_to_hyphen_in_type() {
        let input = r#"accent_token { mode: math, text: "x" }"#;
        let result = parse_rust_debug(input);
        assert_eq!(result["type"], "accent-token");
    }
}
