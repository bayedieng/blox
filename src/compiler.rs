// parses Source Code directly to bytecode 
use crate::lexer::{Lexer, Token, TokenKind};
use crate::vm::bytecode::{Chunk, Opcode};
use std::fmt;

// Number literals: 123
// Parentheses for grouping: (123)
// Unary negation: -123
// The Four Horsemen of the Arithmetic: +, -, *, /

// Precedence goes from lowest to highest descending None being lowest
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,
    And,
    Equality, //  ==, !=
    Comparison, // <, >, <=, >=
    Term, // +, -
    Factor, // *, /
    Unary, // !, -
    Call, // ()
    Primary

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
            _ => Precedence::None
        }
        
    }
}

#[derive(Debug)]
pub enum ParseError {
    SyntaxError,
    WrongTokenError,
    TokenError(Token)
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::SyntaxError => write!(f, "SyntaxError for some reason"),
            ParseError::TokenError(token) => write!(f, "TokenError: {}", token.lexeme),
            ParseError::WrongTokenError => write!(f, "Wrong token being used")
        }
    }
}

pub struct Compiler {
    current: Token,
    previous: Token,
    lexer: Lexer,
    pub chunk: Chunk
}

impl Compiler {
    pub fn from_source(src: &str) -> Compiler {
        Compiler {
            current: Token::default_token(),
            previous: Token::default_token(),
            lexer: Lexer::new(src),
            chunk: Chunk::new()
        }
    }

    pub fn advance(&mut self) -> Result<Token, ParseError> {
        self.previous = self.current.clone();

        loop {
            self.current = self.lexer.next_token();
            match self.current.kind {
                TokenKind::Error => return Err(ParseError::TokenError(self.current.clone())),
                _ => break
            }
        }
        Ok(self.current.clone())
    }

    fn consume_if_same(&mut self, expected: TokenKind) -> Result<(), ParseError> {
        if self.current.kind == expected {
            self.advance()?;
            return Ok(())
        }
        Err(ParseError::WrongTokenError)

    }

    fn parse_expr(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn parse_number(&mut self) -> Result<(), ParseError> {
        match self.current.kind {
            TokenKind::Number(num) => Ok(self.chunk.write_chunk(Opcode::Constant(num))),
            _ => Err(ParseError::WrongTokenError)
        }
    }

    fn parse_grouping(&mut self) -> Result<(), ParseError> {
        self.parse_expr()?;
        self.consume_if_same(TokenKind::Rpar)
    }

    fn parse_unary(&mut self) -> Result<(), ParseError> {
        let op_kind = self.previous.clone().kind;
        self.parse_expr()?;
        match op_kind {
            TokenKind::Minus => Ok(self.chunk.write_chunk(Opcode::Negate)),
            _ => Err(ParseError::WrongTokenError)
        }
    }

    fn parse_binary(&mut self) -> Result<(), ParseError> {
        let op_kind = self.current.clone().kind;
        match op_kind {
            TokenKind::Plus => Ok(self.chunk.write_chunk(Opcode::Add)),
            TokenKind::Minus => Ok(self.chunk.write_chunk(Opcode::Subtract)),
            TokenKind::Slash => Ok(self.chunk.write_chunk(Opcode::Divide)),
            TokenKind::Star => Ok(self.chunk.write_chunk(Opcode::Multiply)),
            _ => Err(ParseError::WrongTokenError)
        }

    }

    pub fn compile(&mut self) {
        loop {
            self.advance().unwrap();
            let op_kind = self.current.clone().kind;
            match op_kind {
                TokenKind::Number(_) => {
                    self.parse_number();
                    ()
                },
                TokenKind::Eof => break,
                _ => {
                    if is_prefix(op_kind) {
                        self.parse_prefix();
                        ()
                    }
                    self.parse_infix();
                    ()
                }
            }
        }

    }
    fn parse_prefix(&mut self) -> Result<(), ParseError> {
        match self.current.kind {
            TokenKind::Minus => self.parse_unary(),
            TokenKind::LPar => self.parse_grouping(),

            _ => Err(ParseError::WrongTokenError)
        }
    }

    fn parse_infix(&mut self) -> Result<(), ParseError> {
        match self.current.kind {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash => self.parse_binary(),
            _ => Err(ParseError::WrongTokenError)
        }
    }

}

fn is_prefix(kind: TokenKind) -> bool {
    match kind {
        TokenKind::LPar | TokenKind::Minus => true,
        _ => false
    }
}