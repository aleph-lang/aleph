use aleph_syntax_tree::syntax::AlephTree as at;
use crate::filter::parser::Parser;

#[derive(Default)]
pub struct JsonParser;

impl Parser for JsonParser {
    fn parse(&self, source: String) -> at {
       serde_json::from_str(&source).unwrap()
    }
}
