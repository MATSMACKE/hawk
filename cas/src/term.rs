use crate::{factor::*, equation::*, terminal::*, expression::*};

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Factors(Vec<Factor>),
    Div{op1: Factor, op2: Factor},
    Expression(Box<Expression>)
}

impl Term {
    pub fn from_factors(factors: Vec<Factor>) -> Self {
        Self::Factors(factors)
    }

    pub fn from_literal(value: Literal) -> Self {
        Self::Expression(Expression::box_literal(value))
    }

    pub fn from_expressions(exprs: Vec<Expression>) -> Self {
        Self::Factors(exprs.iter().map(|x| Factor::Expression(Box::new(x.to_owned()))).collect())
    }

    pub fn div(op1: Factor, op2: Factor) -> Self {
        Self::Div{op1, op2}
    }

    pub fn contains(&self, var: String) -> bool {
        match self {
            Self::Factors(factors) => {
                let mut contains = false;
                for factor in factors {
                    if factor.contains(var.clone()) {
                        contains = true
                    }
                }
                contains
            },
            Self::Expression(expr) => expr.contains(var),
            Self::Div{op1, op2} => op1.contains(var.clone()) || op2.contains(var)
        }
    }
}

#[test]
fn term_contains() {
    use Literal::*;
    assert!(Term::from_literal(Literal::Symbol(String::from("x"))).contains(String::from("x")));
    assert!(!Term::from_literal(Literal::Symbol(String::from("x"))).contains(String::from("y")));
    assert!(Term::from_factors(vec![Factor::from_literal(Int(5)), Factor::from_literal(Symbol(String::from("x")))]).contains(String::from("x")))
}