use crate::syntax::AlephTree as at;
use crate::filter::parser::Parser;

#[derive(Default)]
pub struct AleParser;

#[rust_sitter::grammar("arithmetic")]
pub mod grammar {
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum Expression {
        Number(#[rust_sitter::leaf(pattern = r"\d+", transform = |v| v.parse().unwrap())] i32),
        #[rust_sitter::prec_left(1)]
        Sub(
            Box<Expression>,
            #[rust_sitter::leaf(text = "-")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(2)]
        Mul(
            Box<Expression>,
            #[rust_sitter::leaf(text = "*")] (),
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
         grammar::Expression::Number(i) => at::Int{value: i.to_string()},
         grammar::Expression::Sub(e1,_, e2) => at::Sub{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))},
         _ => at::Int{value: "-1".to_string()}
     }
}

impl Parser for AleParser {
    fn parse(&self, source: String) -> at {
       translate(grammar::parse(&source).unwrap())
    }
}
