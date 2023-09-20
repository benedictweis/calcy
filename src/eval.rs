use crate::eval::EvalError::VariableNotFound;
use crate::parse::Expr;
use num::traits::Pow;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, PartialEq)]
pub enum EvalError {
    VariableNotFound(String),
}

impl Display for EvalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableNotFound(v) => write!(f, "variable {v} was not found"),
        }
    }
}

pub fn eval_expr<T>(expr: &Expr<T>, variables: &HashMap<String, T>) -> Result<T, EvalError>
where
    T: Pow<T, Output = T> + Copy + Debug + Sized + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Rem<Output = T>,
{
    Ok(match expr {
        Expr::Value(v) => *v,
        Expr::Variable(v) => *variables.get(v).ok_or_else(|| VariableNotFound(v.into()))?,
        Expr::Add(a, b) => eval_expr(a.as_ref(), variables)? + eval_expr(b.as_ref(), variables)?,
        Expr::Sub(a, b) => eval_expr(a.as_ref(), variables)? - eval_expr(b.as_ref(), variables)?,
        Expr::Mul(a, b) => eval_expr(a.as_ref(), variables)? * eval_expr(b.as_ref(), variables)?,
        Expr::Div(a, b) => eval_expr(a.as_ref(), variables)? / eval_expr(b.as_ref(), variables)?,
        Expr::Pow(a, b) => eval_expr(a.as_ref(), variables)?.pow(eval_expr(b.as_ref(), variables)?),
        Expr::Rem(a, b) => eval_expr(a.as_ref(), variables)? % eval_expr(b.as_ref(), variables)?,
    })
}
