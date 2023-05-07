use crate::filter::at;
use crate::filter::parser::Parser;

use js_parser;

#[derive(Default)]
pub struct JavascriptParser;

impl Parser for JavascriptParser {
    fn parse(&self, source: String) -> at {
    	js_parser::parse(source)
    }
}
