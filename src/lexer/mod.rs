use crate::lexer::token::Token;

pub mod lexer;
pub mod token;

pub use lexer::Lexer;

pub fn tokenize(source: String) -> Vec<Token> {
    Lexer::new(&source.clone()).tokenize()
}
