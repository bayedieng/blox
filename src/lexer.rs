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
    Slash,

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

    fn char_at(&self, idx: usize) -> Option<char> {
        self.src.chars().nth(idx)
    }

    fn next_char(&mut self) -> Option<char> {
        self.current += 1;
        self.char_at(self.current - 1)
    }

    fn peek(&self) -> Option<char> {
        self.char_at(self.current)
    }

    fn peek_is_digit(&self) -> bool {
        match self.peek() {
            Some(c) => c.is_digit(10),
            None => false
        }
    }

    fn peek_is_alphanumeric(&self) -> bool {
        match self.peek() {
            Some(c) => c.is_alphanumeric(),
            None => false
        }
    }

    fn peek_matches_next(&self, expected: char) -> bool {
        match self.peek() {
            Some(c) => c == expected,
            None => false
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.next_char();
                }
                Some('\n') => {
                    self.line += 1;
                    self.next_char();
                }
                Some('/') => {
                    if self.peek_matches_next('/') {
                        while !self.peek_matches_next('\n') && !self.is_at_end() {
                            self.next_char();
                        }
                    }

                }
                _ => break
            }
        }
    }

    fn number(&mut self) -> Token {
        while self.peek_is_digit() {
            self.next_char();
        }

       if self.peek_matches_next('.') {
           self.next_char();
           while self.peek_is_digit() {
               self.next_char();
           }
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
        while !self.peek_matches_next('"') && !self.is_at_end() {
            if self.peek_matches_next('\n') {
                self.line += 1;
            }
            self.next_char();
        }
        if self.is_at_end() {
            return self.error_token("unterminated string")
        }
        
        self.next_char();
        let string = self.src[self.start..self.current].to_string();
        self.make_token(TokenKind::String(string))
    }

    fn identifier(&mut self) -> Token {
        while self.peek_is_alphanumeric() {
            self.next_char();
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

    fn make_eof(&self) -> Token {
        Token {
            kind: TokenKind::Eof,
            lexeme: "\0".to_string(),
            line: self.line,
        }
    }

    fn token_matches(&mut self, first: TokenKind, second: TokenKind) -> Token {
        if self.peek() == Some('=') {
            self.next_char();
            return self.make_token(first);
        }
        self.make_token(second)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        let c = self.next_char();
        match c {
            Some(c) => match c {
                c if c.is_alphabetic() => self.identifier(),
                c if c.is_digit(10) => self.number(),

                '(' => self.make_token(TokenKind::LPar),
                ')' => self.make_token(TokenKind::Rpar),
                '{' => self.make_token(TokenKind::LBrace),
                '}' => self.make_token(TokenKind::RBrace),
                ';' => self.make_token(TokenKind::Semicolon),
                ',' => self.make_token(TokenKind::Comma),
                '.' => self.make_token(TokenKind::Dot),
                '-' => self.make_token(TokenKind::Minus),
                '+' => self.make_token(TokenKind::Plus),
                '/' => self.make_token(TokenKind::Slash),
                '*' => self.make_token(TokenKind::Star),

                '!' => self.token_matches(TokenKind::NotBang, TokenKind::Bang),
                '=' => self.token_matches(TokenKind::IsEqual, TokenKind::Equal),
                '>' => self.token_matches(TokenKind::GreaterEqual, TokenKind::Greater),
                '<' => self.token_matches(TokenKind::LessEqual, TokenKind::Less),

                '"' => self.string(),
                _ => self.error_token(&format!("Unexpected character {}", c)),
            },

            None => self.make_eof(),
        }
    }
}
