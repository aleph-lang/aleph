use crate::syntax;
use crate::filter::transform::Transform;
use syntax::AlephTree as at;

#[derive(Default)]
pub struct BetaReduction;


fn beta(ast: at, env:Vec<String>) -> at {
    match ast {

        at::Length{var} => at::Length{var: var},

        at::Neg{expr} => at::Neg{expr:Box::new(beta(*expr, env))},

        at::And{bool_expr1,bool_expr2} => at::And{bool_expr1: Box::new(beta(*bool_expr1, env.clone())), bool_expr2: Box::new(beta(*bool_expr2, env))},

        at::Or{bool_expr1,bool_expr2} => at::Or{bool_expr1: Box::new(beta(*bool_expr1, env.clone())), bool_expr2: Box::new(beta(*bool_expr2, env))},

        at::Eq{expr1,expr2} => at::Eq{expr1: Box::new(beta(*expr1, env.clone())), expr2: Box::new(beta(*expr2, env))},

        at::LE{expr1,expr2} => at::LE{expr1: Box::new(beta(*expr1, env.clone())), expr2: Box::new(beta(*expr2, env))},

        at::Add{number_expr1, number_expr2} => at::Add{number_expr1: Box::new(beta(*number_expr1, env.clone())), number_expr2: Box::new(beta(*number_expr2, env))},

        at::Sub{number_expr1,number_expr2} => at::Sub{number_expr1: Box::new(beta(*number_expr1, env.clone())), number_expr2: Box::new(beta(*number_expr2, env))},

        at::Mul{number_expr1,number_expr2} => at::Mul{number_expr1: Box::new(beta(*number_expr1, env.clone())), number_expr2: Box::new(beta(*number_expr2, env))},

        at::Div{number_expr1,number_expr2} => at::Div{number_expr1: Box::new(beta(*number_expr1, env.clone())), number_expr2: Box::new(beta(*number_expr2, env))},

        at::If{condition, then, els} => at::If{condition:condition , then:Box::new(beta(*then, env.clone())), els:Box::new(beta(*els, env))},

        at::Stmts{expr1,expr2} => at::Stmts{expr1:Box::new(beta(*expr1, env.clone())), expr2:Box::new(beta(*expr2, env))},

        at::While{init_expr, condition, loop_expr, post_expr} => at::While{
            init_expr:Box::new(beta(*init_expr, env.clone())),
            condition:Box::new(beta(*condition, env.clone())),
            loop_expr:Box::new(beta(*loop_expr, env.clone())),
            post_expr:Box::new(beta(*post_expr, env.clone()))},

        _ => ast
    }
}

impl Transform for BetaReduction{
    fn transform(&self, ast: at) -> at{
        println!("doing beta red");
        beta(ast, vec![])
    }
}

