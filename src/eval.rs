use crate::parse::Expr;
use std::ops::{Add, Div, Mul, Sub};

pub fn eval_expr<T>(expr: &Expr<T>) -> T
where
    T: Copy + Sized + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    match expr {
        Expr::Value(v) => *v,
        Expr::Add(a, b) => eval_expr(a.as_ref()) + eval_expr(b.as_ref()),
        Expr::Sub(a, b) => eval_expr(a.as_ref()) - eval_expr(b.as_ref()),
        Expr::Mul(a, b) => eval_expr(a.as_ref()) * eval_expr(b.as_ref()),
        Expr::Div(a, b) => eval_expr(a.as_ref()) / eval_expr(b.as_ref()),
        Expr::Pow(a, b) => eval_expr(a.as_ref()) * eval_expr(b.as_ref()),
    }
}
