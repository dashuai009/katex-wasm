#[cfg(test)]
mod tests {
    use katex_wasm::settings::Settings;
    use katex_wasm::parse::parseTree;

    #[test]
    fn test_parse_tree() {
        let settings = Settings::new_rust();
        let test_string = "c=ma^2".to_string();
        let tree = parseTree(test_string,settings);

    }
}
