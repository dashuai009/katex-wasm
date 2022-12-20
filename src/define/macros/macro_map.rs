use super::public::MacroDefinition;
use crate::token::Token;

fn new_me(tokens: Vec<Token>, num_args: i32) -> MacroDefinition {
    MacroDefinition::MacroExpansion(super::public::MacroExpansion {
        tokens,
        num_args,
        delimiters: None,
        unexpandable: false,
    })
}


pub fn create_macro_map() -> std::collections::HashMap<String, MacroDefinition> {
    let mut res = std::collections::HashMap::from([
        ("\\noexpand".to_string(), MacroDefinition::MacroContext(|context| {
            // The expansion is the token itself; but that token is interpreted
            // as if its meaning were ‘\relax’ if it is a control sequence that
            // would ordinarily be expanded by TeX’s expansion rules.
            let mut t = context.pop_token();
            if (context.is_expandable(&t.text)) {
                t.noexpand = true;
                t.treatAsRelax = true;
            }
            return new_me(vec![t], 0);
        })),
    ]);

    res
}
