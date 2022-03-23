#![allow(dead_code, unused_imports)]

use expression::Expression;

mod equation;
mod expression;
mod terminal;
mod term;
mod factor;


trait ToCASExpr {
    fn to_cas_expr(&mut self) -> Expression;
    fn from_cas_expr(expr: Expression) -> Self;
}
