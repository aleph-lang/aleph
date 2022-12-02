use crate::filter::at;

#[cfg(feature="ale_gen")]
mod ale;
#[cfg(feature="bash_gen")]
mod bash;
#[cfg(feature="java_gen")]
mod java;
#[cfg(feature="json_gen")]
mod json;
#[cfg(feature="ocaml_gen")]
mod ocaml;
#[cfg(feature="python_gen")]
mod python;
#[cfg(feature="rust_gen")]
mod rust;


fn to_gen(s:String) -> Option<Box<dyn Gen>> {
    match s.as_str() {
        #[cfg(feature="ale_gen")]
        "aleph" | "ale" => Some(Box::new(ale::AleGen{})),
        #[cfg(feature="bash_gen")]
        "bash" => Some(Box::new(bash::BashGen{})),
        #[cfg(feature="java_gen")]
        "java" => Some(Box::new(java::JavaGen{})),
        #[cfg(feature="json_gen")]
        "json" => Some(Box::new(json::JsonGen{})),
        #[cfg(feature="ocaml_gen")]
        "ocaml" | "ml" => Some(Box::new(ocaml::OcamlGen{})),
        #[cfg(feature="python_gen")]
        "python" | "py" => Some(Box::new(python::PythonGen{})),
        #[cfg(feature="rust_gen")]
        "rust" | "rs" => Some(Box::new(rust::RustGen{})),
        _ => None
    }
}

// select right generator and generate the code from the ast
pub fn generate(to: String, ast: at) -> String {
    match to_gen(to) {
        Some(g) => g.generate(ast),
        None => "Generator Not Implemented: Use external binary as generator".to_string()
    }
}

fn gen_list_expr_sep(ast_list: Vec<Box<at>>, f: fn(at, i64) -> String, sep: &str) -> String {
    format!("{}", ast_list.into_iter().map(|e| f(*e, 0)).collect::<Vec<String>>().join(sep))
}

fn gen_list_expr(ast_list: Vec<Box<at>>, f: fn(at, i64) -> String) -> String {
    gen_list_expr_sep(ast_list, f, " ")
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
   comp_indent_sep(indent, String::from("    "))
}

/**
* this trait should be implemented by all generators
*/
trait Gen {
    fn generate(&self, ast: at) -> String ;
}
