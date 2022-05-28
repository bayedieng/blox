use crate::ast::Expression;
use crate::lexer::{Lexer, Token, TokenKind};

#[derive(Debug)]
pub enum ParseError {
    NumberError,
    MismatchError,
    ExpectedTokenError,
}

pub type ParseResult = Result<Expression, ParseError>;

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

pub struct Parser {
    previous: Token,
    current: Token,
    lexer: Lexer,
}

impl Parser {
    pub fn new(src: &str) -> Parser {
        Parser {
            previous: Token::default_token(),
            current: Token::default_token(),
            lexer: Lexer::new(src),
        }
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();
        loop {
            self.current = self.lexer.next_token();
            match self.current.kind {
                TokenKind::Error => {
                    eprintln!("Error {}. Line {}", self.current.lexeme, self.current.line)
                }
                _ => break,
            }
        }
    }

    fn expect_and_consume(&mut self, expected: TokenKind) -> Result<Token, ParseError> {
        if self.current.kind == expected {
            self.advance();
            return Ok(self.current.clone());
        } else {
            return Err(ParseError::MismatchError);
        }
    }

    fn parse_number(&self) -> ParseResult {
        match self.previous.clone().kind {
            TokenKind::Number(num) => Ok(Expression::Number(num)),
            _ => Err(ParseError::NumberError),
        }
    }

    fn parse_grouping(&mut self) -> ParseResult {
        let expr = self.parse_precedence(Precedence::None)?;
        self.expect_and_consume(TokenKind::Rpar)?;
        Ok(Expression::Grouping(Box::new(expr)))
    }

    fn parse_unary(&mut self) -> ParseResult {
        let operator = self.previous.clone();
        let expr = self.parse_precedence(Precedence::Unary)?;
        Ok(Expression::Unary {
            operator: operator,
            expression: Box::new(expr),
        })
    }

    fn parse_binary(&mut self) -> ParseResult {
        todo!()
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> ParseResult {
        self.advance();
        let mut expr = self.parse_prefix(self.previous.clone().kind)?;
        while !self.lexer.is_at_end() {
            let next_precedence = Precedence::from(self.current.clone());
            if precedence >= next_precedence {
                break;
            }
            expr = self.parse_infix(self.previous.clone().kind)?;
        }
        Ok(expr)
    }

    fn parse_prefix(&mut self, kind: TokenKind) -> ParseResult {
        match kind {
            TokenKind::Bang | TokenKind::Minus => self.parse_unary(),
            TokenKind::Number(_) => self.parse_number(),
            TokenKind::LPar => self.parse_grouping(),
            _ => Err(ParseError::MismatchError),
        }
    }

    fn parse_infix(&mut self, kind: TokenKind) -> ParseResult {
        match kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                self.parse_binary()
            }
            _ => Err(ParseError::MismatchError),
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.advance();
        self.parse_precedence(Precedence::None)
    }
}
