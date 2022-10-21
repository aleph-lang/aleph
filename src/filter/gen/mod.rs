use crate::syntax;
use syntax::AlephTree as at;

#[cfg(feature="ale_gen")]
mod ale;


fn to_gen(s:String) -> Option<Box<dyn Gen>> {
    match s.as_str() {
        #[cfg(feature="ale_gen")]
        "ale" => Some(Box::new(ale::AleGen{})),

        _ => None
    }
}

// select right generator and generate the code from the ast
pub fn generate(to: String, ast: at) -> String {
    match to_gen(to) {
        Some(g) => g.generate(ast),
        None => "Not Implemented: Use external binary as generator".to_string()
    }
}

// indentation for generators
fn comp_indent_sep(indent: i64, sep: String) -> String {
    let mut res = "".to_string();
    for _ in 0..indent {
        res.push_str(&sep);
    }
    res
}

// use comp_indent_sep with tab
fn comp_indent(indent: i64) -> String {
   comp_indent_sep(indent, String::from("\t"))
}

/**
* this trait should be implemented by all generators
*/
trait Gen {
    fn generate(&self, ast: at) -> String ;
}
