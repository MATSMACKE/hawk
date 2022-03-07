use crate::{Object, token::TokenType, tree::{Expression, Statement}};

pub fn get_std_hawk_fn(identifier: String) -> Option<Object> {
match identifier.as_str() {





"pi" => Some(Object::Function{params: Vec::new(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Literal(Object::Float(3.141592653589793)))),
]))}),

"eNum" => Some(Object::Function{params: Vec::new(), block: Box::new(Statement::Return(Box::new(Expression::Literal(Object::Float(2.7182818284590455)))))}),

"mod" => Some(Object::Function{params: vec!["dividend", "divisor"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("divisor".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(0))), operator: TokenType::EqualEqual}), if_block: Box::new(Statement::Return(Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}))), else_block: Box::new(Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("divisor".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("dividend".to_string()))), operator: TokenType::GreaterThan}), if_block: Box::new(Statement::Return(Box::new(Expression::Literal(Object::Identifier("dividend".to_string()))))), else_block: Box::new(Statement::Block(vec![Statement::Definition{name: "product".to_string(), value: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("dividend".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("divisor".to_string()))), operator: TokenType::Slash})},
Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("dividend".to_string()))), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("product".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("divisor".to_string()))), operator: TokenType::Asterisk}), operator: TokenType::Minus})),
]))})},
]))}),

"gcd" => Some(Object::Function{params: vec!["x", "y"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_string()))), operator: TokenType::EqualEqual}), if_block: Box::new(Statement::Return(Box::new(Expression::Literal(Object::Identifier("x".to_string()))))), else_block: Box::new(Statement::Block(vec![Statement::Definition{name: "r".to_string(), value: Box::new(Expression::Literal(Object::Int(0)))},
Statement::While{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("y".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(0))), operator: TokenType::NotEqual}), block: Box::new(Statement::Block(vec![Statement::Definition{name: "r".to_string(), value: Box::new(Expression::FunctionCall{identifier: "mod".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
Box::new(Expression::Literal(Object::Identifier("y".to_string()))),
]})},
Statement::Definition{name: "x".to_string(), value: Box::new(Expression::Literal(Object::Identifier("y".to_string())))},
Statement::Definition{name: "y".to_string(), value: Box::new(Expression::Literal(Object::Identifier("r".to_string())))},
]))},
Statement::Return(Box::new(Expression::Literal(Object::Identifier("x".to_string())))),
]))},
]))}),

"lcm" => Some(Object::Function{params: vec!["x", "y"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_string()))), operator: TokenType::Asterisk}), operand2: Box::new(Expression::FunctionCall{identifier: "gcd".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
Box::new(Expression::Literal(Object::Identifier("y".to_string()))),
]}), operator: TokenType::Slash})))}),

"ln" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::Definition{name: "sum".to_string(), value: Box::new(Expression::Literal(Object::Int(0)))},
Statement::Definition{name: "i".to_string(), value: Box::new(Expression::Literal(Object::Int(1)))},
Statement::While{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("i".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(40))), operator: TokenType::LessThan}), block: Box::new(Statement::Block(vec![Statement::Definition{name: "sum".to_string(), value: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("sum".to_string()))), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}), operand2: Box::new(Expression::Literal(Object::Identifier("i".to_string()))), operator: TokenType::Caret}), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("i".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}), operator: TokenType::Caret}), operator: TokenType::Asterisk}), operand2: Box::new(Expression::Literal(Object::Identifier("i".to_string()))), operator: TokenType::Slash}), operator: TokenType::Plus})},
]))},
Statement::Return(Box::new(Expression::Literal(Object::Identifier("sum".to_string())))),
]))}),


