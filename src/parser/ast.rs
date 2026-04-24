use crate::parser::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    NewVar {
        mutable: bool,
        name: String,
        type_name: String,
        value: Option<Expr>,
    },
    Assign {
        name: String,
        value: Expr,
    },
    If {
        cond: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    Fn {
        name: String,
        args: Vec<(String, String)>,
        return_type: Option<String>,
        body: Vec<Stmt>,
    },
    Call {
        path: Vec<String>,
        args: Vec<Expr>,
    },
    Block(Vec<Stmt>),
    Return(Option<Expr>),
    Pkg(Vec<String>),
    Use(Vec<String>),
}
