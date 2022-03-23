use crate::{equation::*, terminal::*, term::*, factor::*};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Terms(Vec<Term>),
    Term(Term),
    Literal(Literal)
}

impl Expression {
    pub fn box_literal(value: Literal) -> Box<Self> {
        Box::new(Self::Literal(value))
    }

    pub fn from_literal(value: Literal) -> Self {
        Self::Literal(value)
    }

    pub fn from_term(term: Term) -> Self {
        Self::Term(term)
    }

    pub fn from_terms(terms: Vec<Term>) -> Self {
        Self::Terms(terms)
    }

    pub fn box_terms(terms: Vec<Term>) -> Box<Self> {
        Box::new(Self::Terms(terms))
    }

    pub fn box_symbol(symbol: String) -> Box<Self> {
        Box::new(Self::Literal(Literal::Symbol(symbol)))
    }

    pub fn symbol(symbol: String) -> Self {
        Self::Literal(Literal::Symbol(symbol))
    }

    pub fn contains(&self, var: String) -> bool {
        match self {
            Self::Literal(term) => {
                if let Literal::Symbol(sym) = term {
                    sym == &var
                } else {
                    false
                }
            },
            Self::Term(term) => term.contains(var),
            Self::Terms(terms) => {
                let mut contains = false;
                for term in terms {
                    if term.contains(var.clone()) {
                        contains = true
                    }
                }
                contains
            }
        }
    }

    pub fn simplify(&mut self) {

    }
}

#[test]
fn expression_contains() {
    use Literal::*;
    assert!(Factor::from_literal(Literal::Symbol(String::from("x"))).contains(String::from("x")));
    assert!(!Factor::from_literal(Literal::Symbol(String::from("x"))).contains(String::from("y")));
    assert!(Factor::from_terms(vec![Term::from_literal(Int(5)), Term::from_literal(Symbol(String::from("x")))]).contains(String::from("x")))
}