use crate::syntax;
use syntax::AlephTree as at;

mod ale;
mod json;

fn do_parse(s:String) -> Option<Box<dyn Parser>> {
    match s.as_str() {
        "ale" => Some(Box::new(ale::AleParser{})),
        "json" => Some(Box::new(json::JsonParser{})),
        _ => None
    }
}

// select right generator and generate the code from the ast
pub fn parse(to: String, source: String) -> at {
    match do_parse(to) {
        Some(g) => g.parse(source),
        None => at::String{value : "Parser Not Implemented".to_string()}
    }
}

/**
* this trait should be implemented by all parser
*/
trait Parser {
    fn parse(&self, source: String) -> at;
}
