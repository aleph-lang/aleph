pub mod syntax;
pub mod parser;
pub mod transform;
pub mod gen;

use syntax::AlephTree as at;

pub fn generate(content_type: String, content: String, transformer_list : Option<Vec<String>>, return_type: String) -> String {

    let parsed_content: syntax::AlephTree = parser::parse(content_type, content);

    let transformed_content: syntax::AlephTree = match transformer_list {
        Some(list)=> transform_dispatcher(list, parsed_content),
        None => parsed_content
    };
    
    return gen::generate(return_type, transformed_content);
}

pub fn transform_dispatcher(transformer_list: Vec<String>, ast: at) ->at{
    transformer_list.iter().fold(ast, |accum, transformer_name|
        {
            transform::transform(transformer_name.to_string(), accum)
        })
}
