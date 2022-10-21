use crate::syntax::AlephTree as at;
use crate::filter::parser::Parser;

#[derive(Default)]
pub struct AleParser;

#[rust_sitter::grammar("ale")]
pub mod grammar {
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum Expression {
        Bool(#[rust_sitter::leaf(pattern = "true|false", transform = |v| v.parse().unwrap())] bool),
        Number(#[rust_sitter::leaf(pattern = r"\d+", transform = |v| v.parse().unwrap())] i32),
        Float(#[rust_sitter::leaf(pattern = r"\d*\.\d*",transform = |v| v.parse().unwrap())] f32),
        #[rust_sitter::prec_left(1)]
        LPexpRP(
            #[rust_sitter::leaf(text = "(")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = ")")] (),
        ),
        #[rust_sitter::prec_left(10)]
        Not(
            #[rust_sitter::leaf(text = "!")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(10)]
        Neg(
            #[rust_sitter::leaf(text = "-")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(3)]
        And(
            Box<Expression>,
            #[rust_sitter::leaf(text = "&")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(2)]
        Or(
            Box<Expression>,
            #[rust_sitter::leaf(text = "|")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(3)]
        EQ(
            Box<Expression>,
            #[rust_sitter::leaf(text = "=")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(4)]
        LE(
            Box<Expression>,
            #[rust_sitter::leaf(text = "<=")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(5)]
        Add(
            Box<Expression>,
            #[rust_sitter::leaf(text = "+")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(6)]
        Sub(
            Box<Expression>,
            #[rust_sitter::leaf(text = "-")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(7)]
        Mul(
            Box<Expression>,
            #[rust_sitter::leaf(text = "*")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(8)]
        Div(
            Box<Expression>,
            #[rust_sitter::leaf(text = "/")] (),
            Box<Expression>,
        ),
    }

    #[rust_sitter::extra]
    struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
    }
}

fn translate(tree : grammar::Expression) -> at {
     match tree {
         grammar::Expression::LPexpRP(_,e,_) => translate(*e),
         grammar::Expression::Bool(b) => at::Bool{value: b.to_string()},
         grammar::Expression::Number(i) => at::Int{value: i.to_string()},
         grammar::Expression::Float(f) => at::Float{value: f.to_string()},
         grammar::Expression::Not(_, e) => at::Not{bool_expr: Box::new(translate(*e))},
         grammar::Expression::Neg(_, e) => at::Neg{expr: Box::new(translate(*e))},
         grammar::Expression::And(e1,_, e2) => at::And{bool_expr1 : Box::new(translate(*e1)), bool_expr2: Box::new(translate(*e2))},
         grammar::Expression::Or(e1,_, e2) => at::Or{bool_expr1 : Box::new(translate(*e1)), bool_expr2: Box::new(translate(*e2))},
         grammar::Expression::EQ(e1,_, e2) => at::Eq{expr1 : Box::new(translate(*e1)), expr2: Box::new(translate(*e2))},
         grammar::Expression::LE(e1,_, e2) => at::LE{expr1 : Box::new(translate(*e1)), expr2: Box::new(translate(*e2))},
         grammar::Expression::Add(e1,_, e2) => at::Add{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))},
         grammar::Expression::Sub(e1,_, e2) => at::Sub{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))},
         grammar::Expression::Mul(e1,_, e2) => at::Mul{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))},
         grammar::Expression::Div(e1,_, e2) => at::Div{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))}, 
     }
}

impl Parser for AleParser {
    fn parse(&self, source: String) -> at {
       translate(grammar::parse(&source).unwrap())
    }
}
