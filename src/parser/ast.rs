use crate::parser::expr::Expr;
use std::fmt;

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stmts_str = self
            .stmts
            .iter()
            .map(|stmt| format!("    {}", stmt))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "\n{}\n", stmts_str)
    }
}

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
        then_body: Block,
        else_body: Option<Block>,
    },
    While {
        cond: Expr,
        body: Block,
    },
    Fn {
        name: String,
        args: Vec<(String, String)>,
        return_type: Option<String>,
        body: Block,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Block(Block),
    Return(Option<Expr>),
    Pkg(Vec<String>),
    Use(Vec<String>),
}
