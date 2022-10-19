use crate::syntax;
use syntax::AlephTree as at;

#[cfg(feature="ale_gen")]
pub mod ale;


fn to_gen(s:String) -> Option<impl Gen> {
    match s.as_str() {
        #[cfg(feature="ale_gen")]
        "ale" => Some(ale::AleGen{}),

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

/**
* this trait should be implemented by all generators
*/
pub trait Gen {
    fn generate(&self, ast: at) -> String ;
}
