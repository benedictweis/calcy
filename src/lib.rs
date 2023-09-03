use crate::eval::{eval_expr, eval_expr_with, EvalError};
use crate::parse::{parse_string, tokenize, ParseError};
use log::{debug, info};
use std::any::type_name;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

pub mod eval;
pub mod parse;
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum CalcyError {
    ParseError(ParseError),
    EvalError(EvalError),
}

impl Display for CalcyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcyError::ParseError(p) => write!(f, "{p}"),
            CalcyError::EvalError(e) => write!(f, "{e}"),
        }
    }
}

impl From<ParseError> for CalcyError {
    fn from(value: ParseError) -> Self {
        CalcyError::ParseError(value)
    }
}

impl From<EvalError> for CalcyError {
    fn from(value: EvalError) -> Self {
        CalcyError::EvalError(value)
    }
}

pub fn solve(input: String) -> Result<f64, CalcyError> {
    solve_vars(input, &HashMap::new())
}

pub fn solve_vars(input: String, variables: &HashMap<String, f64>) -> Result<f64, CalcyError> {
    info!("Solving equation {input} with variables {variables:?}");
    let tokenized_input = tokenize(input)?;
    debug!("Tokenized input: {tokenized_input:?}");
    let parsed_input = parse_string(tokenized_input)?;
    debug!("Parsed input: {parsed_input:?}");
    Ok(eval_expr(&parsed_input, variables)?)
}

pub fn solve_with<T: PartialEq>(input: String) -> Result<T, CalcyError>
where
    T: Debug + FromStr + Copy + Sized + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    solve_vars_with(input, &HashMap::new())
}

pub fn solve_vars_with<T: PartialEq>(input: String, variables: &HashMap<String, T>) -> Result<T, CalcyError>
where
    T: Debug + FromStr + Copy + Sized + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    info!("Solving equation {input} with type {} and variables {variables:?}", type_name::<T>());
    let tokenized_input = tokenize(input)?;
    debug!("Tokenized input: {tokenized_input:?}");
    let parsed_input = parse_string(tokenized_input)?;
    Ok(eval_expr_with::<T>(&parsed_input, variables)?)
}
