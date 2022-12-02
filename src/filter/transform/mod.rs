use crate::filter::at;

mod ident;

fn select_transform(s:String) -> Option<Box<dyn Transform>> {
    match s.as_str() {
        "ident" => Some(Box::new(ident::Ident)),
        _ => None
    }
}

// select right generator and generate the code from the ast
pub fn transform(transformer_name: String, ast: at) -> at {
    match select_transform(transformer_name) {
        Some(t) => t.transform(ast),
        None => ast,
    }
}

/**
* this trait should be implemented by all transformers
*/
trait Transform {
    fn transform(&self, ast: at) -> at ;
}
