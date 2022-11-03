use crate::syntax;
use crate::filter::transform::Transform;
use syntax::AlephTree as at;

#[derive(Default)]
pub struct BetaReduction;


fn beta(ast: at, _env:Vec<String>) -> at {
    ast
}

impl Transform for BetaReduction{
    fn transform(&self, ast: at) -> at{
        println!("doing beta red");
        beta(ast, vec![])
    }
}

