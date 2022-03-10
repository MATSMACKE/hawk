use core::panic;

use crate::token::{Token, TokenType};
use crate::tree::{Statement, Expression};
use crate::object::Object;

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
            TokenType::Loop => {
                let block = Box::new(self.statement());
                Statement::Loop(block)
            },
            TokenType::Break => {
                Statement::Break
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
            },
            TokenType::Function => {
                if let Some(Object::Identifier(identifier)) = self.current().literal {
                    self.consume();
                    if let TokenType::ParenthesisLeft = self.current().token_type {
                        self.consume();
                        let mut params: Vec<String> = Vec::new();
                        if let TokenType::ParenthesisRight = self.current().token_type {
                            self.consume();
                        } else {
                            while let TokenType::Comma | TokenType::ParenthesisRight = self.next().token_type {
                                if let Some(Object::Identifier(identifier)) = self.current().literal {
                                    params.push(identifier)
                                } else {
                                    panic!("Expected identifier as function parameter")
                                }
                                self.consume();
                                self.consume()
                            }
                        }
                        let block = Box::new(self.statement());
                        Statement::Function{identifier, params, block}
                    } else {
                        panic!("Expected parenthesis after function identifier")
                    }
                } else {
                    panic!("Functions need identifiers")
                }
            },
            TokenType::Return => Statement::Return(self.expression()),
            TokenType::Import => Statement::Import(self.expression()),
            TokenType::Process => {
                let readfile = self.expression();
                let writefile = self.expression();

                let block = Box::new(self.statement());
                Statement::Process{readfile, writefile, block}
            },
            _ => {
                match self.current().token_type {
                    TokenType::Assign => {
                        let name: String;
                        if let Some(Object::Identifier(x)) = self.previous().literal {
                            name = x
                        } else {
                            panic!("Expected variable name to be String")
                        }
                        self.consume();
                        let value = self.expression();
                        Statement::Definition{name, value}
                    }
                    _ => {
                        self.index -= 1;
                        Statement::Expression(self.expression())
                    }
                }
            }
        }
    }

    fn expression(&mut self) -> Box<Expression> {
        self.or()
    }

    fn or(&mut self) -> Box<Expression> {
        let mut temp = self.and();

        while let 
                  TokenType::Or = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.and();

            temp = Box::new(Expression::Binary{operand1: temp, operator, operand2});
        }

        temp
    }

    fn and(&mut self) -> Box<Expression> {
        let mut temp = self.equality();

        while let TokenType::And = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.equality();

            temp = Box::new(Expression::Binary{operand1: temp, operator, operand2});
        }

        temp
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
        let mut temp = self.power();

        while let 
                TokenType::Slash 
                | TokenType::Asterisk = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.power();

            temp = Box::new(Expression::Binary{operand1: temp, operator, operand2});
        }

        temp
    }

    fn power(&mut self) -> Box<Expression> {
        let mut temp = self.uncertainty();

        while let TokenType::Caret = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.uncertainty();

            temp = Box::new(Expression::Binary{operand1: temp, operator, operand2});
        }

        temp
    }

    fn uncertainty(&mut self) -> Box<Expression> {
        let mut temp = self.unary();

        while let TokenType::PlusMinus = self.current().token_type {
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
            if let TokenType::ParenthesisLeft = self.next().token_type {
                if let Some(Object::Identifier(identifier)) = self.current().literal {
                    self.consume();
                    self.consume();
                    let mut args: Vec<Box<Expression>> = Vec::new();

                    while self.previous().token_type != TokenType::ParenthesisRight {
                        
                        args.push(self.expression());

                        self.consume()
                    }
                    Box::new(Expression::FunctionCall{identifier, args})
                } else {
                    panic!("Couldn't get function parameters")
                }
            }
            else if let TokenType::BracketLeft = self.next().token_type {
                if let Some(Object::Identifier(identifier)) = self.current().literal {
                    self.consume();
                    self.consume();
                    let index = self.expression();
                    self.consume();
                    Box::new(Expression::ArrayIndex{identifier, index})
                } else {
                    panic!("Couldn't get function parameters")
                }
            }
            else if let TokenType::Dot = self.next().token_type {
                if let Some(Object::Identifier(identifier)) = self.current().literal {
                    if let Some(Object::Identifier(methodname)) = self.tokens[self.index + 2].literal.clone() {
                        self.consume();
                        self.consume();
                        self.consume();
                        let mut args: Vec<Box<Expression>> = Vec::new();

                        while self.previous().token_type != TokenType::ParenthesisRight {
                            
                            args.push(self.expression());

                            self.consume()
                        }
                        Box::new(Expression::MethodCall{object: identifier, method: methodname, args})
                    } else {
                        panic!("Method call needs method name")
                    }
                    
                } else {
                    panic!("Couldn't get function parameters")
                }
            }
            else {
                if let Some(x) = self.current().literal {
                    self.consume();
                    Box::new(Expression::Literal(x))
                } else {
                    panic!("Couldn't parse literal on line {}", self.current().line)
                }
            }
        } else if let TokenType::ParenthesisLeft = self.current().token_type {
            self.consume();
            let expression = self.equality();
            if let TokenType::ParenthesisRight = self.current().token_type {
                self.consume();
            }
            expression
        } else if let TokenType::BracketLeft = self.current().token_type {
            self.consume();
            let mut items: Vec<Box<Expression>> = Vec::new();

            while self.previous().token_type != TokenType::BracketRight {
                items.push(self.expression());

                self.consume()
            }
            Box::new(Expression::Array(items))
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

    fn next(&self) -> Token {
        self.tokens[self.index + 1].clone()
    }
    
    fn consume(&mut self) {
        self.index += 1;
    }
}