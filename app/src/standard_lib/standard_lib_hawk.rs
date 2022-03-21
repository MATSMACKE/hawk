use crate::{Object, token::TokenType, tree::{Expression, Statement}};

pub fn get_std_hawk_fn(identifier: String) -> Option<Object> {
match identifier.as_str() {





"mod" => Some(Object::Function{params: vec!["dividend", "divisor"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("divisor".to_owned()))), operand2: Box::new(Expression::Literal(Object::Int(0))), operator: TokenType::EqualEqual}), if_block: Box::new(Statement::Return(Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}))), else_block: Box::new(Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("divisor".to_owned()))), operand2: Box::new(Expression::Literal(Object::Identifier("dividend".to_owned()))), operator: TokenType::GreaterThan}), if_block: Box::new(Statement::Return(Box::new(Expression::Literal(Object::Identifier("dividend".to_owned()))))), else_block: Box::new(Statement::Block(vec![Statement::Definition{name: "product".to_owned(), value: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("dividend".to_owned()))), operand2: Box::new(Expression::Literal(Object::Identifier("divisor".to_owned()))), operator: TokenType::Slash})},
Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("dividend".to_owned()))), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("product".to_owned()))), operand2: Box::new(Expression::Literal(Object::Identifier("divisor".to_owned()))), operator: TokenType::Asterisk}), operator: TokenType::Minus})),
]))})},
]))}),

"gcd" => Some(Object::Function{params: vec!["x", "y"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_owned()))), operator: TokenType::EqualEqual}), if_block: Box::new(Statement::Return(Box::new(Expression::Literal(Object::Identifier("x".to_owned()))))), else_block: Box::new(Statement::Block(vec![Statement::Definition{name: "r".to_owned(), value: Box::new(Expression::Literal(Object::Int(0)))},
Statement::While{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("y".to_owned()))), operand2: Box::new(Expression::Literal(Object::Int(0))), operator: TokenType::NotEqual}), block: Box::new(Statement::Block(vec![Statement::Definition{name: "r".to_owned(), value: Box::new(Expression::FunctionCall{identifier: "mod".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
Box::new(Expression::Literal(Object::Identifier("y".to_owned()))),
]})},
Statement::Definition{name: "x".to_owned(), value: Box::new(Expression::Literal(Object::Identifier("y".to_owned())))},
Statement::Definition{name: "y".to_owned(), value: Box::new(Expression::Literal(Object::Identifier("r".to_owned())))},
]))},
Statement::Return(Box::new(Expression::Literal(Object::Identifier("x".to_owned())))),
]))},
]))}),

"lcm" => Some(Object::Function{params: vec!["x", "y"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_owned()))), operator: TokenType::Asterisk}), operand2: Box::new(Expression::FunctionCall{identifier: "gcd".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
Box::new(Expression::Literal(Object::Identifier("y".to_owned()))),
]}), operator: TokenType::Slash})))}),


"cos" => Some(Object::Function{params: vec!["x"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::FunctionCall{identifier: "sin".to_owned(), args: vec![Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "pi".to_owned(), args: vec![Box::new(Expression::Literal(Object::Null)),
]}), operand2: Box::new(Expression::Literal(Object::Int(2))), operator: TokenType::Slash}), operator: TokenType::Plus}),
]})),
]))}),

"tan" => Some(Object::Function{params: vec!["x"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "sin".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "cos".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
]}), operator: TokenType::Slash})),
]))}),

"csc" => Some(Object::Function{params: vec!["x"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Int(1))), operand2: Box::new(Expression::FunctionCall{identifier: "sin".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
]}), operator: TokenType::Slash})),
]))}),

"sec" => Some(Object::Function{params: vec!["x"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Int(1))), operand2: Box::new(Expression::FunctionCall{identifier: "cos".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
]}), operator: TokenType::Slash})),
]))}),

"cot" => Some(Object::Function{params: vec!["x"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Int(1))), operand2: Box::new(Expression::FunctionCall{identifier: "tan".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
]}), operator: TokenType::Slash})),
]))}),

"sinh" => Some(Object::Function{params: vec!["x"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "eNum".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "eNum".to_owned(), args: vec![Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operator: TokenType::Minus}),
]}), operator: TokenType::Minus}), operand2: Box::new(Expression::Literal(Object::Int(2))), operator: TokenType::Slash})),
]))}),

"cosh" => Some(Object::Function{params: vec!["x"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "eNum".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "eNum".to_owned(), args: vec![Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operator: TokenType::Minus}),
]}), operator: TokenType::Plus}), operand2: Box::new(Expression::Literal(Object::Int(2))), operator: TokenType::Slash})),
]))}),

"tanh" => Some(Object::Function{params: vec!["x"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "sinh".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "cosh".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
]}), operator: TokenType::Slash})),
]))}),


"factorial" => Some(Object::Function{params: vec!["x"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operand2: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::LessThanEqual}), if_block: Box::new(Statement::Return(Box::new(Expression::Literal(Object::Int(1))))), else_block: Box::new(Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operand2: Box::new(Expression::FunctionCall{identifier: "factorial".to_owned(), args: vec![Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operand2: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}),
]}), operator: TokenType::Asterisk})))},
]))}),

"permutation" => Some(Object::Function{params: vec!["x", "y"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_owned()))), operator: TokenType::LessThan}), if_block: Box::new(Statement::Return(Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}))), else_block: Box::new(Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "factorial".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "factorial".to_owned(), args: vec![Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_owned()))), operator: TokenType::Minus}),
]}), operator: TokenType::Slash})))},
]))}),

"combination" => Some(Object::Function{params: vec!["x", "y"].iter().map(std::string::ToString::to_string).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_owned()))), operator: TokenType::LessThan}), if_block: Box::new(Statement::Return(Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}))), else_block: Box::new(Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "factorial".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_owned()))),
]}), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "factorial".to_owned(), args: vec![Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_owned()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_owned()))), operator: TokenType::Minus}),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "factorial".to_owned(), args: vec![Box::new(Expression::Literal(Object::Identifier("y".to_owned()))),
]}), operator: TokenType::Asterisk}), operator: TokenType::Slash})))},
]))}),
_ => None
}
}