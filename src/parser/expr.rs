#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Arith(Box<Expr>, Op, Box<Expr>),
    Comp(Box<Expr>, CompOp, Box<Expr>),
    Logic(Box<Expr>, LogicOp, Box<Expr>),
    Unary(Unary, Box<Expr>),
    Bool(bool),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum CompOp {
    Lt, // <
    Gt, // >
    Ge, // >=
    Le, // <=
    Eq, // ==
    Ne, // !=
}

#[derive(Debug)]
pub enum LogicOp {
    And, // &&
    Or,  // ||
}

#[derive(Debug)]
pub enum Unary {
    Pos,
    Neg,
    Not,
}
