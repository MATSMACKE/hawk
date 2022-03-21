use crate::object::Object;
use crate::token::{Token, TokenType};
use crate::tree::{Expression, Statement};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize
}

impl Parser {
    /// Constructs a `Parser` and parses a vec of tokens
    pub fn parse(tokens: &Vec<Token>) -> Vec<Statement> {
        let mut parser = Parser{tokens: tokens.clone(), index: 0};
        let mut statements: Vec<Statement> = Vec::new();

        while !parser.at_end() {
            let start_line = parser.current().line;

            statements.push(parser.statement());

            let mut end_line = parser.current().line;
            while end_line > start_line {
                statements.push(Statement::Line);
                end_line -= 1;
            }
        }
        
        statements
    }

    fn statement(&mut self) -> Statement {
        self.consume();
        match self.previous().token_type {
            TokenType::Print => Statement::Print(self.expression()),
            TokenType::EOF => Statement::EOF,
            TokenType::Let => self.parse_let(),
            TokenType::If => self.parse_if(),
            TokenType::Loop => self.parse_loop(),
            TokenType::Break => Statement::Break,
            TokenType::While => self.parse_while_loop(),
            TokenType::BraceLeft => self.parse_block(),
            TokenType::Function => self.parse_function(),
            TokenType::Return => Statement::Return(self.expression()),
            TokenType::Import => Statement::Import(self.expression()),
            TokenType::Process => self.parse_process_block(),
            _ => self.parse_other()
        }
    }

    fn parse_other(&mut self) -> Statement {
        match self.current().token_type {
            TokenType::Assign => self.parse_assignment(),
            _ => {
                self.index -= 1;
                Statement::Expression(self.expression())
            }
        }
    }

    fn parse_assignment(&mut self) -> Statement {
        let name: String;
        if let Some(Object::Identifier(x)) = self.previous().literal {
            name = x
        } else {
            panic!("Expected variable name to be String")
        }
        self.consume();
        let value = self.expression();
        Statement::Definition { name, value }
    }

    fn parse_loop(&mut self) -> Statement {
        let block = Box::new(self.statement());
        Statement::Loop(block)
    }

    fn parse_let(&mut self) -> Statement {
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
        Statement::Definition { name, value }
    }

    fn parse_if(&mut self) -> Statement {
        let condition = self.expression();
        let block = Box::new(self.statement());

        if let TokenType::Else = self.current().token_type {
            self.consume();
            let else_block = Box::new(self.statement());
            Statement::IfElse { condition, if_block: block, else_block }
        } else {
            Statement::If { condition, block }
        }
    }

    fn parse_while_loop(&mut self) -> Statement {
        let condition = self.expression();
        let block = self.statement();
        Statement::While { condition, block: Box::new(block) }
    }

    fn parse_block(&mut self) -> Statement {
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

    fn parse_process_block(&mut self) -> Statement {
        let readfile = self.expression();
        let writefile = self.expression();

        let block = Box::new(self.statement());
        Statement::Process { readfile, writefile, block }
    }

    fn parse_function(&mut self) -> Statement {
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
                Statement::Function { identifier, params, block }
            } else {
                panic!("Expected parenthesis after function identifier")
            }
        } else {
            panic!("Functions need identifiers")
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
                  TokenType::Not 
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
                self.parse_functioncall()
            }
            else if let TokenType::BracketLeft = self.next().token_type {
                self.parse_array_index()
            }
            else if let TokenType::Dot = self.next().token_type {
                self.parse_methodcall()
            }
            else {
                self.parse_literal()
            }
        }
        else if let TokenType::ParenthesisLeft = self.current().token_type {
            self.parse_parenthesized()
        }
        else if let TokenType::BracketLeft = self.current().token_type {
            self.parse_array_literal()
        }
        else {
            Box::new(Expression::Literal(Object::Null))
        }
    }

    fn parse_array_literal(&mut self) -> Box<Expression> {
        self.consume();
        let mut items: Vec<Box<Expression>> = Vec::new();

        while self.previous().token_type != TokenType::BracketRight {
            items.push(self.expression());

            self.consume()
        }
        Box::new(Expression::Array(items))
    }

    fn parse_parenthesized(&mut self) -> Box<Expression> {
        self.consume();
        let expression = self.expression();
        
        if let TokenType::ParenthesisRight = self.current().token_type {
            self.consume();
        } else {
            panic!("Expected closing parenthesis, instead found {}", self.current())
        }

        expression
    }

    fn parse_literal(&mut self) -> Box<Expression> {
        if let Some(x) = self.current().literal {
            self.consume();
            Box::new(Expression::Literal(x))
        } else {
            panic!("Couldn't parse literal on line {}", self.current().line)
        }
    }

    fn parse_methodcall(&mut self) -> Box<Expression> {
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
                Box::new(Expression::MethodCall { object: identifier, method: methodname, args })
            } else {
                panic!("Method call needs method name")
            }
        } else {
            panic!("Couldn't get function parameters")
        }
    }

    fn parse_array_index(&mut self) -> Box<Expression> {
        if let Some(Object::Identifier(identifier)) = self.current().literal {
            self.consume();
            self.consume();
            let index = self.expression();
            self.consume();
            Box::new(Expression::ArrayIndex { identifier, index })
        } else {
            panic!("Couldn't get array index")
        }
    }

    fn parse_functioncall(&mut self) -> Box<Expression> {
        if let Some(Object::Identifier(identifier)) = self.current().literal {
            self.consume();
            self.consume();
            let mut args: Vec<Box<Expression>> = Vec::new();

            while self.previous().token_type != TokenType::ParenthesisRight {
                args.push(self.expression());

                self.consume()
            }
            Box::new(Expression::FunctionCall { identifier, args })
        } else {
            panic!("Couldn't get function parameters")
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