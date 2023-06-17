use aleph_syntax_tree::syntax::AlephTree as at;

/// Generator
/// #Arguments
/// `to` - Name of generator, actually name of language (ale, py, ...)
/// `ast` - ast source to translate
///
/// # Return
/// This function return generated source code
pub fn generate(to: String, ast: at) -> String {
    match to.as_str() {
        #[cfg(feature="ale_gen")]
        "aleph" | "ale" => alegen::generate(ast),
        #[cfg(feature="json_gen")]
        "json" => aleph_syntax_tree::syntax::to_json(ast),
        #[cfg(feature="python_gen")]
        "python" | "py" => pythongen::generate(ast),
        _ => "Generator Not Implemented: Use external binary as generator".to_string()
    }
}

