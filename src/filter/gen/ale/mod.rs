use aleph_syntax_tree::syntax::AlephTree as at;
use aleph_syntax_tree::Gen;

use alegen;

#[derive(Default)]
pub struct AleGen;

impl Gen for AleGen {
    fn generate(&self, ast: at) -> String {
        alegen::generate(ast)
    }
}
