use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros;

#[derive(Debug, strum_macros::Display, Clone, PartialEq)]
pub enum TokenType {
    #[strum(to_string = "+")]
    Plus,
    #[strum(to_string = "-")]
    Minus,
    #[strum(to_string = "*")]
    Star,
    #[strum(to_string = "/")]
    Slash,
    #[strum(to_string = "=")]
    Assign,
    #[strum(to_string = "**")]
    Pow,
    #[strum(to_string = "//")]
    FloorDiv,
    #[strum(to_string = "%")]
    Rem,

    #[strum(to_string = "+=")]
    PlusAssign,
    #[strum(to_string = "-=")]
    MinusAssign,
    #[strum(to_string = "*=")]
    StarAssign,
    #[strum(to_string = "/=")]
    SlashAsaign,

    #[strum(to_string = "(")]
    LParen,
    #[strum(to_string = ")")]
    RParen,
    #[strum(to_string = "{{")]
    LBrace,
    #[strum(to_string = "}}")]
    RBrace,
    #[strum(to_string = "[")]
    LBracket,
    #[strum(to_string = "]")]
    RBracket,

    #[strum(to_string = "==")]
    Eq,
    #[strum(to_string = "!=")]
    Ne,
    #[strum(to_string = "!")]
    Not,
    #[strum(to_string = "<")]
    Lt,
    #[strum(to_string = ">")]
    Gt,
    #[strum(to_string = "<=")]
    Le,
    #[strum(to_string = ">=")]
    Ge,
    #[strum(to_string = "&&")]
    And,
    #[strum(to_string = "||")]
    Or,

    #[strum(to_string = ":")]
    Colon,
    #[strum(to_string = ";")]
    Semicolon,
    #[strum(to_string = ",")]
    Comma,
    #[strum(to_string = ".")]
    Dot,

    String(String),
    Number(f64),
    Bool(bool),
    Keyword(Keyword),
    Id(String),
    Type(String),
    Eof,
}

#[derive(
    Debug, Clone, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString, PartialEq,
)]
pub enum Keyword {
    #[strum(to_string = "if")]
    If,
    #[strum(to_string = "else")]
    Else,
    #[strum(to_string = "fn")]
    Fn,
    #[strum(to_string = "for")]
    For,
    #[strum(to_string = "while")]
    While,
    #[strum(to_string = "return")]
    Return,
}

impl Keyword {
    pub fn is_keyword(word: String) -> bool {
        Keyword::iter().any(|kw| kw.to_string() == word)
    }

    pub fn to_token(word: String) -> TokenType {
        TokenType::Keyword(Self::from_str(&word.clone()).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(kind: TokenType, line: usize, col: usize) -> Self {
        Self { kind, line, col }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " Token in {}:{} {:?}", self.line, self.col, self.kind)
    }
}
