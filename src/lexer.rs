#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single Character Tokens
    LPar,
    Rpar,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Semicolon,
    Minus,
    Plus,
    Star,

    // Single or More Character tokens
    Bang,
    NotBang,
    Equal,
    IsEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Ident(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    lexeme: String,
    line: u64,
}

pub struct Lexer {
    start: usize,
    current: usize,
    line: u64,
    src: String,
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src: src.to_string(),
            current: 0,
            start: 0,
            line: 1,
        }
    }

    fn char_at(&self, idx: usize) -> char {
        self.src.chars().nth(idx).unwrap()
    }

    fn next_char(&mut self) -> char {
        self.current += 1;
        self.char_at(self.current - 1)
    }

    fn peek(&self) -> char {
        self.char_at(self.current)
    }

    fn peek_next(&self) -> char {
        self.char_at(self.current + 1)
    }

    fn eat(&mut self) {
        self.current += 1
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => self.eat(),
                '\n' => {
                    self.line += 1;
                    self.eat()
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.eat()
                        }
                        self.eat()
                    }
                }
                _ => break,
            }
        }
    }

    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.eat()
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.eat()
        }
        while self.peek().is_digit(10) {
            self.eat()
        }
        let num: f64 = self.src[self.start..self.current].parse().unwrap();

        self.make_token(TokenKind::Number(num))
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        Token {
            kind: kind,
            lexeme: self.src[self.start..self.current].to_string(),
            line: self.line,
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.eat()
        }

        if self.is_at_end() {
            return self.error_token("unterminated string");
        }
        let string = self.src[self.start..self.current].to_string();
        self.make_token(TokenKind::String(string))
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphanumeric() {
            self.eat()
        }
        let ident_or_keyword = &self.src[self.start..self.current];
        match ident_or_keyword {
            "and" => self.make_token(TokenKind::And),
            "class" => self.make_token(TokenKind::Class),
            "else" => self.make_token(TokenKind::Else),
            "false" => self.make_token(TokenKind::False),
            "for" => self.make_token(TokenKind::For),
            "fun" => self.make_token(TokenKind::Fun),
            "if" => self.make_token(TokenKind::If),
            "nil" => self.make_token(TokenKind::Nil),
            "or" => self.make_token(TokenKind::Or),
            "print" => self.make_token(TokenKind::Print),
            "return" => self.make_token(TokenKind::Return),
            "super" => self.make_token(TokenKind::Super),
            "this" => self.make_token(TokenKind::This),
            "true" => self.make_token(TokenKind::True),
            "var" => self.make_token(TokenKind::Var),
            "while" => self.make_token(TokenKind::While),
            _ => self.make_token(TokenKind::Ident(ident_or_keyword.to_string())),
        }
    }

    fn error_token(&mut self, msg: &str) -> Token {
        Token {
            kind: TokenKind::Error,
            lexeme: msg.to_string(),
            line: self.line,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        let c = self.next_char();
        match c {
            '(' => self.make_token(TokenKind::LPar),
            ')' => self.make_token(TokenKind::Rpar),
            '{' => self.make_token(TokenKind::LBrace),
            '}' => self.make_token(TokenKind::RBrace),
            ';' => self.make_token(TokenKind::Semicolon),
            '+' => self.make_token(TokenKind::Plus),
            '-' => self.make_token(TokenKind::Minus),
            '*' => self.make_token(TokenKind::Star),
            '.' => self.make_token(TokenKind::Dot),
            '!' => {
                if self.is_match('=') {
                    return self.make_token(TokenKind::NotBang);
                }
                self.make_token(TokenKind::Bang)
            }

            '>' => {
                if self.is_match('=') {
                    return self.make_token(TokenKind::GreaterEqual);
                }
                self.make_token(TokenKind::Greater)
            }

            '<' => {
                if self.is_match('=') {
                    return self.make_token(TokenKind::LessEqual);
                }
                self.make_token(TokenKind::Less)
            }
            '=' => {
                if self.is_match('=') {
                    return self.make_token(TokenKind::IsEqual);
                }
                self.make_token(TokenKind::Equal)
            }
            '0'..='9' => self.number(),
            '"' => self.string(),
            'a'..='z' | 'A'..='Z' => self.identifier(),

            '\0' => self.make_token(TokenKind::Eof),
            _ => Token {
                kind: TokenKind::Error,
                lexeme: format!("Token error, line {}", self.line),
                line: self.line,
            },
        }
    }
}
