#![allow(dead_code)]
use crate::larp::lexer::{Token};

pub enum Expression {
    Number(f64),
    Binary {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>
    },
    Unary {
        op: Token,
        expr: Box<Expression>
    },
    Variable(String)
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, position: 0 }
    }
    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }
    fn advance(&mut self) {
        self.position += 1;
    }
    fn peek(&self, offset: usize) -> Option<&Token> {
        if self.position + offset < self.tokens.len() {
            Some(&self.tokens[self.position+offset])
        } else {
            None
        }
    }
    pub fn parse(&mut self) -> Expression {
        self.parse_expression()
    }
    fn parse_expression(&mut self) -> Expression {
        todo!()
    }
    fn parse_term(&mut self) -> Expression {
        todo!()
    }
    fn parse_factor(&mut self) -> Expression {
        todo!()
    }
}