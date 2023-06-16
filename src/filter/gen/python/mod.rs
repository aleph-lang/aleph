use aleph_syntax_tree::syntax::AlephTree as at;
use aleph_syntax_tree::Gen;

use pythongen;

#[derive(Default)]
pub struct PythonGen;

impl Gen for PythonGen {
    fn generate(&self, ast: at) -> String {
        pythongen::generate(ast)
    }
}
