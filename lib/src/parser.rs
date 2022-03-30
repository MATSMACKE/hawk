use std::collections::HashMap;

use hawk_common::object::Object;
use hawk_common::token::{Token, TokenType};
use hawk_common::tree::{Expression, Statement};

use hawk_common::token::UserPrint;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    warn: fn(String, usize) -> ()
}

impl Parser {
    /// Constructs a `Parser` and parses a vec of tokens
    pub fn parse(tokens: &Vec<Token>, warn: fn(String, usize) -> ()) -> Result<Vec<Statement>, (String, usize)> {
        let mut parser = Parser {
            tokens: tokens.clone(),
            index: 0,
            warn
        };
        let mut statements: Vec<Statement> = Vec::new();

        while !parser.at_end() {
            let start_line = parser.current().line;

            statements.push(parser.statement()?);

            let mut end_line = parser.current().line;
            while end_line > start_line {
                statements.push(Statement::Line);
                end_line -= 1;
            }
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, (String, usize)> {
        self.consume();
        match self.previous().token_type {
            TokenType::Print => Ok(Statement::Print(self.expression()?)),
            TokenType::EOF => Ok(Statement::EOF),
            TokenType::Let => self.parse_let(),
            TokenType::If => self.parse_if(),
            TokenType::Loop => self.parse_loop(),
            TokenType::Break => Ok(Statement::Break),
            TokenType::While => self.parse_while_loop(),
            TokenType::BraceLeft => self.parse_block(),
            TokenType::Function => self.parse_function(),
            TokenType::Return => Ok(Statement::Return(self.expression()?)),
            TokenType::Import => Ok(Statement::Import(self.expression()?)),
            TokenType::Process => self.parse_process_block(),
            TokenType::Finder => self.parse_finder(),
            _ => self.parse_other(),
        }
    }

    fn parse_other(&mut self) -> Result<Statement, (String, usize)> {
        match self.current().token_type {
            TokenType::Assign => self.parse_assignment(),
            TokenType::BracketLeft => self.parse_array_assign(),
            _ => {
                self.index -= 1;
                Ok(Statement::Expression(self.expression()?))
            }
        }
    }

    fn parse_finder(&mut self) -> Result<Statement, (String, usize)> {
        if let Some(Object::Identifier(identifier)) = self.current().literal {
            self.consume();
            let mut equations: Vec<(Expression, Expression)> = Vec::new();

            if let TokenType::BraceLeft = self.current().token_type {
                self.consume();
                while !(self.current().token_type == TokenType::BraceRight) {
                    if !self.expect(TokenType::Equation) {
                        return Err(("Expected 'equation'".to_string(), self.current().line));
                    }
                    let lhs = self.expression()?;
                    if !self.expect(TokenType::Assign) {
                        return Err(("Expected '='".to_string(), self.current().line));
                    }
                    let rhs = self.expression()?;
                    equations.push((*lhs, *rhs));
                }
                self.consume();
            } else {
                return Err((
                    "Expected curly brace after ".to_string(),
                    self.current().line,
                ));
            }
            Ok(Statement::Finder {
                identifier,
                equations,
            })
        } else {
            Err((format!("Finder needs identifier"), self.current().line))
        }
    }

    fn parse_assignment(&mut self) -> Result<Statement, (String, usize)> {
        let name: String;
        if let Some(Object::Identifier(x)) = self.previous().literal {
            name = x
        } else {
            return Err((
                format!(
                    "Expected identifier as left hand side of assignment, found {}",
                    self.previous()
                        .literal
                        .unwrap()
                        .user_print(self.previous().line)?
                ),
                self.previous().line,
            ));
        }
        self.consume();
        let value = self.expression()?;
        Ok(Statement::Definition { name, value })
    }

    fn parse_array_assign(&mut self) -> Result<Statement, (String, usize)> {
        let name: String;
        if let Some(Object::Identifier(x)) = self.previous().literal {
            name = x
        } else {
            return Err((
                format!(
                    "Expected identifier for array assignment, found {}",
                    self.previous()
                        .literal
                        .unwrap()
                        .user_print(self.previous().line)?
                ),
                self.previous().line,
            ));
        }
        self.consume();
        let idx = self.expression()?;
        self.consume();
        self.consume();
        let value = self.expression()?;

        Ok(Statement::ArrayAssign { name, idx, value })
    }

    fn parse_loop(&mut self) -> Result<Statement, (String, usize)> {
        let block = Box::new(self.statement()?);
        Ok(Statement::Loop(block))
    }

    fn parse_let(&mut self) -> Result<Statement, (String, usize)> {
        let name: String;
        if let Some(Object::Identifier(x)) = self.current().literal {
            name = x
        } else {
            return Err((
                format!(
                    "Expected identifier after 'let', found {}",
                    self.previous()
                        .literal
                        .unwrap()
                        .user_print(self.previous().line)?
                ),
                self.previous().line,
            ));
        }
        self.consume();
        let value: Box<Expression>;
        if let TokenType::Assign = self.current().token_type {
            self.consume();
            value = self.expression()?;
        } else {
            value = Box::new(Expression::Literal(Object::Null));
        }
        Ok(Statement::Definition { name, value })
    }

    fn parse_if(&mut self) -> Result<Statement, (String, usize)> {
        let condition = self.expression()?;
        let block = Box::new(self.statement()?);

        if let TokenType::Else = self.current().token_type {
            self.consume();
            let else_block = Box::new(self.statement()?);
            Ok(Statement::IfElse {
                condition,
                if_block: block,
                else_block,
            })
        } else {
            Ok(Statement::If { condition, block })
        }
    }

    fn parse_while_loop(&mut self) -> Result<Statement, (String, usize)> {
        let condition = self.expression()?;
        let block = self.statement()?;
        Ok(Statement::While {
            condition,
            block: Box::new(block),
        })
    }

    fn parse_block(&mut self) -> Result<Statement, (String, usize)> {
        let mut block: Vec<Statement> = Vec::new();
        let mut in_block = true;

        while in_block {
            if let TokenType::BraceRight = self.current().token_type {
                in_block = false;
                self.consume();
            } else {
                block.push(self.statement()?)
            }
        }

        Ok(Statement::Block(block))
    }

    fn parse_process_block(&mut self) -> Result<Statement, (String, usize)> {
        let readfile = self.expression()?;
        let writefile = self.expression()?;

        let block = Box::new(self.statement()?);
        Ok(Statement::Process {
            readfile,
            writefile,
            block,
        })
    }

    fn parse_function(&mut self) -> Result<Statement, (String, usize)> {
        if let Some(Object::Identifier(identifier)) = self.current().literal {
            self.consume();
            if let TokenType::ParenthesisLeft = self.current().token_type {
                self.consume();
                let mut params: Vec<String> = Vec::new();
                if let TokenType::ParenthesisRight = self.current().token_type {
                    self.consume();
                } else {
                    while let TokenType::Comma | TokenType::ParenthesisRight =
                        self.next().token_type
                    {
                        if let Some(Object::Identifier(identifier)) = self.current().literal {
                            params.push(identifier)
                        } else {
                            return Err((
                                format!(
                                    "Expected identifier as function parameter, found {}",
                                    self.current()
                                        .literal
                                        .unwrap()
                                        .user_print(self.current().line)?
                                ),
                                self.current().line,
                            ));
                        }
                        self.consume();
                        self.consume()
                    }
                }
                let block = Box::new(self.statement()?);
                Ok(Statement::Function {
                    identifier,
                    params,
                    block,
                })
            } else {
                Err((
                    format!("Expected parentheses after function identifier"),
                    self.current().line,
                ))
            }
        } else {
            Err((format!("Function needs identifier"), self.current().line))
        }
    }

    fn expression(&mut self) -> Result<Box<Expression>, (String, usize)> {
        self.or()
    }

    fn or(&mut self) -> Result<Box<Expression>, (String, usize)> {
        let mut temp = self.and()?;

        while let TokenType::Or = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.and()?;

            temp = Box::new(Expression::Binary {
                operand1: temp,
                operator,
                operand2,
            });
        }

        Ok(temp)
    }

    fn and(&mut self) -> Result<Box<Expression>, (String, usize)> {
        let mut temp = self.equality()?;

        while let TokenType::And = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.equality()?;

            temp = Box::new(Expression::Binary {
                operand1: temp,
                operator,
                operand2,
            });
        }

        Ok(temp)
    }

