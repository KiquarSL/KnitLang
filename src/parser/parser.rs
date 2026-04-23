use crate::lexer::token::{Token, TokenType};
use crate::parser::expr::{CompOp, Expr, LogicOp, Op, Unary};

type TT = TokenType;

pub struct Parser {
    pos: usize,
    tokens: Vec<Token>,
    exprs: Vec<Expr>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            pos: 0,
            tokens,
            exprs: Vec::new(),
        }
    }

    // Expressions

    pub fn expr(&mut self) -> Expr {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();
        loop {
            if self.peek(0).kind == TT::Or {
                self.advance();
                left = Expr::Logic(Box::new(left), LogicOp::Or, Box::new(self.parse_and()));
                continue;
            }
            break;
        }
        left
    }

    fn parse_and(&mut self) -> Expr {
        let mut left = self.comparison();
        loop {
            if self.peek(0).kind == TT::And {
                self.advance();
                left = Expr::Logic(Box::new(left), LogicOp::And, Box::new(self.comparison()));
                continue;
            }
            break;
        }
        left
    }

    fn comparison(&mut self) -> Expr {
        let mut left = self.addition();
        loop {
            let op = match self.peek(0).kind {
                TT::Lt => CompOp::Lt,
                TT::Le => CompOp::Le,
                TT::Gt => CompOp::Gt,
                TT::Ge => CompOp::Ge,
                TT::Eq => CompOp::Eq,
                TT::Ne => CompOp::Ne,
                _ => break,
            };
            self.advance();
            let right = self.addition();
            left = Expr::Comp(Box::new(left), op, Box::new(right));
        }
        left
    }

    fn addition(&mut self) -> Expr {
        let mut left = self.multiplication();
        loop {
            if self.peek(0).kind == TT::Plus {
                self.advance();
                let right = self.multiplication();
                left = Expr::Arith(Box::new(left), Op::Add, Box::new(right));
            } else if self.peek(0).kind == TT::Minus {
                self.advance();
                let right = self.multiplication();
                left = Expr::Arith(Box::new(left), Op::Sub, Box::new(right));
            } else {
                break;
            }
        }
        left
    }

    fn multiplication(&mut self) -> Expr {
        let mut left = self.unary();
        loop {
            if self.peek(0).kind == TT::Star {
                self.advance();
                let right = self.unary();
                left = Expr::Arith(Box::new(left), Op::Mul, Box::new(right));
            } else if self.peek(0).kind == TT::Slash {
                self.advance();
                let right = self.unary();
                left = Expr::Arith(Box::new(left), Op::Div, Box::new(right));
            } else {
                break;
            }
        }
        left
    }
    fn unary(&mut self) -> Expr {
        let current = self.peek(0);
        match current.kind {
            TT::Minus => {
                self.advance();
                Expr::Unary(Unary::Neg, Box::new(self.primary()))
            }
            TT::Not => {
                self.advance();
                Expr::Unary(Unary::Not, Box::new(self.primary()))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        let token = self.peek(0);
        self.advance();
        match token.kind {
            TT::Number(num) => Expr::Number(num),
            TT::Bool(v) => Expr::Bool(v),
            TT::LParen => {
                self.match_type(TT::LParen);
                let result = self.expr();
                self.match_type(TT::RParen);
                result
            }
            _ => panic!("Unexpected token at {}:{}", token.line, token.col),
        }
    }

    // Support methods

    fn match_type(&mut self, kind: TokenType) -> bool {
        if self.peek(0).kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&self, offset: i8) -> Token {
        let index = self.pos + offset as usize;
        self.tokens[index].clone()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn len(&self) -> usize {
        self.tokens.len()
    }
}
