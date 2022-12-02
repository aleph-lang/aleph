use crate::filter::at;
use crate::filter::transform::Transform;

#[derive(Default)]
pub struct Ident;

impl Transform for Ident {
    fn transform(&self, ast: at) -> at {
        ast
    }
}
