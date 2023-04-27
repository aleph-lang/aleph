use crate::filter::at;
use crate::filter::parser::Parser;

use python_parser;

#[derive(Default)]
pub struct PythonParser;

impl Parser for PythonParser {
    fn parse(&self, source: String) -> at {
    	python_parser::python_parse(source)
    }
}
