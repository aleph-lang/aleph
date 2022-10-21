use crate::syntax::AlephTree as at;
pub mod parser;
pub mod transform;
pub mod gen;

pub fn transform_dispatcher(transformer_list: Vec<String>, ast: at) ->at{
    transformer_list.iter().fold(ast, |accum, transformer_name|
        {
            transform::transform(transformer_name.to_string(), accum)
        })
}
