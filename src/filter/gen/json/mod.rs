use crate::syntax::AlephTree as at;
use crate::filter::gen::Gen;

#[derive(Default)]
pub struct JsonGen;

impl Gen for JsonGen {
    fn generate(&self, ast: at) -> String {
        serde_json::to_string(&ast).unwrap()
    }
}
