pub mod ast;
pub mod expr;
pub mod parser;

pub use parser::Parser;

use crate::lexer::token::Token;
use crate::parser::ast::Stmt;

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    Parser::new(tokens).parse()
}
