use crate::syntax::AlephTree as at;
use crate::filter::gen::Gen;

#[derive(Default)]
pub struct AleGen;

fn gen_list_expr(ast_list: Vec<Box<at>>) -> String {
    format!("{}", ast_list.into_iter().map(|e| gen(*e)).collect::<Vec<String>>().join(", "))
}

fn gen( ast: at) -> String {
    match ast {
        at::Unit => format!("{}", "Unit"),
        at::Int{value} => format!("Int {}", value),
        at::Float{value} => format!("Float {}", value),
        at::Bool{value} => format!("Bool {}", value),
        at::String{value} => format!("String {}", value),
        at::Tuple{elems} => format!("Tuple {}", gen_list_expr(elems)),
        at::Array{elems} => format!("Array {}", gen_list_expr(elems)),
        at::Neg{expr} => format!("Neg {}", gen(*expr)),
        at::Not{bool_expr} => format!("Not {}", gen(*bool_expr)),
        at::And{bool_expr1, bool_expr2} => format!("And {}, {}", gen(*bool_expr1), gen(*bool_expr2)),
        at::Or{bool_expr1, bool_expr2} => format!("Or {}, {}", gen(*bool_expr1), gen(*bool_expr2)),
        at::Add{number_expr1, number_expr2} => format!("Add {}, {}", gen(*number_expr1), gen(*number_expr2)),
        at::Sub{number_expr1, number_expr2} => format!("Sub {}, {}", gen(*number_expr1), gen(*number_expr2)),
        at::Mul{number_expr1, number_expr2} => format!("Mul {}, {}", gen(*number_expr1), gen(*number_expr2)),
        at::Div{number_expr1, number_expr2} => format!("Div {}, {}", gen(*number_expr1), gen(*number_expr2)),
        at::Eq{expr1, expr2} => format!("Eq {}, {}", gen(*expr1), gen(*expr2)),
        at::LE{expr1, expr2} => format!("LE {}, {}", gen(*expr1), gen(*expr2)),
        at::If{condition, then, els} => format!("If {}, {}, {}", gen(*condition), gen(*then), gen(*els)),
        at::While{init_expr, condition, loop_expr, post_expr} => format!("While {}, {}, {}, {}", gen(*init_expr), gen(*condition), gen(*loop_expr), gen(*post_expr)),
        at::Let{var, is_pointer, value, expr} => format!("Let {} {} = {}; {}", var, is_pointer, gen(*value), gen(*expr)),
        at::LetRec{name, args, body} => format!("LetRec {}, {}, {}", name, gen_list_expr(args), gen(*body)),
        at::Get{array_name, elem} => format!("Get {}, {}", array_name, gen(*elem)),
        at::Put{array_name, elem, value, insert} => format!("Put {}, {},{}, {}", array_name, gen(*elem), gen(*value), insert),
        at::Remove{array_name, elem, is_value, ret} => format!("Remove {}, {} {} {}", array_name, gen(*elem), is_value, ret),
        at::Length{var} => format!("Length {}", var),
        at::Match{expr, case_list} => format!("Match {}; {}", gen(*expr), gen_list_expr(case_list)),
        at::MatchLine{condition, case_expr} => format!("MatchLine {} -> {}", gen(*condition), gen(*case_expr)),
        at::Var{var, is_pointer} => format!("Var {} {}", var, is_pointer),
        at::App{object_name, fun, param_list} => format!("App {}.{} {}",object_name, gen(*fun), gen_list_expr(param_list)),
        at::Stmts{expr1, expr2} => format!("Stmts {}, {}", gen(*expr1), gen(*expr2)), 
        at::Iprt{name} => format!("Import {}", name),
        at::Clss{name, attribute_list, body} => format!("Class {}, {}, {}", name, attribute_list.join("\n "), gen(*body)), 
    }
}

impl Gen for AleGen {
    fn generate(ast: at) -> String {
        println!("AST {:?}", &ast);
        gen(ast)
    }
}
