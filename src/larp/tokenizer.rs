
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Identifier,
    Number,
    Operator,
    End
}
#[allow(unused)]
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    input.split_whitespace().map(|s| {
        if s.chars().all(|c| c.is_alphabetic() || c == '_') {
            Token { token_type: TokenType::Identifier, value: s.to_string() }
        } else if s.chars().all(|c| c.is_ascii_digit()) {
            Token { token_type: TokenType::Number, value: s.to_string() }
        } else if s==";" {
            Token { token_type: TokenType::End, value: s.to_string() }
        } else {
            Token { token_type: TokenType::Operator, value: s.to_string() }
        }
    }).collect()
}