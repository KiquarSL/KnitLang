use crate::lexer::token::{Keyword, Token, TokenType};

type TT = TokenType;
const SINGLE_CHARS: &str = "(){}[].,;:";
const DEFAULT_TYPES: &[&str] = &["i32", "f32", "bool", "char"];

pub struct Lexer {
    pos: usize,
    text: Vec<char>,
    tokens: Vec<Token>,

    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            pos: 0,
            text: source.chars().collect(),
            tokens: Vec::new(),
            line: 0,
            col: 0,
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        while self.pos < self.len() {
            let start_line = self.line;
            let start_col = self.col;
            let Some(current) = self.peek(0) else { break };

            match current {
                ch if ch.is_whitespace() => {
                    // skip whitespaces
                    self.advance();
                    continue;
                }
                ch if SINGLE_CHARS.contains(ch) => {
                    let kind = match ch {
                        '.' => TT::Dot,
                        ',' => TT::Comma,
                        ';' => TT::Semicolon,
                        ':' => TT::Colon,
                        '(' => TT::LParen,
                        ')' => TT::RParen,
                        '{' => TT::LBrace,
                        '}' => TT::RBrace,
                        '[' => TT::LBracket,
                        ']' => TT::RBracket,
                        _ => unreachable!(),
                    };
                    self.push(kind, start_line, start_col);
                }
                '+' => {
                    if self.peek(1) == Some('=') {
                        self.push(TT::PlusAssign, start_line, start_col);
                        self.advance();
                    } else {
                        self.push(TT::Plus, start_line, start_col);
                    }
                }
                '&' => {
                    if self.peek(1) == Some('&') {
                        self.push(TT::And, start_line, start_col);
                        self.advance();
                        self.advance();
                    } else {
                        break;
                    }
                }
                '|' => {
                    if self.peek(1) == Some('|') {
                        self.push(TT::Or, start_line, start_col);
                        self.advance();
                        self.advance();
                    } else {
                        break;
                    }
                }
                '-' => {
                    if self.peek(1) == Some('=') {
                        self.push(TT::MinusAssign, start_line, start_col);
                        self.advance();
                    } else if self.peek(1) == Some('-') {
                        while self.peek(0) != Some('\n') && self.pos < self.len() {
                            self.advance();
                        }
                        continue;
                    } else {
                        self.push(TT::Minus, start_line, start_col);
                    }
                }
                '*' => {
                    if self.peek(1) == Some('=') {
                        self.push(TT::StarAssign, start_line, start_col);
                        self.advance();
                    } else if self.peek(1) == Some('*') {
                        self.push(TT::Pow, start_line, start_col);
                        self.advance();
                    } else {
                        self.push(TT::Star, start_line, start_col);
                    }
                }
                '/' => {
                    if self.peek(1) == Some('*') {
                        self.advance();
                        self.advance();
                        while let (Some(a), Some(b)) = (self.peek(0), self.peek(1)) {
                            self.advance();
                            if a == '*' && b == '/' && self.pos < self.len() {
                                self.advance();
                                self.advance();
                                break;
                            }
                        }
                        continue;
                    } else {
                        self.push(TT::Slash, start_line, start_col);
                    }
                }
                '=' => {
                    if self.peek(1) == Some('=') {
                        self.push(TT::Eq, start_line, start_col);
                        self.advance();
                    } else {
                        self.push(TT::Assign, start_line, start_col);
                    }
                }
                '<' => {
                    if self.peek(1) == Some('=') {
                        self.push(TT::Le, start_line, start_col);
                        self.advance();
                    } else {
                        self.push(TT::Lt, start_line, start_col);
                    }
                }
                '>' => {
                    if self.peek(1) == Some('=') {
                        self.push(TT::Ge, start_line, start_col);
                        self.advance();
                    } else {
                        self.push(TT::Gt, start_line, start_col);
                    }
                }
                '!' => {
                    if self.peek(1) == Some('=') {
                        self.push(TT::Ne, start_line, start_col);
                        self.advance();
                    } else {
                        self.push(TT::Not, start_line, start_col);
                    }
                }
                '%' => {
                    self.push(TT::Rem, start_line, start_col);
                }
                ch if ch.is_alphabetic() => {
                    let mut buffer = String::new();
                    while let Some(c) = self.peek(0) {
                        self.advance();
                        if c.is_alphabetic() || c.is_digit(10) || c == '_' && self.pos < self.len()
                        {
                            buffer.push(c);
                        } else {
                            break;
                        }
                    }
                    if let Ok(kw) = buffer.parse::<Keyword>() {
                        self.push(TT::Keyword(kw), start_line, start_col);
                    } else if buffer == "true" || buffer == "false" {
                        self.push(TT::Bool(buffer == "true"), start_line, start_col);
                    } else if DEFAULT_TYPES.contains(&buffer.as_str()) {
                        self.push(TT::Type(buffer), start_line, start_col);
                    } else {
                        self.push(TT::Id(buffer), start_line, start_col);
                    }
                    continue;
                }
                ch if ch.is_digit(10) => {
                    let mut buffer = String::new();
                    let mut has_dot = false;
                    while let Some(c) = self.peek(0)
                        && self.pos < self.len()
                    {
                        self.advance();
                        if c.is_digit(10) {
                            buffer.push(c);
                        } else if c == '.' && !has_dot {
                            buffer.push(c);
                            has_dot = true;
                        } else {
                            break;
                        }
                    }
                    let num = buffer.parse::<f64>().unwrap();
                    self.push(TT::Number(num), start_line, start_col);
                    continue;
                }
                '"' | '\'' => {
                    let quote = current;
                    self.advance();
                    let mut buffer = String::new();
                    while let Some(c) = self.peek(0)
                        && self.pos < self.len()
                    {
                        self.advance();
                        if c == quote {
                            break;
                        }
                        buffer.push(c);
                    }
                    self.push(TT::String(buffer), start_line, start_col);
                    continue;
                }
                _ => {
                    panic!("Unknown char '{}' at {}:{}", current, self.line, self.col);
                }
            }
            self.advance();
        }
        self.push(TT::Eof, self.line, self.col);
        self.tokens
    }

    // Support methods

    fn peek(&self, offset: i8) -> Option<char> {
        let index = self.pos + offset as usize;
        if index < self.text.len() {
            Some(self.text[index])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if self.pos < self.text.len() {
            if self.text[self.pos] == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }
    }

    fn len(&self) -> usize {
        self.text.len()
    }

    fn push(&mut self, kind: TokenType, line: usize, col: usize) {
        self.tokens.push(Token::new(kind, line, col));
    }
}
