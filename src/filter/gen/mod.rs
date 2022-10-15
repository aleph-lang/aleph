use crate::syntax;
use syntax::AlephTree as at;

pub mod ale;

/**
* this trait should be implemented by all generators
*/
pub trait Gen {
    fn generate(ast: at) -> String;
}
