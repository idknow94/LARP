#![allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TokenKind {
    Identifier(String),
    Number(f64),
    Let,
    Const,
    If,
    Else,
    For,
    While,
    
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Equal,
    Greater,
    GTE,
    Less,
    LTE,
    Not,
    NE,
    And,
    Or,
    BAND,
    BOR,
    BXOR,

    Comma,
    Colon,
    Semicolon,
    Quote,
    DQuote,
    LPar,
    RPar,
    LCurly,
    RCurly,
    LSquare,
    RSquare,

    EOF,
    Unknown
}
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer<'_> {
        Lexer { input: input, position: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    pub fn end(&self) -> bool {
        return self.position >= self.input.len();
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.position += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }

        let start = self.position;

        if self.end() {
            return Token {
                kind: TokenKind::EOF,
                span: Span { start, end: start },
            };
        }

        let ch = self.peek().unwrap();

        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut word = String::new();

            while let Some(c) = self.peek() {
                if c.is_ascii_alphanumeric() || c == '_' {
                    word.push(c);
                    self.advance();
                } else {
                    break;
                }
            }

            let token_type = match word.as_str() {
                "let" => TokenKind::Let,
                "const" => TokenKind::Const,
                "if" => TokenKind::If,
                "while" => TokenKind::While,
                "for" => TokenKind::For,
                _ => TokenKind::Identifier(word),
            };

            return Token {
                kind: token_type,
                span: Span { start, end: self.position },
            };
        }

        if ch.is_ascii_digit() {
            let mut num = String::new();
            let mut seen_dot = false;

            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    num.push(c);
                    self.advance();
                } else if c == '.' && !seen_dot {
                    seen_dot = true;
                    num.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        
            return Token {
                kind: TokenKind::Number(num.parse().unwrap()),
                span: Span { start, end: self.position },
            };
        }

        match ch {
            '+' => { self.advance(); Token { kind: TokenKind::Plus, span: Span { start, end: self.position } } }
            '-' => { self.advance(); Token { kind: TokenKind::Minus, span: Span { start, end: self.position } } }
            '*' => { self.advance(); Token { kind: TokenKind::Multiply, span: Span { start, end: self.position } } }
            '/' => { self.advance(); Token { kind: TokenKind::Divide, span: Span { start, end: self.position } } }

            '(' => { self.advance(); Token { kind: TokenKind::LPar, span: Span { start, end: self.position } } }
            ')' => { self.advance(); Token { kind: TokenKind::RPar, span: Span { start, end: self.position } } }

            '[' => { self.advance(); Token { kind: TokenKind::LSquare, span: Span { start, end: self.position } } }
            ']' => { self.advance(); Token { kind: TokenKind::RSquare, span: Span { start, end: self.position } } }

            '{' => { self.advance(); Token { kind: TokenKind::LCurly, span: Span { start, end: self.position } } }
            '}' => { self.advance(); Token { kind: TokenKind::RCurly, span: Span { start, end: self.position } } }

            ';' => { self.advance(); Token { kind: TokenKind::Semicolon, span: Span { start, end: self.position } } }
            ':' => { self.advance(); Token { kind: TokenKind::Colon, span: Span { start, end: self.position } } }

            '"' => { self.advance(); Token { kind: TokenKind::DQuote, span: Span { start, end: self.position } } }
            '\'' => { self.advance(); Token { kind: TokenKind::Quote, span: Span { start, end: self.position } } }

            '^' => { self.advance(); Token { kind: TokenKind::BXOR, span: Span { start, end: self.position } } }

            '=' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token { kind: TokenKind::Equal, span: Span { start, end: self.position } }
                } else {
                    Token { kind: TokenKind::Assign, span: Span { start, end: self.position } }
                }
            }

            '>' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token { kind: TokenKind::GTE, span: Span { start, end: self.position } }
                } else {
                    Token { kind: TokenKind::Greater, span: Span { start, end: self.position } }
                }
            }

            '<' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token { kind: TokenKind::LTE, span: Span { start, end: self.position } }
                } else {
                    Token { kind: TokenKind::Less, span: Span { start, end: self.position } }
                }
            }

            '!' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token { kind: TokenKind::NE, span: Span { start, end: self.position } }
                } else {
                    Token { kind: TokenKind::Not, span: Span { start, end: self.position } }
                }
            }

            '&' => {
                self.advance();
                if self.peek() == Some('&') {
                    self.advance();
                    Token { kind: TokenKind::And, span: Span { start, end: self.position } }
                } else {
                    Token { kind: TokenKind::BAND, span: Span { start, end: self.position } }
                }
            }

            '|' => {
                self.advance();
                if self.peek() == Some('|') {
                    self.advance();
                    Token { kind: TokenKind::Or, span: Span { start, end: self.position } }
                } else {
                    Token { kind: TokenKind::BOR, span: Span { start, end: self.position } }
                }
            }

            _ => {
                self.advance();
                Token { kind: TokenKind::Unknown, span: Span { start, end: self.position } }
            }
        }
    }
}