pub mod parser;
pub mod transform;
pub mod gen;

use aleph_syntax_tree::syntax::AlephTree as at;
use aleph_syntax_tree::syntax::json_parse;

/// Generate
/// #Arguments
/// `content_type` - Name of content type
/// `content` - Source code
/// `transformer_list` - List of transoformer function
/// `return_type` - Name of ouput language
///
/// # Return
/// This function return generated source code
pub fn generate(content_type: String, content: String, transformer_list : Option<Vec<String>>, return_type: String) -> String {

    let parsed_content: at = match content_type.as_str() {
        "ast_json" => json_parse(content),
        _ => parser::parse(content_type.clone(), content),
    };

    let transformed_content: at = match transformer_list {
        Some(list)=> transform_dispatcher(list, parsed_content),
        None => parsed_content
    };
    
    return gen::generate(return_type, transformed_content);
}

/// Generator
/// #Arguments
/// `transformer_list` - List of transoformer function
/// `ast` - ast source to transform
///
/// # Return
/// This function return an Alephtree
pub fn transform_dispatcher(transformer_list: Vec<String>, ast: at) ->at{
    transformer_list.iter().fold(ast, |accum, transformer_name|
        {
            transform::transform(transformer_name.to_string(), accum)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trip test for the cognitive layer.
    /// parse(source) → alegen → parse → alegen must be idempotent.
    #[cfg(all(feature = "ale_parse", feature = "ale_gen"))]
    #[test]
    fn cognitive_roundtrip() {
        let source = include_str!(
            "../../test/dataset/ale/testCognitive.ale"
        ).to_string();

        // First pass: Aleph → AST → Aleph
        let pass1 = generate(
            "ale".to_string(),
            source,
            None,
            "ale".to_string(),
        );
        assert!(!pass1.is_empty(), "first pass produced empty output");
        assert!(
            !pass1.contains("Can't parse"),
            "first pass produced a parse error:\n{}", pass1
        );

        // Second pass: idempotency check
        let pass2 = generate(
            "ale".to_string(),
            pass1.clone(),
            None,
            "ale".to_string(),
        );
        assert_eq!(pass1, pass2, "cognitive round-trip is not idempotent");
    }
}
