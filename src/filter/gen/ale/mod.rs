use crate::filter::at;
use crate::filter::gen::Gen;

use alegen;

#[derive(Default)]
pub struct AleGen;

impl Gen for AleGen {
    fn generate(&self, ast: at) -> String {
        alegen::generate(ast)
    }
}
