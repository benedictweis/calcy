use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use crate::eval::eval_expr;
use crate::parse::{parse_string, ParseError};

pub mod parse;
pub mod eval;
#[cfg(test)]
mod tests;

pub fn solve(input: String) -> Result<f64, ParseError> {
    solve_with::<f64>(input)
}

pub fn solve_with<T>(input: String) -> Result<T,ParseError>
    where T: Debug + FromStr + Copy + Sized + Add<Output = T> + Sub<Output  = T> + Mul<Output = T> + Div<Output = T> {
    Ok(eval_expr::<T>(&parse_string::<T>(input)?))
}

