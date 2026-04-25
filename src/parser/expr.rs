use std::fmt;
use strum_macros;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Id(String),
    Arith(Box<Expr>, Op, Box<Expr>),
    Comp(Box<Expr>, CompOp, Box<Expr>),
    Logic(Box<Expr>, LogicOp, Box<Expr>),
    Unary(Unary, Box<Expr>),
    Bool(bool),
    String(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(num) => write!(f, "{}", num),
            Expr::String(v) => write!(f, "\"{}\"", v),
            Expr::Id(name) => write!(f, "{}", name),
            Expr::Bool(v) => write!(f, "{}", v),
            Expr::Arith(v1, op, v2) => write!(f, "({} {} {})", v1, op, v2),
            Expr::Comp(v1, op, v2) => write!(f, "({} {} {})", v1, op, v2),
            Expr::Logic(v1, op, v2) => write!(f, "({} {} {})", v1, op, v2),
            Expr::Unary(unary, v) => write!(f, "{}{}", unary, v),
        }
    }
}

#[derive(Debug, strum_macros::Display)]
pub enum Op {
    #[strum(to_string = "+")]
    Add,
    #[strum(to_string = "-")]
    Sub,
    #[strum(to_string = "*")]
    Mul,
    #[strum(to_string = "/")]
    Div,
}

#[derive(Debug, strum_macros::Display)]
pub enum CompOp {
    #[strum(to_string = "<")]
    Lt,
    #[strum(to_string = ">")]
    Gt,
    #[strum(to_string = ">=")]
    Ge,
    #[strum(to_string = "<=")]
    Le,
    #[strum(to_string = "==")]
    Eq,
    #[strum(to_string = "!=")]
    Ne,
}

#[derive(Debug, strum_macros::Display)]
pub enum LogicOp {
    #[strum(to_string = "&&")]
    And,
    #[strum(to_string = "||")]
    Or,
}

#[derive(Debug, strum_macros::Display)]
pub enum Unary {
    #[strum(to_string = "")]
    Pos,
    #[strum(to_string = "-")]
    Neg,
    #[strum(to_string = "!")]
    Not,
}
