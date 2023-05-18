use aleph_syntax_tree::syntax::AlephTree as at;

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

/// this trait should be implemented by all transformers
trait Transform {
    fn transform(&self, ast: at) -> at ;
}
