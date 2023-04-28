use aleph_syntax_tree::syntax::AlephTree as at;

use rustpython_parser::{ast, parser};
use rustpython_parser::ast::{ExprKind, StmtKind};
use crate::ast::Constant;

fn extract_constant(value : Constant) -> at {
    match value {
        Constant::None => at::Unit,
        Constant::Bool(b) => at::Bool{value: b.to_string()},
        Constant::Str(s) => at::String{value: format!("\"{}\"", s)},
        Constant::Bytes(b) => {
            println!("Not yet impl {:?}", b);
            at::Unit
        },
        Constant::Int(i) => at::Int{value: i.to_string()},
        Constant::Tuple(v) => {
            println!("Not yet impl {:?}", v);
            at::Unit
        },
        Constant::Float(f) => at::Float{value: f.to_string()},
        Constant::Complex{real, imag} => {
            println!("Not yet impl {:?} + ({:?} * i)", real, imag);
            at::Unit
        },
        Constant::Ellipsis => {
            println!("Not yet impl Ellipsis : {:?}", value);
            at::Unit
        }
    }
}

fn extract_name(ek: ExprKind) -> String {
    match ek {
        ExprKind::Name{id, ctx: _} => id,
        _ => {
            println!("Not Impl {:?}", ek);
            "".to_string()
        }
    }
}

fn translate_expr_kind(ek: ExprKind) -> at {
    match ek {
        ExprKind::BoolOp{op, values} => {
            println!("Not impl {:?} {:?}", op, values);
            at::Unit
        },
        ExprKind::NamedExpr{target, value} => {
            println!("Not impl {:?} {:?}", target, value);
            at::Unit
        },
        ExprKind::BinOp{left, op, right} => {
            println!("Not impl {:?} {:?} {:?}", left, op, right);
            at::Unit
        },
        ExprKind::UnaryOp{op, operand} => {
            println!("Not impl {:?} {:?}", op, operand);
            at::Unit
        },
        ExprKind::Lambda{args, body} => {
            println!("Not impl {:?} {:?}", args, body);
            at::Unit
        },
        ExprKind::IfExp{test, body, orelse} => {
            println!("Not impl {:?} {:?} {:?}", test, body, orelse);
            at::Unit
        },
        ExprKind::Dict{keys, values} => {
            println!("Not impl {:?} {:?}", keys, values);
            at::Unit
        },
        ExprKind::Set{elts} => {
            println!("Not impl {:?}", elts);
            at::Unit
        },
        ExprKind::ListComp{elt, generators} => {
            println!("Not impl {:?} {:?}", elt, generators);
            at::Unit
        },
        ExprKind::SetComp{elt, generators} => {
            println!("Not impl {:?} {:?}", elt, generators);
            at::Unit
        },
        ExprKind::DictComp{key, value, generators} => {
            println!("Not impl {:?} {:?} {:?}", key, value, generators);
            at::Unit
        },
        ExprKind::GeneratorExp{elt, generators} => {
            println!("Not impl {:?} {:?}", elt, generators);
            at::Unit
        },
        ExprKind::Await{value} => {
            println!("Not impl {:?}", value);
            at::Unit
        },
        ExprKind::Yield{value} => {
            println!("Not impl {:?}", value);
            at::Unit
        },
        ExprKind::YieldFrom{value} => {
            println!("Not impl {:?}", value);
            at::Unit
        },
        ExprKind::Compare{left, ops, comparators} => {
            println!("Not impl {:?} {:?} {:?}", left, ops, comparators);
            at::Unit
        },
        ExprKind::Call{func, args, keywords: _} => {
            let name = extract_name(func.node);
            let mut param_list: Vec<Box<at>> = Vec::new();
            for arg in args {
                param_list.push(Box::new(translate_expr_kind(arg.node)));
            }
            at::App{object_name: "".to_string(), fun: Box::new(at::String{value: name}), param_list: param_list}
        },
        ExprKind::FormattedValue{value, conversion, format_spec} => {
            println!("Not impl {:?} {:?} {:?}", value, conversion, format_spec);
            at::Unit
        },
        ExprKind::JoinedStr{values} => {
            println!("Not impl {:?}", values);
            at::Unit
        },
        ExprKind::Constant{value, kind: _} => extract_constant(value),
        ExprKind::Attribute{value, attr, ctx} => {
            println!("Not impl {:?} {:?} {:?}", value, attr, ctx);
            at::Unit
        },
        ExprKind::Subscript{value, slice, ctx} => {
            println!("Not impl {:?} {:?} {:?}", value, slice, ctx);
            at::Unit
        },
        ExprKind::Starred{value, ctx} => {
            println!("Not impl {:?} {:?}", value, ctx);
            at::Unit
        },
        ExprKind::Name{id, ctx} => {
            println!("Not impl {:?} {:?}", id, ctx);
            at::Unit
        },
        ExprKind::List{elts, ctx} => {
            println!("Not impl {:?} {:?}", elts, ctx);
            at::Unit
        },
        ExprKind::Tuple{elts, ctx} => {
            println!("Not impl {:?} {:?}", elts, ctx);
            at::Unit
        },
        ExprKind::Slice{lower, upper, step} => {
            println!("Not impl {:?} {:?} {:?}", lower, upper, step);
            at::Unit
        }
    }
}

