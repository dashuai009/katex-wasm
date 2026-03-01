use crate::{
    parse_error::ParseError,
    parse_node::types::AnyParseNode,
    settings::Settings,
    Parser::Parser,
};

// @flow
/**
 * Provides a single function for parsing an expression using a Parser
 * TODO(emily): Remove this
 */

/**
 * Parses an expression using a Parser, then returns the parsed result.
 */
pub fn parse_tree_with_error(
    to_parse: String,
    settings: Settings,
) -> Result<Vec<Box<dyn AnyParseNode>>, ParseError> {
    let mut parser = Parser::new(to_parse, &settings);

    // Blank out any \df@tag to avoid spurious "Duplicate \tag" errors
    // delete parser.gullet.macros.current["\\df@tag"];

    let tree = parser.parse();
    if let Some(error) = parser.take_error() {
        return Err(error);
    }

    // Prevent a color definition from persisting between calls to katex.render().
    // delete parser.gullet.macros.current["\\current@color"];
    // delete parser.gullet.macros.current["\\color"];

    // If the input used \tag, it will set the \df@tag macro to the tag.
    // In this case, we separately parse the tag and wrap the tree.
    // if (parser.gullet.macros.get("\\df@tag")) {
    //     if (!settings.displayMode) {
    //         throw new ParseError("\\tag works only in display equations");
    //     }
    //     tree = [{
    //         type: "tag",
    //         mode: "text",
    //         body: tree,
    //         tag: parser.subparse([new Token("\\df@tag")]),
    //     }];
    // }

    Ok(tree)
}

pub fn  parseTree(toParse: String, settings: Settings)-> Vec<Box::<dyn AnyParseNode>> { 
    parse_tree_with_error(toParse, settings)
        .unwrap_or_else(|error| panic!("ParseError: {}", error))
}
