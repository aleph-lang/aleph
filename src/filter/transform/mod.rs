use aleph_syntax_tree::syntax::AlephTree as at;
use aleph_syntax_tree::Transform;

mod betareduction;

fn select_transform(s:String) -> Option<Box<dyn Transform>> {
    match s.as_str() {
        "betareduction" => Some(Box::new(betareduction::BetaReduction)),
        _ => None
    }
}

/// Transform
/// #Arguments
/// `transformer_name` - Name of transform function
/// `ast` - AlephTree to transform
///
/// # Return
/// This function return an AlephTree
pub fn transform(transformer_name: String, ast: at) -> at {
    match select_transform(transformer_name) {
        Some(t) => t.transform(ast),
        None => ast,
    }
}

