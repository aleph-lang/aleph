use aleph_syntax_tree::syntax::AlephTree as at;

use rustpython_parser::ast;
use rustpython_parser::parser;
use rustpython_parser::ast::Located;
use rustpython_parser::ast::ExprKind;
use rustpython_parser::ast::ExprKind::Name;
use rustpython_parser::ast::ExprKind::Constant;
use crate::ast::Constant::Str;

fn translate_expression(value: Box<Located<ExprKind>>) -> at {
    match value.node {
       ExprKind::Call{func, args, ..} => {
           let Name{id, ..} = func.node else { todo!() };
           let mut param_list: Vec<Box<at>> = Vec::new();
           for arg in args {
               match arg.node {
                    v => {
                        match v {
                            Constant{value, ..} => match value {
                                Str(s) => param_list.push(Box::new(at::String{value: format!("\"{}\"", s)})),
                                e => println!("Not yet impl {:?}", e)
                            },
                            e => println!("Not yet impl {:?}", e)
                        }
                    }
               }
           }
           return at::App{object_name: "".to_string(), fun: Box::new(at::String{value: id}), param_list: param_list}
       },
       e => println!("Not impl {:?}", e)
    }
    at::Unit
}

pub fn python_parse(source: String) -> at {
    let ast = parser::parse_program(&source, "<embedded>").unwrap();
    //  println!("AST: {:?}", ast);
    let mut res = at::Unit;
    for statement in ast {
        res = match statement.node {
             ast::StmtKind::Expr{value} => match res {
                at::Unit => translate_expression(value),
                _ => at::Stmts{expr1: Box::new(res), expr2: Box::new(translate_expression(value))}
             },
             sk => {
                 println!("Not implemented {:?}", sk);
                 res
             }
        }
    }
    res
}


