use crate::syntax;
use syntax::AlephTree as at;

#[cfg(feature="ale_gen")]
pub mod ale;

/**
* this trait should be implemented by all generators
*/
pub trait Gen {
    fn generate(&self, ast: at) -> String ;
}
