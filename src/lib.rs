use crate::eval::{eval_expr, EvalError};
use crate::parse::{parse_string, tokenize, ParseError};
use log::{debug, info};
use num::traits::Pow;
use std::any::type_name;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

pub mod decimal;
pub mod eval;
pub mod parse;
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError(ParseError),
    EvalError(EvalError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(p) => write!(f, "{p}"),
            Error::EvalError(e) => write!(f, "{e}"),
        }
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Error::ParseError(value)
    }
}

impl From<EvalError> for Error {
    fn from(value: EvalError) -> Self {
        Error::EvalError(value)
    }
}

#[macro_export] macro_rules! solve {
    ($ex:expr) => {
            $crate::solve($ex)
    };
    ($ex:expr, $vars:expr) => {
            $crate::solve_vars($ex, $vars)
    };
}

pub fn solve(input: String) -> Result<f64, Error> {
    solve_with::<f64>(input)
}

pub fn solve_vars(input: String, variables: &HashMap<String, f64>) -> Result<f64, Error> {
    solve_vars_with::<f64>(input, variables)
}

pub fn solve_with<T: PartialEq>(input: String) -> Result<T, Error>
where
    T: Pow<T, Output = T> + Debug + FromStr + Copy + Sized + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Rem<Output = T>,
{
    solve_vars_with(input, &HashMap::new())
}

pub fn solve_vars_with<T: PartialEq>(input: String, variables: &HashMap<String, T>) -> Result<T, Error>
where
    T: Pow<T, Output = T> + Debug + FromStr + Copy + Sized + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Rem<Output = T>,
{
    info!("Solving equation {input} with type {} and variables {variables:?}", type_name::<T>());
    let tokenized_input = tokenize(input)?;
    debug!("Tokenized input: {tokenized_input:?}");
    let parsed_input = parse_string(tokenized_input)?;
    debug!("Parsed input: {parsed_input:?}");
    Ok(eval_expr::<T>(&parsed_input, variables)?)
}
