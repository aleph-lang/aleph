use crate::filter::at;
use crate::filter::parser::Parser;

use aleparser;

#[derive(Default)]
pub struct AleParser;

impl Parser for AleParser {
    fn parse(&self, source: String) -> at {
        aleparser::parse(source)
    }
}
