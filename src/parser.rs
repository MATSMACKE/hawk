use core::panic;

use crate::token::{Token, TokenType, Object};
use crate::tree::{Statement, Expression};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize
}

impl Parser {
    pub fn parse(tokens: &Vec<Token>) -> Vec<Statement> {
        let mut parser = Parser{tokens: tokens.clone(), index: 0};
        let mut statements: Vec<Statement> = Vec::new();

        while !parser.at_end() {
            statements.push(parser.statement())
        }

        println!("{:?}\n\n", statements);
        statements
    }

    fn statement(&mut self) -> Statement {
        self.consume();
        match self.previous().token_type {
            TokenType::Print => Statement::Print(self.expression()),
            TokenType::EOF => Statement::EOF,
            TokenType::Let => {
                let name: String;
                if let Some(Object::Identifier(x)) = self.current().literal {
                    name = x
                } else {
                    panic!("Expected variable name to be String");
                }
                self.consume();
                let value: Box<Expression>;
                if let TokenType::Assign = self.current().token_type {
                    self.consume();
                    value = self.expression();
                } else {
                    value = Box::new(Expression::Literal(Object::Null));
                }
                Statement::Definition{name, value}
            },
            TokenType::If => {
                let condition = self.expression();
                let block = Box::new(self.statement());
                if let TokenType::Else = self.current().token_type {
                    self.consume();
                    let else_block = Box::new(self.statement());
                    Statement::IfElse{condition, if_block: block, else_block}
                } else {
                    Statement::If{condition, block}
                }
            },
            TokenType::While => {
                let condition = self.expression();
                let block = self.statement();
                Statement::While{condition, block: Box::new(block)}
            },
            TokenType::BraceLeft => {
                let mut block: Vec<Statement> = Vec::new();
                let mut in_block = true;
                while in_block {
                    if let TokenType::BraceRight = self.current().token_type {
                        in_block = false;
                        self.consume();
                    } else {
                        block.push(self.statement())
                    }
                }
                Statement::Block(block)
            }
            _ => {
                match self.current().token_type {
                    TokenType::Assign => {
                        let name: String;
                        if let Some(Object::Identifier(x)) = self.previous().literal {
                            name = x
                        } else {
                            panic!("Expected variable name to be String");
                        }
                        self.consume();
                        let value = self.expression();
                        Statement::Definition{name, value}
                    }
                    _ => Statement::Expression(self.expression())
                }
            }
        }
    }

    fn expression(&mut self) -> Box<Expression> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expression> {
        let mut temp = self.comparison();

        while let 
                  TokenType::EqualEqual 
                | TokenType::NotEqual = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.comparison();

            temp = Box::new(Expression::Binary{operand1: temp, operator, operand2});
        }

        temp
    }

    fn comparison(&mut self) -> Box<Expression> {
        let mut temp = self.term();

        while let 
                  TokenType::LessThan 
                | TokenType::LessThanEqual 
                | TokenType::GreaterThan 
                | TokenType::GreaterThanEqual = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.term();

            temp = Box::new(Expression::Binary{operand1: temp, operator, operand2});
        }

        temp
    }

    fn term(&mut self) -> Box<Expression> {
        let mut temp = self.factor();

        while let 
                TokenType::Minus 
                | TokenType::Plus = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.factor();

            temp = Box::new(Expression::Binary{operand1: temp, operator, operand2});
        }

        temp
    }

    fn factor(&mut self) -> Box<Expression> {
        let mut temp = self.unary();

        while let 
                TokenType::Slash 
                | TokenType::Asterisk = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.unary();

            temp = Box::new(Expression::Binary{operand1: temp, operator, operand2});
        }

        temp
    }

    fn unary(&mut self) -> Box<Expression> {
        if let 
                  TokenType::ExclamationMark 
                | TokenType::Minus = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand = self.unary();
            Box::new(Expression::Unary{operator, operand})
        }
        else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<Expression> {
        if let 
              TokenType::Int 
            | TokenType::Float 
            | TokenType::True 
            | TokenType::False
            | TokenType::String
            | TokenType::Identifier 
                = self.current().token_type {

            if let Some(x) = self.current().literal {
                self.consume();
                Box::new(Expression::Literal(x))
            } else {
                panic!("Couldn't parse literal on line {}", self.current().line)
            }
        } else if let TokenType::ParenthesisLeft = self.current().token_type {
            self.consume();
            let expression = self.equality();
            if let TokenType::ParenthesisRight = self.current().token_type {
                self.consume();
            }
            expression
        }

        else {
            Box::new(Expression::Literal(Object::Null))
        }
    }

    pub fn at_end(&self) -> bool {
        self.index.saturating_sub(1) == self.tokens.len() || self.current().token_type == TokenType::EOF
    }

    fn current(&self) -> Token {
        self.tokens[self.index].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.index - 1].clone()
    }
    
    fn consume(&mut self) {
        self.index += 1;
    }
}