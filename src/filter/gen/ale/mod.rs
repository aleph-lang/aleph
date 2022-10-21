use crate::syntax::AlephTree as at;
use crate::filter::gen::Gen;
use crate::filter::gen::comp_indent;

#[derive(Default)]
pub struct AleGen;

fn gen_list_expr(ast_list: Vec<Box<at>>) -> String {
    format!("{}", ast_list.into_iter().map(|e| gen(*e, 0)).collect::<Vec<String>>().join(", "))
}

fn gen(ast: at, indent: i64) -> String {
    match ast {
        at::Unit => format!("{}", ""),
        at::Int{value} => format!("{}{}", comp_indent(indent), value),
        at::Float{value} => format!("{} {}", comp_indent(indent), value),
        at::Bool{value} => format!("{}{}", comp_indent(indent), value),
        at::String{value} => format!("{}{}", comp_indent(indent), value),
        at::Tuple{elems} => format!("{}", gen_list_expr(elems)),
        at::Array{elems} => format!("{}", gen_list_expr(elems)),
        at::Neg{expr} => format!("{}-{}", comp_indent(indent), gen(*expr, 0)),
        at::Not{bool_expr} => format!("{}!({})", comp_indent(indent), gen(*bool_expr, 0)),
        at::And{bool_expr1, bool_expr2} => format!("{}{} & {}", comp_indent(indent), gen(*bool_expr1, 0), gen(*bool_expr2, 0)),
        at::Or{bool_expr1, bool_expr2} => format!("{}{} | {}", comp_indent(indent), gen(*bool_expr1, 0), gen(*bool_expr2, 0)),
        at::Add{number_expr1, number_expr2} => format!("{}{} + {}", comp_indent(indent), gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Sub{number_expr1, number_expr2} => format!("{}{} - {}", comp_indent(indent), gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Mul{number_expr1, number_expr2} => format!("{}{} * {}", comp_indent(indent), gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Div{number_expr1, number_expr2} => format!("{}{} / {}", comp_indent(indent), gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Eq{expr1, expr2} => format!("{}{} = {}", comp_indent(indent), gen(*expr1, 0), gen(*expr2, 0)),
        at::LE{expr1, expr2} => format!("{}{} <= {}", comp_indent(indent), gen(*expr1, 0), gen(*expr2, 0)),
        at::If{condition, then, els} => format!("{}({})?{{{}}}:{{{}}}", comp_indent(indent), gen(*condition, indent), gen(*then, indent), gen(*els, indent)),
        at::While{init_expr, condition, loop_expr, post_expr} => format!("{}{}\n{}({})?*{{\n{};\n{}}}", comp_indent(indent), gen(*init_expr, indent), comp_indent(indent), gen(*condition, 0), gen(*loop_expr, indent+1), gen(*post_expr, indent+1)),
        at::Let{var, is_pointer, value, expr} => format!("{}{}{} = {};\n{}", comp_indent(indent), var, (if is_pointer=="true" {":"} else {""}), gen(*value, 0), gen(*expr, indent)),
        at::LetRec{name, args, body} => format!("{}fun {}({}) = {{\n{}\n{}}}", comp_indent(indent), name, gen_list_expr(args), gen(*body, indent+1), comp_indent(indent)),
        at::Get{array_name, elem} => format!("{}{}[{}]", comp_indent(indent), array_name, gen(*elem, indent)),
        at::Put{array_name, elem, value, insert} => format!("{}{}[{}{}] = {}", comp_indent(indent), array_name, (if insert=="true" {"+"} else {""}), gen(*elem, indent), gen(*value, indent)),
        at::Remove{array_name, elem, is_value, ret: _} => format!("{}{} {} {}", comp_indent(indent), array_name, (if is_value=="true" {"-"} else {"/"}), gen(*elem, indent)),
        at::Length{var} => format!("{}|{}|", comp_indent(indent), var),
        at::Match{expr, case_list} => format!("{}match {} with\n{}", comp_indent(indent), gen(*expr, 0), gen_list_expr(case_list)),
        at::MatchLine{condition, case_expr} => format!("{}: {} -> {}\n", comp_indent(indent), gen(*condition, 0), gen(*case_expr, 0)),
        at::Var{var, is_pointer} => format!("{}{}{}",comp_indent(indent), (if is_pointer=="true" {"!"} else {""}), var),
        at::App{object_name, fun, param_list} => format!("{}{}{}({})",comp_indent(indent), (if object_name.ne("") {format!("{}.", object_name)} else {String::from("")}), gen(*fun, 0), gen_list_expr(param_list)),
        at::Stmts{expr1, expr2} => format!("{};\n{}", gen(*expr1, indent), gen(*expr2, indent)), 
        at::Iprt{name} => format!("{}import {}", comp_indent(indent), name),
        at::Clss{name, attribute_list, body} => format!("{}class {} {{\n{}{};\n{}\n}}", comp_indent(indent), name, comp_indent(indent+1), attribute_list.join(&format!(";\n{}", comp_indent(indent+1))), gen(*body, indent+1)), 
    }
}

impl Gen for AleGen {
    fn generate(&self, ast: at) -> String {
        gen(ast, 0)
    }
}
