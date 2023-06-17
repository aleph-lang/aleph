use aleph_syntax_tree::syntax::AlephTree as at;

/// Transform
/// #Arguments
/// `transformer_name` - Name of transform function
/// `ast` - AlephTree to transform
///
/// # Return
/// This function return an AlephTree
pub fn transform(transformer_name: String, ast: at) -> at {
    match transformer_name.as_str() {
        "betareduction" => betareduction::transform(ast),
        _ => ast,
    }
}

