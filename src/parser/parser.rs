use crate::lexer::token::{Keyword, Token, TokenType};
use crate::parser::ast::Stmt;
use crate::parser::expr::{CompOp, Expr, LogicOp, Op, Unary};

type TT = TokenType;

pub struct Parser {
    pos: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { pos: 0, tokens }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek(0).kind != TT::Eof {
            if self.is_statement() {
                stmts.push(self.parse_stmt());
            } else {
                panic!(
                    "Unexpected token at {}:{}",
                    self.peek(0).line,
                    self.peek(0).col
                );
            }
        }
        stmts
    }

    fn match_type(&mut self, expected: TokenType) -> bool {
        if self.peek(0).kind == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, expected: TokenType) {
        let current = self.peek(0);
        if current.kind == expected {
            self.advance();
        } else {
            panic!(
                "Expected {:?}, got {:?} at {}:{}",
                expected, current.kind, current.line, current.col
            )
        }
    }

    fn peek(&self, offset: i8) -> Token {
        let index = self.pos + offset as usize;
        self.tokens[index].clone()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    // path::to::object
    fn parse_path(&mut self) -> Vec<String> {
        let mut segments = Vec::new();
        let first = match self.peek(0).kind {
            TT::Id(ref name) => name.clone(),
            _ => panic!(
                "Expected identifier at {}:{}",
                self.peek(0).line,
                self.peek(0).col
            ),
        };
        self.advance();
        segments.push(first);

        while self.peek(0).kind == TT::Path {
            self.advance();
            let segment = match self.peek(0).kind {
                TT::Id(ref name) => name.clone(),
                _ => panic!(
                    "Expected identifier at {}:{}",
                    self.peek(0).line,
                    self.peek(0).col
                ),
            };
            self.advance();
            segments.push(segment);
        }
        segments
    }
}

impl Parser {
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
        match token.kind {
            TT::Number(num) => {
                self.advance();
                Expr::Number(num)
            }
            TT::String(string) => {
                self.advance();
                Expr::String(string)
            }
            TT::Id(name) => {
                self.advance();
                Expr::Id(name)
            }
            TT::Bool(v) => {
                self.advance();
                Expr::Bool(v)
            }
            TT::LParen => {
                self.consume(TT::LParen);
                let result = self.expr();
                self.consume(TT::RParen);
                result
            }
            _ => panic!("Unexpected token at {}:{}", token.line, token.col),
        }
    }
}