    fn equality(&mut self) -> Result<Box<Expression>, (String, usize)> {
        let mut temp = self.comparison()?;

        while let TokenType::EqualEqual | TokenType::NotEqual = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.comparison()?;

            temp = Box::new(Expression::Binary {
                operand1: temp,
                operator,
                operand2,
            });
        }

        Ok(temp)
    }

    fn comparison(&mut self) -> Result<Box<Expression>, (String, usize)> {
        let mut temp = self.term()?;

        while let TokenType::LessThan
        | TokenType::LessThanEqual
        | TokenType::GreaterThan
        | TokenType::GreaterThanEqual = self.current().token_type
        {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.term()?;

            temp = Box::new(Expression::Binary {
                operand1: temp,
                operator,
                operand2,
            });
        }

        Ok(temp)
    }

    fn term(&mut self) -> Result<Box<Expression>, (String, usize)> {
        let mut temp = self.factor()?;

        while let TokenType::Minus | TokenType::Plus = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.factor()?;

            temp = Box::new(Expression::Binary {
                operand1: temp,
                operator,
                operand2,
            });
        }

        Ok(temp)
    }

    fn factor(&mut self) -> Result<Box<Expression>, (String, usize)> {
        let mut temp = self.power()?;

        while let TokenType::Slash | TokenType::Asterisk = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.power()?;

            temp = Box::new(Expression::Binary {
                operand1: temp,
                operator,
                operand2,
            });
        }

        Ok(temp)
    }

    fn power(&mut self) -> Result<Box<Expression>, (String, usize)> {
        let mut temp = self.uncertainty()?;

        while let TokenType::Caret = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.uncertainty()?;

            temp = Box::new(Expression::Binary {
                operand1: temp,
                operator,
                operand2,
            });
        }

        Ok(temp)
    }

    fn uncertainty(&mut self) -> Result<Box<Expression>, (String, usize)> {
        let mut temp = self.unary()?;

        while let TokenType::PlusMinus = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand2 = self.unary()?;

            temp = Box::new(Expression::Binary {
                operand1: temp,
                operator,
                operand2,
            });
        }

        Ok(temp)
    }

    fn unary(&mut self) -> Result<Box<Expression>, (String, usize)> {
        if let TokenType::Not | TokenType::Minus = self.current().token_type {
            let operator = self.current().token_type;
            self.consume();
            let operand = self.unary()?;
            Ok(Box::new(Expression::Unary { operator, operand }))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Box<Expression>, (String, usize)> {
        if let TokenType::Int
        | TokenType::Decimal
        | TokenType::True
        | TokenType::False
        | TokenType::String
        | TokenType::Identifier = self.current().token_type
        {
            if let TokenType::ParenthesisLeft = self.next().token_type {
                self.parse_functioncall()
            } else if let TokenType::BracketLeft = self.next().token_type {
                self.parse_array_index()
            } else if let TokenType::Dot = self.next().token_type {
                self.parse_methodcall()
            } else {
                self.parse_literal()
            }
        } else if let TokenType::ParenthesisLeft = self.current().token_type {
            self.parse_parenthesized()
        } else if let TokenType::BracketLeft = self.current().token_type {
            self.parse_array_literal()
        } else if let TokenType::Find = self.current().token_type {
            self.parse_finder_call()
        } else {
            (self.warn)(
                format!(
                    "Unexpected token '{}'",
                    self.current().token_type.user_print()
                ),
                self.current().line,
            );
            self.consume();
            Ok(Box::new(Expression::Literal(Object::Null)))
        }
    }

    fn parse_array_literal(&mut self) -> Result<Box<Expression>, (String, usize)> {
        self.consume();
        let mut items: Vec<Box<Expression>> = Vec::new();

        while self.previous().token_type != TokenType::BracketRight {
            items.push(self.expression()?);

            self.consume()
        }
        Ok(Box::new(Expression::Array(items)))
    }

    fn parse_parenthesized(&mut self) -> Result<Box<Expression>, (String, usize)> {
        self.consume();
        let expression = self.expression()?;

        if let TokenType::ParenthesisRight = self.current().token_type {
            self.consume();
        } else {
            return Err((
                format!(
                    "Expected closing parenthesis, instead found {}",
                    self.current().token_type
                ),
                self.current().line,
            ));
        }

        Ok(expression)
    }

    fn parse_literal(&mut self) -> Result<Box<Expression>, (String, usize)> {
        if let Some(x) = self.current().literal {
            self.consume();
            Ok(Box::new(Expression::Literal(x)))
        } else {
            Err((format!("Couldn't parse literal"), self.current().line))
        }
    }

    fn parse_methodcall(&mut self) -> Result<Box<Expression>, (String, usize)> {
        if let Some(Object::Identifier(identifier)) = self.current().literal {
            if let Some(Object::Identifier(methodname)) =
                self.tokens[self.index + 2].literal.clone()
            {
                self.consume();
                self.consume();
                self.consume();
                let mut args: Vec<Box<Expression>> = Vec::new();

                while self.previous().token_type != TokenType::ParenthesisRight {
                    args.push(self.expression()?);

                    self.consume()
                }
                Ok(Box::new(Expression::MethodCall {
                    object: identifier,
                    method: methodname,
                    args,
                }))
            } else {
                Err((
                    format!(
                        "Expected method name, instead found {}",
                        self.tokens[self.index + 2]
                            .literal
                            .clone()
                            .unwrap()
                            .user_print(self.current().line)?
                    ),
                    self.current().line,
                ))
            }
        } else {
            Err((
                format!("Expected object identifier, found {}", self.current()),
                self.current().line,
            ))
        }
    }

    fn parse_array_index(&mut self) -> Result<Box<Expression>, (String, usize)> {
        if let Some(Object::Identifier(identifier)) = self.current().literal {
            self.consume();
            self.consume();
            let index = self.expression()?;
            if !(TokenType::BracketRight == self.next().token_type) {
                (self.warn)(format!("Hawk respects your freedom, so using {} is fine, but consider using a ']' to end array index.", self.current().token_type.user_print()), self.current().line);
            }
            self.consume();
            Ok(Box::new(Expression::ArrayIndex { identifier, index }))
        } else {
            Err((format!("Couldn't get array index"), self.current().line))
        }
    }

    fn parse_finder_call(&mut self) -> Result<Box<Expression>, (String, usize)> {
        self.consume();
        if let Some(Object::Identifier(identifier)) = self.current().literal {
            self.consume();
            self.consume();

            let mut given: HashMap<String, Expression> = HashMap::new();
            let mut to_find = String::new();

            while self.previous().token_type != TokenType::ParenthesisRight {
                if let Some(Object::Identifier(var)) = self.current().literal {
                    if self.next().token_type == TokenType::QuestionMark {
                        to_find = var;
                        self.consume();
                        self.consume();
                    } else {
                        self.consume();
                        self.consume();
                        let expr = self.expression()?;
                        given.insert(var, *expr);
                    }
                }

                self.consume();
            }

            Ok(Box::new(Expression::FinderCall {
                identifier,
                given,
                to_find,
            }))
        } else {
            Err((
                format!("Couldn't get finder parameters"),
                self.current().line,
            ))
        }
    }

    fn parse_functioncall(&mut self) -> Result<Box<Expression>, (String, usize)> {
        if let Some(Object::Identifier(identifier)) = self.current().literal {
            self.consume();
            self.consume();
            let mut args: Vec<Box<Expression>> = Vec::new();

            if !(self.current().token_type == TokenType::ParenthesisRight) {
                while self.previous().token_type != TokenType::ParenthesisRight {
                    args.push(self.expression()?);

                    self.consume()
                }
            } else {
                self.consume()
            }
            Ok(Box::new(Expression::FunctionCall { identifier, args }))
        } else {
            Err((
                format!("Couldn't get function parameters"),
                self.current().line,
            ))
        }
    }

    pub fn at_end(&self) -> bool {
        self.index.saturating_sub(1) == self.tokens.len()
            || self.current().token_type == TokenType::EOF
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

    fn expect(&mut self, ttype: TokenType) -> bool {
        if ttype == self.current().token_type {
            self.consume();
            true
        } else {
            self.consume();
            false
        }
    }
}
