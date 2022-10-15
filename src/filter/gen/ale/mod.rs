use crate::syntax::AlephTree as at;
use crate::filter::gen::Gen;

//TODO: move in GenEnum or GenTypes
pub struct AleGen {}

impl Gen for AleGen {
    fn gen( ast: at) -> String {
        match ast {
            at::Unit => format!("{}", "Unit"),
            at::Int{value} => format!("Int {}", value),
            at::Float{value} => format!("Float {}", value),
            at::Bool{value} => format!("Bool {}", value),
            at::String{value} => format!("String {}", value),
            at::Neg{expr} => format!("Neg {}", Self::gen(*expr)),
            at::Not{bool_expr} => format!("Not {}", Self::gen(*bool_expr)),
            at::And{bool_expr1, bool_expr2} => format!("And {}, {}", Self::gen(*bool_expr1), Self::gen(*bool_expr2)),
            at::Or{bool_expr1, bool_expr2} => format!("Or {}, {}", Self::gen(*bool_expr1), Self::gen(*bool_expr2)),
            at::Add{number_expr1, number_expr2} => format!("Add {}, {}", Self::gen(*number_expr1), Self::gen(*number_expr2)),
            at::Sub{number_expr1, number_expr2} => format!("Sub {}, {}", Self::gen(*number_expr1), Self::gen(*number_expr2)),
            at::Mul{number_expr1, number_expr2} => format!("Mul {}, {}", Self::gen(*number_expr1), Self::gen(*number_expr2)),
            at::Div{number_expr1, number_expr2} => format!("Div {}, {}", Self::gen(*number_expr1), Self::gen(*number_expr2)),
            at::Eq{expr1, expr2} => format!("Eq {}, {}", Self::gen(*expr1), Self::gen(*expr2)),
            at::LE{expr1, expr2} => format!("LE {}, {}", Self::gen(*expr1), Self::gen(*expr2)),
            at::If{condition, then, els} => format!("If {}, {}, {}", Self::gen(*condition), Self::gen(*then), Self::gen(*els)),
            at::While{condition, init_expr, loop_expr} => format!("While {}, {}, {}", Self::gen(*condition), Self::gen(*init_expr), Self::gen(*loop_expr)),
            at::Let{var, is_pointer, value, expr} => format!("Let {} {} = {}; {}", var, is_pointer, Self::gen(*value), Self::gen(*expr)),
            at::LetRec{name, args, body} => format!("LetRec {}, {}, {}", name, "TODO args", Self::gen(*body)),
            at::Var{var, is_pointer} => format!("Var {} {}", var, is_pointer),
            at::App{object_name, fun, param_list} => format!("App {}", Self::gen(*fun)),
            at::Stmts{expr1, expr2} => format!("Stmts {}, {}", Self::gen(*expr1), Self::gen(*expr2)), 
            _ => format!("{}", "Spec")
        }
    }

    fn generate(ast: at) -> String {
        println!("AST {:?}", &ast);
        Self::gen(ast)
    }
}
