#![allow(unused)]

use crate::{expression::*, terminal::*, term::*, factor::*};

#[derive(Debug, PartialEq)]
pub struct Equation {
    lhs: Expression,
    rhs: Expression
}

impl Equation {
    pub fn new(lhs: Expression, rhs: Expression) -> Self {
        Equation{lhs, rhs}
    }

    pub fn solve_for(&mut self, var: String) -> Expression {
        self.lhs.simplify();

        if self.lhs.contains(var.to_owned()) && !self.rhs.contains(var.to_owned()) {

        }

        while !self.is_solved(var.to_owned()) {
            self.do_rearrange_step(var.to_owned())
        }

        self.rhs.clone()
    }

    fn is_solved(&self, var: String) -> bool {
        self.lhs == Expression::symbol(var)
    }

    fn do_rearrange_step(&mut self, var: String) {
        match &self.lhs {
            Expression::Literal(x) => {

            },
            Expression::Term(x) => {

            },
            Expression::Terms(x) => {

            }
        }
    }
}

#[test]
fn solve_mult() {
    use Literal::*;

    // 5 = 10 * x
    let mut eq = 
        Equation::new(
        Expression::from_term(
                Term::from_expressions(vec![
                    Expression::Literal(Int(10)),
                    Expression::symbol(String::from("x"))
                ]
                )
            ),
        Expression::Literal(Int(5)), 
        );

    assert_eq!(
        eq.solve_for(String::from("x")), 
        Expression::from_term(
            Term::div(
                Factor::from_literal(Int(5)), 
                Factor::from_literal(Int(10))
            )
        )
    )
}
