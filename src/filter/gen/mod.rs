use crate::filter::at;

#[cfg(feature="ale_gen")]
mod ale;
#[cfg(feature="json_gen")]
mod json;
#[cfg(feature="python_gen")]
mod python;

fn to_gen(s:String) -> Option<Box<dyn Gen>> {
    match s.as_str() {
        #[cfg(feature="ale_gen")]
        "aleph" | "ale" => Some(Box::new(ale::AleGen{})),
        #[cfg(feature="json_gen")]
        "json" => Some(Box::new(json::JsonGen{})),
        #[cfg(feature="python_gen")]
        "python" | "py" => Some(Box::new(python::PythonGen{})),
        _ => None
    }
}

/// Generator
/// #Arguments
/// `to` - Name of generator, actually name of language (ale, py, ...)
/// `ast` - ast source to translate
///
/// # Return
/// This function return generated source code
pub fn generate(to: String, ast: at) -> String {
    match to_gen(to) {
        Some(g) => g.generate(ast),
        None => "Generator Not Implemented: Use external binary as generator".to_string()
    }
}

/**
* this trait should be implemented by all generators
*/
trait Gen {
    fn generate(&self, ast: at) -> String ;
}
