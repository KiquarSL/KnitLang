mod lexer;
mod token;

pub use lexer::Lexer;
pub use token::{Keyword, Token, TokenType};

pub fn tokenize(source: String) -> Vec<Token> {
    Lexer::new(&source.clone()).tokenize()
}
