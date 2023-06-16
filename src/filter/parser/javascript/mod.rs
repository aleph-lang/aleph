use aleph_syntax_tree::syntax::AlephTree as at;
use aleph_syntax_tree::Parser;

use js_parser;

#[derive(Default)]
pub struct JavascriptParser;

impl Parser for JavascriptParser {
    fn parse(&self, source: String) -> at {
    	js_parser::parse(source)
    }
}
