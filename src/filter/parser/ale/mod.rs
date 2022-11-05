use crate::syntax::AlephTree as at;
use crate::filter::parser::Parser;

#[derive(Default)]
pub struct AleParser;

#[rust_sitter::grammar("ale")]
pub mod grammar {

    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum Expression {
        SE(Box<SimplExpr>),
        U(Box<Unary>),
        #[rust_sitter::prec_left(1)]
        Not(
            #[rust_sitter::leaf(text = "!")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        And(
            Box<Expression>,
            #[rust_sitter::leaf(text = "&")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        Or(
            Box<Expression>,
            #[rust_sitter::leaf(text = "|")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        EQ(
            Box<Expression>,
            #[rust_sitter::leaf(text = "=")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        LE(
            Box<Expression>,
            #[rust_sitter::leaf(text = "<=")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(2)]
        Add(
            Box<Expression>,
            #[rust_sitter::leaf(text = "+")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(2)]
        Sub(
            Box<Expression>,
            Box<Unary>,
        ),
        #[rust_sitter::prec_left(3)]
        Mul(
            Box<Expression>,
            #[rust_sitter::leaf(text = "*")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(3)]
        Div(
            Box<Expression>,
            #[rust_sitter::leaf(text = "/")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        If(
            Box<Condition>,
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        IfElse(
            Box<Condition>,
            Box<Expression>,
            #[rust_sitter::leaf(text = ":")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        While(
            Box<Expression>,
            Box<Condition>,
            #[rust_sitter::leaf(text = "*")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        Import(
            #[rust_sitter::leaf(text = "import")] (),
            Box<SimplExpr>,
        ),
        #[rust_sitter::prec_left(10)]
        IdentConst(
            Box<SimplExpr>,
            Box<IdentSucc>,
        ),
        #[rust_sitter::prec_left(2)]
        Stmts(
            Box<Expression>,
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        Unit(
            #[rust_sitter::leaf(text = ";")] (),
        ),
        #[rust_sitter::prec_left(1)]
        Tuple(
            Box<Tuple>,
        ),
        #[rust_sitter::prec_left(1)]
        Array(
            #[rust_sitter::leaf(text = "[")] (),
            Box<ExprList>,
            #[rust_sitter::leaf(text = "]")] (),
        ),
    }

    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum SimplExpr {
        Bool(#[rust_sitter::leaf(pattern = "true|false", transform = |v| v.parse().unwrap())] bool),
        Number(#[rust_sitter::leaf(pattern = r"\d+", transform = |v| v.parse().unwrap())] i64),
        Float(#[rust_sitter::leaf(pattern = r"\d*\.\d*",transform = |v| v.parse().unwrap())] f64),
        String(#[rust_sitter::leaf(pattern = r#""[^\"]*""#, transform = |v| v.parse().unwrap())] String),
        Ident(#[rust_sitter::leaf(pattern = r"[a-z](\d|[A-Za-z]|'_')*", transform = |v| v.parse().unwrap())] String),
        #[rust_sitter::prec_left(1)]
        LBexpRB(
            #[rust_sitter::leaf(text = "{")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = "}")] (),
        ),
    }
    
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum Condition {
        #[rust_sitter::prec_left(1)]
        Simpl(
            Box<Expression>,
            #[rust_sitter::leaf(text = "?")] (),
        ),
    }
    
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum Unary {
        #[rust_sitter::prec_left(1)]
        Neg(
            #[rust_sitter::leaf(text = "-")] (),
            Box<Expression>,
        ),
    }
        
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum Tuple {
        #[rust_sitter::prec_left(1)]
        Elems(
           #[rust_sitter::leaf(text = "(")] (),
           Box<ExprList>,
           #[rust_sitter::leaf(text = ")")] (),
        ),
        #[rust_sitter::prec_left(10)]
        Empty(
           #[rust_sitter::leaf(text = "()")] (),
        ),
    }
    
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum ExprList {
        #[rust_sitter::prec_left(1)]
        Node(
            Box<ExprList>,
            #[rust_sitter::leaf(text = ",")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(10)]
        Leaf(
            Box<Expression>,
        ),
    }
    
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum IdentSucc {
        Let(
            #[rust_sitter::leaf(text = "=")] (),
            Box<Expression>,
        ),
        LetP(
            #[rust_sitter::leaf(text = ":=")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(10)]
        App(
            Box<Tuple>,
        ),
    }

    #[rust_sitter::extra]
        #[derive(Debug)]
    struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
    }
}

fn translate_simple_expr(tree: grammar::SimplExpr) -> at {
    match tree {
        grammar::SimplExpr::Bool(b) => at::Bool{value: b.to_string()},
        grammar::SimplExpr::LBexpRB(_,e,_) => translate(*e),
        grammar::SimplExpr::Number(i) => at::Int{value: i.to_string()},
        grammar::SimplExpr::Float(f) => at::Float{value: f.to_string()},
        grammar::SimplExpr::String(s) => at::String{value: s},
        grammar::SimplExpr::Ident(s) => at::String{value: s},
    }
}

fn translate_se_string(tree: grammar::SimplExpr) -> String {
    match tree {
        grammar::SimplExpr::Bool(b) => b.to_string(),
        grammar::SimplExpr::LBexpRB(_,_,_) => "".to_string(),
        grammar::SimplExpr::Number(i) => i.to_string(),
        grammar::SimplExpr::Float(f) => f.to_string(),
        grammar::SimplExpr::String(s) => s,
        grammar::SimplExpr::Ident(s) => s,
    }
}

fn translate_unary(tree: grammar::Unary) -> at {
    match tree {
        grammar::Unary::Neg(_, e) => at::Neg{expr: Box::new(translate(*e))},
    }
}

fn translate_cond(tree: grammar::Condition) -> at {
    match tree {
        grammar::Condition::Simpl(e, _) => translate(*e),
    }
}

fn translate_list(list: grammar::ExprList) -> Vec<Box<at>> {
    match list {
        grammar::ExprList::Leaf(e) => vec![Box::new(translate(*e))],
        grammar::ExprList::Node(l, _, e) => {
            let mut v = translate_list(*l);
            v.push(Box::new(translate(*e)));
            v
        },
    }
}

fn translate_tuple(list: grammar::Tuple) -> Vec<Box<at>> {
    match list {
       grammar::Tuple::Empty(_) => Vec::new(),
       grammar::Tuple::Elems(_, l, _) => translate_list(*l),
    }
}

fn translate(tree : grammar::Expression) -> at {
    match tree {
        grammar::Expression::Unit(_) => at::Unit{},
        grammar::Expression::SE(se) => translate_simple_expr(*se),
        grammar::Expression::U(u) => translate_unary(*u),
        grammar::Expression::Not(_, e) => at::Not{bool_expr: Box::new(translate(*e))},
        grammar::Expression::And(e1,_, e2) => at::And{bool_expr1 : Box::new(translate(*e1)), bool_expr2: Box::new(translate(*e2))},
        grammar::Expression::Or(e1,_, e2) => at::Or{bool_expr1 : Box::new(translate(*e1)), bool_expr2: Box::new(translate(*e2))},
        grammar::Expression::EQ(e1,_, e2) => at::Eq{expr1 : Box::new(translate(*e1)), expr2: Box::new(translate(*e2))},
        grammar::Expression::LE(e1,_, e2) => at::LE{expr1 : Box::new(translate(*e1)), expr2: Box::new(translate(*e2))},
        grammar::Expression::Add(e1,_, e2) => at::Add{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))},
        grammar::Expression::Sub(e1,u2) => match *u2 {
            grammar::Unary::Neg(_, e2) => at::Sub{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))},
        }
        grammar::Expression::Mul(e1,_, e2) => at::Mul{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))},
        grammar::Expression::Div(e1,_, e2) => at::Div{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))}, 
        grammar::Expression::If(cond, then) => at::If{condition: Box::new(translate_cond(*cond)), then: Box::new(translate(*then)), els: Box::new(at::Unit
{})}, 
        grammar::Expression::IfElse(cond, then, _, els) => at::If{condition: Box::new(translate_cond(*cond)), then: Box::new(translate(*then)), els: Box::new
(translate(*els))}, 
        grammar::Expression::While(e1, cond, _, e2) => at::While{init_expr: Box::new(translate(*e1)), condition: Box::new(translate_cond(*cond)), loop_expr: 
Box::new(translate(*e2)), post_expr: Box::new(at::Unit{})}, 
        grammar::Expression::Import(_, name) => at::Iprt{name: translate_se_string(*name)}, 
        grammar::Expression::IdentConst(ident, succ) => match *succ {
            grammar::IdentSucc::Let(_, value) => at::Let{var: translate_se_string(*ident), is_pointer: "false".to_string(), value: Box::new(translate(*value)), expr: Box::new(at::Unit{})},
            grammar::IdentSucc::LetP(_, value) => at::Let{var: translate_se_string(*ident), is_pointer: "true".to_string(), value: Box::new(translate(*value)), expr: Box::new(at::Unit{})},
            grammar::IdentSucc::App(param_list) => at::App{object_name: "".to_string(), fun: Box::new(at::String{value: translate_se_string(*ident)}), param_list: translate_tuple(*param_list)},
        } 
        grammar::Expression::Tuple(list) => at::Tuple{elems: translate_tuple(*list)}, 
        grammar::Expression::Array(_, list, _) => at::Array{elems: translate_list(*list)}, 
        grammar::Expression::Stmts(e1, e2) => match (*e1, *e2) {
             (grammar::Expression::Unit(_), grammar::Expression::Unit(_)) => at::Unit{},
             (grammar::Expression::Unit(_), e21) => translate(e21),
             (e11, grammar::Expression::Unit(_)) => translate(e11),
             (e11, e21) => at::Stmts{expr1: Box::new(translate(e11)), expr2: Box::new(translate(e21))}, 
        },
    }
}

impl Parser for AleParser {
    fn parse(&self, source: String) -> at {
        translate(grammar::parse(&source).unwrap())
    }
}
