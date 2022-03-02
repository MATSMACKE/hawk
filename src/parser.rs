use core::panic;

use crate::token::{Token, TokenType, Object};
use crate::tree::{Expression, Binary, Unary, Literal};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    current: Token
}

impl Parser {
    pub fn parse(tokens: &Vec<Token>) -> Box<Expression> {
        let mut parser = Parser{tokens: tokens.clone(), index: 0, current: tokens[0].clone()};
        parser.equality()
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.consume();
            return true
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.next().token_type == token_type
    }

    fn next(&self) -> Token {
        self.tokens[self.index + 1].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.index - 1].clone()
    }

    fn consume(&mut self) {
        self.index = self.index + 1;
        self.current = self.previous()
    }

    fn at_end(&self) -> bool {
        self.next().token_type == TokenType::EOF
    }

    fn equality(&mut self) -> Box<Expression> {
        let mut temp = self.comparison();

        while self.match_token(TokenType::EqualEqual) || self.match_token(TokenType::NotEqual) {
            let operator = self.previous().token_type;
            let operand2 = self.comparison();
            temp = Box::new(Expression::Binary(Binary{operand1: temp.clone(), operand2, operator}));
        }

        temp
    }

    fn comparison(&mut self) -> Box<Expression> {
        let mut temp = self.term();

        while self.match_token(TokenType::EqualEqual) || self.match_token(TokenType::NotEqual) {
            let operator = self.previous().token_type;
            let operand2 = self.term();
            temp = Box::new(Expression::Binary(Binary{operand1: temp, operand2, operator}));
        }

        temp
    }

    fn term(&mut self) -> Box<Expression> {
        let mut temp = self.factor();

        while self.match_token(TokenType::Minus) || self.match_token(TokenType::Plus) {
            let operator = self.previous().token_type;
            let operand2 = self.factor();
            temp = Box::new(Expression::Binary(Binary{operand1: temp, operand2, operator}));
        }

        temp
    }

    fn factor(&mut self) -> Box<Expression> {
        let mut temp = self.unary();

        while self.match_token(TokenType::Slash) || self.match_token(TokenType::Asterisk) {
            let operator = self.previous().token_type;
            let operand2 = self.unary();
            temp = Box::new(Expression::Binary(Binary{operand1: temp, operand2, operator}));
        }

        temp
    }

    fn unary(&mut self) -> Box<Expression> {
        if self.match_token(TokenType::ExclamationMark) || self.match_token(TokenType::Minus) {
            let operator = self.previous().token_type;
            let operand = self.unary();
            return Box::new(Expression::Unary(Unary{operand, operator}))
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<Expression> {
        if self.match_token(TokenType::False) || self.match_token(TokenType::True) {
            if let Some(Object::Boolean(x)) = self.current.literal {
                return Box::new(Expression::Literal(Literal::Boolean(x)))
            }
            else {
                panic!("Couldn't parse Boolean on line {}", self.current.line);
            }
        }
        else if self.match_token(TokenType::Null) {
            Box::new(Expression::Literal(Literal::Null))
        }
        else {
            Box::new(Expression::Literal(Literal::Null))
        }
    }
}