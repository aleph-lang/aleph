use crate::syntax;
use syntax::AlephTree as at;

fn gen(ast: at) -> String {
    match ast {
        at::Unit => format!("{}", "Unit"),
        at::Int{value} => format!("Int {}", value),
        at::Float{value} => format!("Float {}", value),
        at::Bool{value} => format!("Bool {}", value),
        at::String{value} => format!("String {}", value),
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
        at::While{condition, init_expr, loop_expr} => format!("While {}, {}, {}", gen(*condition), gen(*init_expr), gen(*loop_expr)),
        at::Let{var, is_pointer, value, expr} => format!("Let {} {} = {}; {}", var, is_pointer, gen(*value), gen(*expr)),
        at::LetRec{name, args, body} => format!("LetRec {}, {}, {}", name, "TODO args", gen(*body)),
        at::Var{var, is_pointer} => format!("Var {} {}", var, is_pointer),
        at::App{object_name, fun, param_list} => format!("App {}", gen(*fun)),
        at::Stmts{expr1, expr2} => format!("Stmts {}, {}", gen(*expr1), gen(*expr2)), 
        _ => format!("{}", "Spec")
    }
}

pub fn generate(ast: at) -> String {
    println!("AST {:?}", &ast);
    gen(ast)
}
