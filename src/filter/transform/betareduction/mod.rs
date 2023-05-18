use crate::filter::transform::Transform;
use aleph_syntax_tree::syntax::AlephTree as at;

use betareduction;

#[derive(Default)]
pub struct BetaReduction;

impl Transform for BetaReduction{
    fn transform(&self, ast: at) -> at{
         betareduction::transform(ast)
    }
}