fn translate_stmt_kind(sk : StmtKind) -> at {
    match sk {
        StmtKind::FunctionDef{name, args, body, decorator_list, returns, type_comment} => {
            println!("Not impl {:?} {:?} {:?} {:?} {:?} {:?}", name, args, body, decorator_list, returns, type_comment);
            at::Unit 
        },
        StmtKind::AsyncFunctionDef{name, args, body, decorator_list, returns, type_comment} => {
            println!("Not impl {:?} {:?} {:?} {:?} {:?} {:?}", name, args, body, decorator_list, returns, type_comment);
            at::Unit 
        },
        StmtKind::ClassDef{name, bases, keywords, body, decorator_list} => {
            println!("Not impl {:?} {:?} {:?} {:?} {:?}", name, bases, keywords, body, decorator_list);
            at::Unit 
        },
        StmtKind::Return{value} => {
            println!("Not impl {:?}", value);
            at::Unit 
        },
        StmtKind::Delete{targets} => {
            println!("Not impl {:?}", targets);
            at::Unit 
        },
        StmtKind::Assign{targets, value, type_comment} => {
            println!("Not impl {:?} {:?} {:?}", targets, value, type_comment);
            at::Unit 
        },
        StmtKind::AugAssign{target, op, value} => {
            println!("Not impl {:?} {:?} {:?}", target, op, value);
            at::Unit 
        },
        StmtKind::AnnAssign{target, annotation, value, simple} => {
            println!("Not impl {:?} {:?} {:?} {:?}", target, annotation, value, simple);
            at::Unit 
        },
        StmtKind::For{target, iter, body, orelse, type_comment} => {
            println!("Not impl {:?} {:?} {:?} {:?} {:?}", target, iter, body, orelse, type_comment);
            at::Unit 
        },
        StmtKind::AsyncFor{target, iter, body, orelse, type_comment} => {
            println!("Not impl {:?} {:?} {:?} {:?} {:?}", target, iter, body, orelse, type_comment);
            at::Unit 
        },
        StmtKind::While{test, body, orelse} => {
            println!("Not impl {:?} {:?} {:?}", test, body, orelse);
            at::Unit 
        },
        StmtKind::If{test, body, orelse} => {
            println!("Not impl {:?} {:?} {:?}", test, body, orelse);
            at::Unit 
        },
        StmtKind::With{items, body, type_comment} => {
            println!("Not impl {:?} {:?} {:?}", items, body, type_comment);
            at::Unit 
        },
        StmtKind::AsyncWith{items, body, type_comment} => {
            println!("Not impl {:?} {:?} {:?}", items, body, type_comment);
            at::Unit 
        },
        StmtKind::Match{subject, cases} => {
            println!("Not impl {:?} {:?}", subject, cases);
            at::Unit 
        },
        StmtKind::Raise{exc, cause} => {
            println!("Not impl {:?} {:?}", exc, cause);
            at::Unit 
        },
        StmtKind::Try{body, handlers, orelse, finalbody} => {
            println!("Not impl {:?} {:?} {:?} {:?}", body, handlers, orelse, finalbody);
            at::Unit 
        },
        StmtKind::Assert{test, msg} => {
            println!("Not impl {:?} {:?}", test, msg);
            at::Unit 
        },
        StmtKind::Import{names} => {
            println!("Not impl {:?}", names);
            at::Unit 
        },
        StmtKind::ImportFrom{module, names, level} => {
            println!("Not impl {:?} {:?} {:?}", module, names, level);
            at::Unit 
        },
        StmtKind::Global{names} => {
            println!("Not impl {:?}", names);
            at::Unit 
        },
        StmtKind::Nonlocal{names} => {
            println!("Not impl {:?}", names);
            at::Unit 
        },
        StmtKind::Expr{value} => translate_expr_kind(value.node),
        StmtKind::Pass => {
            println!("Not impl");
            at::Unit 
        },
        StmtKind::Break => {
            println!("Not impl");
            at::Unit 
        },
        StmtKind::Continue => {
            println!("Not impl");
            at::Unit 
        }
    }
}

pub fn python_parse(source: String) -> at {
    let ast = parser::parse_program(&source, "<embedded>").unwrap();
    //  println!("AST: {:?}", ast);
    let mut res = at::Unit;
    for statement in ast {
        res = match res {
            at::Unit => translate_stmt_kind(statement.node),
            _ => at::Stmts{expr1: Box::new(res), expr2: Box::new(translate_stmt_kind(statement.node))}
        }
    }
    res
}


