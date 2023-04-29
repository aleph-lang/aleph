pub mod parser;
pub mod transform;
pub mod gen;

use aleph_syntax_tree::syntax::AlephTree as at;

/// Generate
/// #Arguments
/// `content_type` - Name of content type
/// `content` - Source code
/// `transformer_list` - List of transoformer function
/// `return_type` - Name of ouput language
///
/// # Return
/// This function return generated source code
pub fn generate(content_type: String, content: String, transformer_list : Option<Vec<String>>, return_type: String) -> String {

    let parsed_content: at = parser::parse(content_type, content);

    let transformed_content: at = match transformer_list {
        Some(list)=> transform_dispatcher(list, parsed_content),
        None => parsed_content
    };
    
    return gen::generate(return_type, transformed_content);
}

/// Generator
/// #Arguments
/// `transformer_list` - List of transoformer function
/// `ast` - ast source to transform
///
/// # Return
/// This function return an Alephtree
pub fn transform_dispatcher(transformer_list: Vec<String>, ast: at) ->at{
    transformer_list.iter().fold(ast, |accum, transformer_name|
        {
            transform::transform(transformer_name.to_string(), accum)
        })
}
