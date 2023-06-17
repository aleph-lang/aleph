use aleph_syntax_tree::syntax::AlephTree as at;

/// Parser
/// select right generator and generate the code from the ast
/// #Arguments
/// `from` - Name of parser, actually language name (ale, py, ...)
/// `source` - String to parse
///
/// # Return
/// This function return an AlephTree
pub fn parse(from: String, source: String) -> at {
    match from.as_str() {
        #[cfg(feature="ale_parse")]
        "ale" => aleparser::parse(source),
        #[cfg(feature="js_parse")]
        "javascript" | "js" => js_parser::parse(source),
        #[cfg(feature="json_parse")]
        "json" => aleph_syntax_tree::syntax::json_parse(source),
        #[cfg(feature="python_parse")]
        "python" | "py" => python_parser::python_parse(source),
        _ => at::String{value : "Parser Not Implemented".to_string()}
    }
}
