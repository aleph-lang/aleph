use aleph_syntax_tree::syntax::AlephTree as at;
use crate::filter::transform::Transform;

#[derive(Default)]
pub struct Ident;

impl Transform for Ident {
    fn transform(&self, ast: at) -> at {
        ast
    }
}
