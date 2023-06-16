use aleph_syntax_tree::syntax::AlephTree as at;
use aleph_syntax_tree::Gen;

#[derive(Default)]
pub struct JsonGen;

impl Gen for JsonGen {
    fn generate(&self, ast: at) -> String {
        serde_json::to_string_pretty(&ast).unwrap()
    }
}
