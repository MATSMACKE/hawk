use crate::{equation::*, terminal::*, expression::*, term::*};

#[derive(Debug, PartialEq, Clone)]
pub enum Factor {
    Exponent{base: Box<Expression>, exponent: Box<Expression>},
    Expression(Box<Expression>)
}

impl Factor {
    /// Create factor from a literal value
    pub fn from_literal(literal: Literal) -> Self {
        Self::Expression(Expression::box_literal(literal))
    }

    /// Create a factor from a list of terms, `(Term + Term + Term + ...)`
    pub fn from_terms(terms: Vec<Term>) -> Self {
        Self::Expression(Expression::box_terms(terms))
    }

    pub fn contains(&self, var: String) -> bool {
        match self {
            Self::Exponent{base, exponent} => base.contains(var.clone()) || exponent.contains(var),
            Self::Expression(expr) => expr.contains(var)
        }
    }
}

#[test]
fn factor_contains() {
    use Literal::*;
    assert!(Factor::from_literal(Literal::Symbol(String::from("x"))).contains(String::from("x")));
    assert!(!Factor::from_literal(Literal::Symbol(String::from("x"))).contains(String::from("y")));
    assert!(Factor::from_terms(vec![Term::from_literal(Int(5)), Term::from_literal(Symbol(String::from("x")))]).contains(String::from("x")))
}