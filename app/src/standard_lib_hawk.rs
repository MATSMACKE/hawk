use crate::{Object, token::TokenType, tree::{Expression, Statement}};

pub fn get_std_hawk_fn(identifier: String) -> Option<Object> {
match identifier.as_str() {


"factorial" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::LessThanEqual}), if_block: Box::new(Statement::Return(Box::new(Expression::Literal(Object::Int(1))))), else_block: Box::new(Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::FunctionCall{identifier: "factorial".to_string(), args: vec![Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus})
]}), operator: TokenType::Asterisk})))},
]))}),

"mod" => Some(Object::Function{params: vec!["dividend", "divisor"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("divisor".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(0))), operator: TokenType::EqualEqual}), if_block: Box::new(Statement::Return(Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}))), else_block: Box::new(Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("divisor".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("dividend".to_string()))), operator: TokenType::GreaterThan}), if_block: Box::new(Statement::Return(Box::new(Expression::Literal(Object::Identifier("dividend".to_string()))))), else_block: Box::new(Statement::Block(vec![Statement::Definition{name: "product".to_string(), value: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("dividend".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("divisor".to_string()))), operator: TokenType::Slash})},
Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("dividend".to_string()))), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("product".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("divisor".to_string()))), operator: TokenType::Asterisk}), operator: TokenType::Minus})),
]))})},
]))}),


_ => None
}
}