"sin" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::Definition{name: "sum".to_string(), value: Box::new(Expression::Literal(Object::Int(0)))},
Statement::Definition{name: "i".to_string(), value: Box::new(Expression::Literal(Object::Int(1)))},
Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(0))), operator: TokenType::GreaterThan}), if_block: Box::new(Statement::Block(vec![Statement::While{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Int(2))), operand2: Box::new(Expression::FunctionCall{identifier: "pi".to_string(), args: vec![Box::new(Expression::Literal(Object::Null)),
]}), operator: TokenType::Asterisk}), operator: TokenType::GreaterThan}), block: Box::new(Statement::Block(vec![Statement::Definition{name: "x".to_string(), value: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Int(2))), operand2: Box::new(Expression::FunctionCall{identifier: "pi".to_string(), args: vec![Box::new(Expression::Literal(Object::Null)),
]}), operator: TokenType::Asterisk}), operator: TokenType::Minus})},
]))},
])), else_block: Box::new(Statement::Block(vec![Statement::While{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Unary{operand: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Int(2))), operand2: Box::new(Expression::FunctionCall{identifier: "pi".to_string(), args: vec![Box::new(Expression::Literal(Object::Null)),
]}), operator: TokenType::Asterisk}), operator: TokenType::Minus}), operator: TokenType::LessThan}), block: Box::new(Statement::Block(vec![Statement::Definition{name: "x".to_string(), value: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Int(2))), operand2: Box::new(Expression::FunctionCall{identifier: "pi".to_string(), args: vec![Box::new(Expression::Literal(Object::Null)),
]}), operator: TokenType::Asterisk}), operator: TokenType::Plus})},
]))},
]))},
Statement::While{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("i".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(20))), operator: TokenType::LessThan}), block: Box::new(Statement::Block(vec![Statement::Definition{name: "sum".to_string(), value: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("sum".to_string()))), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("i".to_string()))), operator: TokenType::Caret}), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("i".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}), operand2: Box::new(Expression::Literal(Object::Int(2))), operator: TokenType::Slash}), operator: TokenType::Caret}), operator: TokenType::Asterisk}), operand2: Box::new(Expression::FunctionCall{identifier: "factorial".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("i".to_string()))),
]}), operator: TokenType::Slash}), operator: TokenType::Plus})},
Statement::Definition{name: "i".to_string(), value: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("i".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(2))), operator: TokenType::Plus})},
]))},
Statement::Return(Box::new(Expression::Literal(Object::Identifier("sum".to_string())))),
]))}),

"cos" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::FunctionCall{identifier: "sin".to_string(), args: vec![Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "pi".to_string(), args: vec![Box::new(Expression::Literal(Object::Null)),
]}), operand2: Box::new(Expression::Literal(Object::Int(2))), operator: TokenType::Slash}), operator: TokenType::Plus}),
]})),
]))}),

"tan" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "sin".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "cos".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
]}), operator: TokenType::Slash})),
]))}),

"csc" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Int(1))), operand2: Box::new(Expression::FunctionCall{identifier: "sin".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
]}), operator: TokenType::Slash})),
]))}),

"sec" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Int(1))), operand2: Box::new(Expression::FunctionCall{identifier: "cos".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
]}), operator: TokenType::Slash})),
]))}),

"cot" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Int(1))), operand2: Box::new(Expression::FunctionCall{identifier: "tan".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
]}), operator: TokenType::Slash})),
]))}),

"sinh" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "eNum".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "eNum".to_string(), args: vec![Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operator: TokenType::Minus}),
]}), operator: TokenType::Minus}), operand2: Box::new(Expression::Literal(Object::Int(2))), operator: TokenType::Slash})),
]))}),

"cosh" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "eNum".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "eNum".to_string(), args: vec![Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operator: TokenType::Minus}),
]}), operator: TokenType::Plus}), operand2: Box::new(Expression::Literal(Object::Int(2))), operator: TokenType::Slash})),
]))}),

"tanh" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "sinh".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "cosh".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
]}), operator: TokenType::Slash})),
]))}),


"factorial" => Some(Object::Function{params: vec!["x"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::LessThanEqual}), if_block: Box::new(Statement::Return(Box::new(Expression::Literal(Object::Int(1))))), else_block: Box::new(Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::FunctionCall{identifier: "factorial".to_string(), args: vec![Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}),
]}), operator: TokenType::Asterisk})))},
]))}),

"permutation" => Some(Object::Function{params: vec!["x", "y"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_string()))), operator: TokenType::LessThan}), if_block: Box::new(Statement::Return(Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}))), else_block: Box::new(Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "factorial".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "factorial".to_string(), args: vec![Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_string()))), operator: TokenType::Minus}),
]}), operator: TokenType::Slash})))},
]))}),

"combination" => Some(Object::Function{params: vec!["x", "y"].iter().map(|x| x.to_string()).collect(), block: Box::new(Statement::Block(vec![Statement::IfElse{condition: Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_string()))), operator: TokenType::LessThan}), if_block: Box::new(Statement::Return(Box::new(Expression::Unary{operand: Box::new(Expression::Literal(Object::Int(1))), operator: TokenType::Minus}))), else_block: Box::new(Statement::Return(Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "factorial".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("x".to_string()))),
]}), operand2: Box::new(Expression::Binary{operand1: Box::new(Expression::FunctionCall{identifier: "factorial".to_string(), args: vec![Box::new(Expression::Binary{operand1: Box::new(Expression::Literal(Object::Identifier("x".to_string()))), operand2: Box::new(Expression::Literal(Object::Identifier("y".to_string()))), operator: TokenType::Minus}),
]}), operand2: Box::new(Expression::FunctionCall{identifier: "factorial".to_string(), args: vec![Box::new(Expression::Literal(Object::Identifier("y".to_string()))),
]}), operator: TokenType::Asterisk}), operator: TokenType::Slash})))},
]))}),
_ => None
}
}