use aleph_syntax_tree::syntax::AlephTree as at;

use rustpython_parser::parser;
use rustpython_parser::ast::{Program, Expression};
use rustpython_parser::ast::Statement;
use rustpython_parser::ast;

/*fn translate_ast_to_aleph_tree(ast: ast::Node) -> at {
    match ast {
        ast::Node::Stmt(stmts) => {
            let mut expr1 = at::Unit;
            let mut expr2 = at::Unit;

            for stmt in stmts.iter() {
                let new_expr = translate_ast_to_aleph_tree(stmt.clone());

                if expr1 == at::Unit {
                    expr1 = new_expr;
                } else if expr2 == at::Unit {
                    expr2 = new_expr;
                } else {
                    expr1 = at::Stmts {
                        expr1: Box::new(expr1),
                        expr2: Box::new(expr2),
                    };
                    expr2 = new_expr;
                }
            }

            if expr2 == at::Unit {
                expr1
            } else {
                at::Stmts {
                    expr1: Box::new(expr1),
                    expr2: Box::new(expr2),
                }
            }
        }
        ast::Node::Expr(expr) => translate_expr_to_aleph_tree(expr),
        _ => at::Unit,
    }
}

fn translate_expr_to_aleph_tree(expr: ast::Expr) -> at {
    match expr {
        ast::Expr::BoolOp(boolop) => {
            match boolop.op {
                ast::BoolOperator::And => at::And {
                    bool_expr1: Box::new(translate_expr_to_aleph_tree(boolop.values[0].clone())),
                    bool_expr2: Box::new(translate_expr_to_aleph_tree(boolop.values[1].clone())),
                },
                ast::BoolOperator::Or => at::Or {
                    bool_expr1: Box::new(translate_expr_to_aleph_tree(boolop.values[0].clone())),
                    bool_expr2: Box::new(translate_expr_to_aleph_tree(boolop.values[1].clone())),
                },
            }
        }
        ast::Expr::BinOp(binop) => {
            match binop.op {
                ast::BinOperator::Add => at::Add {
                    number_expr1: Box::new(translate_expr_to_aleph_tree(*binop.left)),
                    number_expr2: Box::new(translate_expr_to_aleph_tree(*binop.right)),
                },
                ast::BinOperator::Sub => at::Sub {
                    number_expr1: Box::new(translate_expr_to_aleph_tree(*binop.left)),
                    number_expr2: Box::new(translate_expr_to_aleph_tree(*binop.right)),
                },
                _ => at::Unit,
            }
        }
    }
}*/

pub fn python_parse(source: String) -> at {
    let ast = parser::parse_program(&source).unwrap();
    println!("AST: {:?}", ast);
    // translate_ast_to_aleph_tree(&ast)
    at::Unit
}


