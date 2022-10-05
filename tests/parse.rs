#[cfg(test)]
mod tests {
    use katex_wasm::parse::parseTree;
    use katex_wasm::settings::Settings;

    #[test]
    fn test_parse_tree() {
        let settings = Settings::new();
        let test_string = "c=ma^2".to_string();
        let tree = parseTree(test_string, settings);
    }
}