impl Parser {
    fn parse_stmt(&mut self) -> Stmt {
        match (self.peek(0).kind, self.peek(1).kind, self.peek(2).kind) {
            (TT::Keyword(Keyword::Pkg), TT::Id(_), _) => self.parse_pkg(),
            (TT::Keyword(Keyword::Use), TT::Id(_), _) => self.parse_use(),
            (TT::Keyword(Keyword::Fn), TT::Id(_), _) => self.parse_fn(),
            (TT::Keyword(Keyword::If), _, _) => self.parse_if(),
            (TT::Keyword(Keyword::While), _, _) => self.parse_while(),
            (TT::Keyword(Keyword::Ret), _, _) => self.parse_return(),
            (TT::Id(_), TT::Assign, _) => self.parse_assign(),
            (TT::Id(_), TT::Path, _) => self.parse_call(),
            (TT::Keyword(Keyword::Imut), TT::Id(_), TT::Id(_)) => self.parse_new_var(), // imut тип имя
            (TT::Id(_), TT::Id(_), _) => self.parse_new_var(),                          // тип имя
            _ => panic!(
                "Unexpected token at {}:{}",
                self.peek(0).line,
                self.peek(0).col
            ),
        }
    }
    fn is_statement(&self) -> bool {
        match (self.peek(0).kind, self.peek(1).kind) {
            (TT::Keyword(Keyword::Pkg), TT::Id(_))
            | (TT::Keyword(Keyword::Use), TT::Id(_))
            | (TT::Keyword(Keyword::Fn), TT::Id(_))
            | (TT::Keyword(Keyword::If), _)
            | (TT::Keyword(Keyword::While), _)
            | (TT::Keyword(Keyword::Ret), _)
            | (TT::Id(_), TT::Assign)
            | (TT::Id(_), TT::Path)
            | (TT::Id(_), TT::Id(_))
            | (TT::Keyword(Keyword::Imut), TT::Id(_)) => true,
            _ => false,
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut block = Vec::new();
        while self.peek(0).kind != TT::Eof {
            if self.is_statement() {
                block.push(self.parse_stmt());
            } else {
                break;
            }
        }
        block
    }
    fn parse_fn(&mut self) -> Stmt {
        self.consume(TT::Keyword(Keyword::Fn));
        if let TokenType::Id(name) = self.peek(0).kind {
            self.advance();
            self.consume(TT::LParen);
            let mut args = Vec::new();
            let mut return_type = None;
            while self.peek(0).kind != TT::RParen {
                if let TT::Id(type_name) = self.peek(0).kind {
                    self.advance();
                    if let TT::Id(name) = self.peek(0).kind {
                        args.push((name, type_name));
                    } else {
                        panic!("Expected id at {}:{}", self.peek(0).line, self.peek(0).col);
                    }
                    continue;
                } else if self.peek(0).kind == TT::Comma {
                    self.advance();
                    continue;
                }
            }
            self.advance();
            if self.peek(0).kind == TT::Arrow {
                self.advance();
            }
            if let TT::Id(name) = self.peek(0).kind {
                return_type = Some(name);
                self.advance();
            }
            self.consume(TT::LBrace);
            let body = self.parse_block();
            self.consume(TT::RBrace);
            return Stmt::Fn {
                name,
                args,
                return_type,
                body,
            };
        }
        panic!("Expected id at {}:{}", self.peek(0).line, self.peek(0).col);
    }

    fn parse_if(&mut self) -> Stmt {
        self.consume(TT::Keyword(Keyword::If));
        let cond = self.expr();
        self.consume(TT::LBrace);
        let then_body = self.parse_block();
        let mut else_body: Option<Vec<Stmt>> = None;
        self.consume(TT::RBrace);
        if self.match_type(TT::Keyword(Keyword::Else)) {
            self.consume(TT::LBrace);
            else_body = Some(self.parse_block());
            self.consume(TT::RBrace);
        }
        return Stmt::If {
            cond,
            then_body,
            else_body,
        };
    }

    fn parse_while(&mut self) -> Stmt {
        self.consume(TT::Keyword(Keyword::While));
        let cond = self.expr();
        self.consume(TT::LBrace);
        let body = self.parse_block();
        self.consume(TT::RBrace);

        return Stmt::While { cond, body };
    }

    fn parse_pkg(&mut self) -> Stmt {
        self.consume(TT::Keyword(Keyword::Pkg));
        let path = self.parse_path();
        self.advance();
        self.match_type(TT::Semicolon);
        Stmt::Pkg(path)
    }

    fn parse_use(&mut self) -> Stmt {
        self.consume(TT::Keyword(Keyword::Use));
        let path = self.parse_path();
        self.match_type(TT::Semicolon);
        Stmt::Use(path)
    }

    fn parse_call(&mut self) -> Stmt {
        let path = self.parse_path();
        self.consume(TT::LParen);
        let mut args = Vec::new();

        while self.peek(0).kind != TT::RParen {
            if self.peek(0).kind == TT::Comma {
                self.advance();
                continue;
            }
            args.push(self.expr());
        }

        self.consume(TT::RParen);
        self.match_type(TT::Semicolon);

        Stmt::Call { path, args }
    }

    fn parse_return(&mut self) -> Stmt {
        self.consume(TT::Keyword(Keyword::Ret));
        let expr = self.expr();
        self.match_type(TT::Semicolon);
        Stmt::Return(Some(expr))
    }

    fn parse_new_var(&mut self) -> Stmt {
        let mut mutable = true;
        if self.match_type(TT::Keyword(Keyword::Imut)) {
            mutable = false;
        }
        let dtype_t = self.peek(0);
        let name_t = self.peek(1);
        if let TokenType::Id(type_name) = dtype_t.kind {
            if let TokenType::Id(name) = name_t.kind {
                self.advance();
                self.advance();
                let mut value: Option<Expr> = None;

                let mut is_success = false;
                if let TokenType::Assign = self.peek(0).kind {
                    // type name = value;
                    self.advance();
                    value = Some(self.expr());
                    self.match_type(TT::Semicolon);
                    is_success = true;
                } else if let TokenType::Semicolon = self.peek(0).kind {
                    // type name;
                    self.consume(TT::Semicolon);
                    is_success = true;
                }
                if is_success {
                    return Stmt::NewVar {
                        mutable,
                        name,
                        type_name,
                        value,
                    };
                }
            }
        }
        panic!("Expected id at {}:{}", self.peek(0).line, self.peek(0).col);
    }

    fn parse_assign(&mut self) -> Stmt {
        let name = match self.peek(0).kind {
            TT::Id(ref n) => n.clone(),
            _ => panic!(
                "Expected identifier at {}:{}",
                self.peek(0).line,
                self.peek(0).col
            ),
        };
        self.advance();
        self.consume(TT::Assign);
        let value = self.expr();
        self.match_type(TT::Semicolon);
        Stmt::Assign { name, value }
    }
}
