#![allow(dead_code)]
pub enum TokenType {
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
    pub token_type: TokenType,
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
    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
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

    if start >= self.input.len() {
        return Token {
            token_type: TokenType::EOF,
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
            "let" => TokenType::Let,
            "const" => TokenType::Const,
            "if" => TokenType::If,
            "while" => TokenType::While,
            "for" => TokenType::For,
            _ => TokenType::Identifier(word),
        };

        return Token {
            token_type,
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
        token_type: TokenType::Number(num.parse().unwrap()),
        span: Span { start, end: self.position },
    };
}

    match ch {
        '+' => { self.advance(); Token { token_type: TokenType::Plus, span: Span { start, end: self.position } } }
        '-' => { self.advance(); Token { token_type: TokenType::Minus, span: Span { start, end: self.position } } }
        '*' => { self.advance(); Token { token_type: TokenType::Multiply, span: Span { start, end: self.position } } }
        '/' => { self.advance(); Token { token_type: TokenType::Divide, span: Span { start, end: self.position } } }

        '(' => { self.advance(); Token { token_type: TokenType::LPar, span: Span { start, end: self.position } } }
        ')' => { self.advance(); Token { token_type: TokenType::RPar, span: Span { start, end: self.position } } }

        '[' => { self.advance(); Token { token_type: TokenType::LSquare, span: Span { start, end: self.position } } }
        ']' => { self.advance(); Token { token_type: TokenType::RSquare, span: Span { start, end: self.position } } }

        '{' => { self.advance(); Token { token_type: TokenType::LCurly, span: Span { start, end: self.position } } }
        '}' => { self.advance(); Token { token_type: TokenType::RCurly, span: Span { start, end: self.position } } }

        ';' => { self.advance(); Token { token_type: TokenType::Semicolon, span: Span { start, end: self.position } } }
        ':' => { self.advance(); Token { token_type: TokenType::Colon, span: Span { start, end: self.position } } }

        '"' => { self.advance(); Token { token_type: TokenType::DQuote, span: Span { start, end: self.position } } }
        '\'' => { self.advance(); Token { token_type: TokenType::Quote, span: Span { start, end: self.position } } }

        '^' => { self.advance(); Token { token_type: TokenType::BXOR, span: Span { start, end: self.position } } }

        '=' => {
            self.advance();
            if self.peek() == Some('=') {
                self.advance();
                Token { token_type: TokenType::Equal, span: Span { start, end: self.position } }
            } else {
                Token { token_type: TokenType::Assign, span: Span { start, end: self.position } }
            }
        }

        '>' => {
            self.advance();
            if self.peek() == Some('=') {
                self.advance();
                Token { token_type: TokenType::GTE, span: Span { start, end: self.position } }
            } else {
                Token { token_type: TokenType::Greater, span: Span { start, end: self.position } }
            }
        }

        '<' => {
            self.advance();
            if self.peek() == Some('=') {
                self.advance();
                Token { token_type: TokenType::LTE, span: Span { start, end: self.position } }
            } else {
                Token { token_type: TokenType::Less, span: Span { start, end: self.position } }
            }
        }

        '!' => {
            self.advance();
            if self.peek() == Some('=') {
                self.advance();
                Token { token_type: TokenType::NE, span: Span { start, end: self.position } }
            } else {
                Token { token_type: TokenType::Not, span: Span { start, end: self.position } }
            }
        }

        '&' => {
            self.advance();
            if self.peek() == Some('&') {
                self.advance();
                Token { token_type: TokenType::And, span: Span { start, end: self.position } }
            } else {
                Token { token_type: TokenType::BAND, span: Span { start, end: self.position } }
            }
        }

        '|' => {
            self.advance();
            if self.peek() == Some('|') {
                self.advance();
                Token { token_type: TokenType::Or, span: Span { start, end: self.position } }
            } else {
                Token { token_type: TokenType::BOR, span: Span { start, end: self.position } }
            }
        }

        _ => {
            self.advance();
            Token { token_type: TokenType::Unknown, span: Span { start, end: self.position } }
        }
    }
}
}