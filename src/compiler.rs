// parses Source Code directly to bytecode
use crate::lexer::{Lexer, Token, TokenKind};
use crate::vm::bytecode::{Chunk, Opcode};
use std::fmt;

// Number literals: 123
// Parentheses for grouping: (123)
// Unary negation: -123
// The Four Horsemen of the Arithmetic: +, -, *, /

// Precedence goes from lowest to highest descending None being lowest
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    None = 0,
    Assignment, // =
    Or,
    And,
    Equality,   //  ==, !=
    Comparison, // <, >, <=, >=
    Term,       // +, -
    Factor,     // *, /
    Unary,      // !, -
    Call,       // ()
    Primary,
}

impl Precedence {
    fn next(&self) -> Precedence {
        let ret_order = (self.order() + 1).min(10);
        Precedence::from_order(ret_order)
    }

    fn order(&self) -> u8 {
        match self {
            Precedence::None => 0,
            Precedence::Assignment => 1,
            Precedence::Or => 2,
            Precedence::And => 3,
            Precedence::Equality => 4,
            Precedence::Comparison => 5,
            Precedence::Term => 6,
            Precedence::Factor => 7,
            Precedence::Unary => 8,
            Precedence::Call => 9,
            Precedence::Primary => 10,
        }
    }

    fn from_order(order: u8) -> Precedence {
        match order {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparison,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            10 => Precedence::Primary,
            _ => panic!("Unrecognized order {}", order),
        }
    }
}

impl From<Token> for Precedence {
    fn from(token: Token) -> Self {
        match token.kind {
            TokenKind::Equal => Precedence::Assignment,
            TokenKind::IsEqual => Precedence::Equality,
            TokenKind::LessEqual
            | TokenKind::Less
            | TokenKind::GreaterEqual
            | TokenKind::Greater => Precedence::Comparison,
            TokenKind::Plus | TokenKind::Minus => Precedence::Term,
            TokenKind::Star | TokenKind::Slash => Precedence::Factor,
            TokenKind::Bang => Precedence::Unary,
            TokenKind::Dot => Precedence::Call,
            _ => Precedence::None,
        }
    }
}

impl From<TokenKind> for Precedence {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Equal => Precedence::Assignment,
            TokenKind::IsEqual => Precedence::Equality,
            TokenKind::LessEqual
            | TokenKind::Less
            | TokenKind::GreaterEqual
            | TokenKind::Greater => Precedence::Comparison,
            TokenKind::Plus | TokenKind::Minus => Precedence::Term,
            TokenKind::Star | TokenKind::Slash => Precedence::Factor,
            TokenKind::Bang => Precedence::Unary,
            TokenKind::Dot => Precedence::Call,
            _ => Precedence::None,
        }
    }
}

impl From<&Token> for Precedence {
    fn from(token: &Token) -> Self {
        match token.kind {
            TokenKind::Equal => Precedence::Assignment,
            TokenKind::IsEqual => Precedence::Equality,
            TokenKind::LessEqual
            | TokenKind::Less
            | TokenKind::GreaterEqual
            | TokenKind::Greater => Precedence::Comparison,
            TokenKind::Plus | TokenKind::Minus => Precedence::Term,
            TokenKind::Star | TokenKind::Slash => Precedence::Factor,
            TokenKind::Bang => Precedence::Unary,
            TokenKind::Dot => Precedence::Call,
            _ => Precedence::None,
        }
    }
}

impl From<&TokenKind> for Precedence {
    fn from(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Equal => Precedence::Assignment,
            TokenKind::IsEqual => Precedence::Equality,
            TokenKind::LessEqual
            | TokenKind::Less
            | TokenKind::GreaterEqual
            | TokenKind::Greater => Precedence::Comparison,
            TokenKind::Plus | TokenKind::Minus => Precedence::Term,
            TokenKind::Star | TokenKind::Slash => Precedence::Factor,
            TokenKind::Bang => Precedence::Unary,
            TokenKind::Dot => Precedence::Call,
            _ => Precedence::None,
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    SyntaxError,
    WrongTokenError,
    TokenError(Token),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::SyntaxError => write!(f, "SyntaxError for some reason"),
            ParseError::TokenError(token) => write!(f, "TokenError: {}", token.lexeme),
            ParseError::WrongTokenError => write!(f, "Wrong token being used"),
        }
    }
}

pub struct Compiler {
    current: Token,
    previous: Token,
    lexer: Lexer,
    pub chunk: Chunk,
}

impl Compiler {
    pub fn from_source(src: &str) -> Compiler {
        Compiler {
            current: Token::default_token(),
            previous: Token::default_token(),
            lexer: Lexer::new(src),
            chunk: Chunk::new(),
        }
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();
        loop {
            self.current = self.lexer.next_token();
            match self.current.clone().kind {
                TokenKind::Error => self.error_current(&self.current.lexeme),
                _ => break,
            }
        }
    }

    fn consume(&mut self, expected: TokenKind, msg: &str) {
        if self.current.kind == expected {
            self.advance()
        } else {
            self.error_current(msg)
        }
    }

    fn error(&self, line: u64, msg: &str) {
        println!("[line {}] Error: {}", line, msg)
    }
    fn error_current(&self, msg: &str) {
        self.error(self.current.line, msg)
    }

    fn parse_number(&mut self) {
        match self.previous.clone().kind {
            TokenKind::Number(num) => self.chunk.write_chunk(Opcode::Constant(num)),
            _ => (),
        }
    }

    fn parse_grouping(&mut self) {
        self.parse_expression();
        self.consume(TokenKind::Rpar, "expected a ')' token ")
    }

    fn parse_unary(&mut self) {
        let op_kind = self.previous.clone().kind;
        self.parse_precedence(Precedence::Unary);
        match op_kind {
            TokenKind::Minus => self.chunk.write_chunk(Opcode::Negate),
            _ => todo!(),
        }
    }

    fn parse_binary(&mut self) {
        let op_kind = self.previous.clone().kind;
        let next_precedence = Precedence::from(&op_kind).next();
        self.parse_precedence(next_precedence);
        match op_kind {
            TokenKind::Plus => self.chunk.write_chunk(Opcode::Add),
            TokenKind::Minus => self.chunk.write_chunk(Opcode::Subtract),
            TokenKind::Star => self.chunk.write_chunk(Opcode::Multiply),
            TokenKind::Slash => self.chunk.write_chunk(Opcode::Divide),
            _ => unimplemented!(),
        }
    }

    fn parse_expression(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        self.parse_prefix(self.previous.clone().kind);

        while precedence <= Precedence::from(&self.current.kind) {
            self.advance();
            self.parse_infix(self.previous.clone().kind);
        }
    }

    fn parse_prefix(&mut self, kind: TokenKind) {
        match kind {
            TokenKind::Bang | TokenKind::Minus => self.parse_unary(),
            TokenKind::Number(_) => self.parse_number(),
            TokenKind::LPar => self.parse_grouping(),
            _ => unimplemented!(),
        }
    }

    fn parse_infix(&mut self, kind: TokenKind) {
        match kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                self.parse_binary()
            }
            _ => unimplemented!(),
        }
    }

    fn is_eof(&self) -> bool {
        self.current.kind == TokenKind::Eof
    }

    pub fn compile(&mut self) {
        self.advance();
        while !self.is_eof() {
            self.parse_expression()
        }
        self.chunk.write_chunk(Opcode::Return)
    }
}
