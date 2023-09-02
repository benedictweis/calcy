use std::any::type_name;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::parse::Expr::{Add, Div, Mul, Pow, Sub, Value, Variable};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ValueError(String, String),
    EmptyError,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ValueError(input, type_name) => {
                write!(f, "could not parse {input} to {type_name}")
            }
            ParseError::EmptyError => write!(f, "empty input found while parsing"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr<T> {
    Value(T),
    Variable(String),
    Add(Box<Expr<T>>, Box<Expr<T>>),
    Sub(Box<Expr<T>>, Box<Expr<T>>),
    Mul(Box<Expr<T>>, Box<Expr<T>>),
    Div(Box<Expr<T>>, Box<Expr<T>>),
    Pow(Box<Expr<T>>, Box<Expr<T>>),
}

pub fn parse_simple_string<T: Debug + FromStr>(input: String) -> Result<Expr<T>, ParseError> {
    if let Some((left, right)) = input.split_once(|c| c == '+') {
        Ok(Add(
            Box::new(parse_simple_string(left.into())?),
            Box::new(parse_simple_string(right.into())?),
        ))
    } else if let Some((left, right)) = input.rsplit_once(|c| c == '-') {
        Ok(Sub(
            Box::new(parse_simple_string(left.into())?),
            Box::new(parse_simple_string(right.into())?),
        ))
    } else if let Some((left, right)) = input.split_once(|c| c == '*') {
        Ok(Mul(
            Box::new(parse_simple_string(left.into())?),
            Box::new(parse_simple_string(right.into())?),
        ))
    } else if let Some((left, right)) = input.rsplit_once(|c| c == '/') {
        Ok(Div(
            Box::new(parse_simple_string(left.into())?),
            Box::new(parse_simple_string(right.into())?),
        ))
    } else if let Some((left, right)) = input.split_once(|c| c == '^') {
        Ok(Pow(
            Box::new(parse_simple_string(left.into())?),
            Box::new(parse_simple_string(right.into())?),
        ))
    } else {
        match trim(&input).parse::<T>() {
            Ok(n) => Ok(Value(n)),
            Err(_) => parse_variables::<T>(parse_simple_string, input),
        }
    }
}

pub fn parse_string<T: Debug + FromStr>(input: String) -> Result<Expr<T>, ParseError> {
    if input.is_empty() {
        return Err(ParseError::EmptyError);
    }
    if let Ok(n) = trim(&input).parse::<T>() {
        Ok(Value(n))
    } else {
        let operand = split_at_major_operand(&input);
        let recurse =
            |d: String| -> Result<Box<Expr<T>>, ParseError> { Ok(Box::new(parse_string::<T>(d)?)) };
        match operand {
            Some(('+', left, right)) => Ok(Add(recurse(left)?, recurse(right)?)),
            Some(('-', left, right)) => Ok(Sub(recurse(left)?, recurse(right)?)),
            Some(('*', left, right)) => Ok(Mul(recurse(left)?, recurse(right)?)),
            Some(('/', left, right)) => Ok(Div(recurse(left)?, recurse(right)?)),
            Some(('^', left, right)) => Ok(Pow(recurse(left)?, recurse(right)?)),
            _ => parse_variables::<T>(parse_string, input),
        }
    }
}

pub(crate) fn parse_variables<T: Debug + FromStr>(
    recurse: fn(String) -> Result<Expr<T>, ParseError>,
    input: String,
) -> Result<Expr<T>, ParseError> {
    if !input.chars().all(|c| c.is_alphabetic() || c == '"') {
        return Err(ParseError::ValueError(input, type_name::<T>().into()));
    }
    let mut vars: Vec<String> = Vec::new();
    let mut iter = input.chars();
    while let Some(c) = iter.next() {
        if c == '\"' {
            let mut buf = String::new();
            for e in iter.by_ref() {
                if e == '\"' {
                    break;
                }
                buf.push(e);
            }
            vars.push(buf);
            continue;
        }
        vars.push(c.to_string());
    }
    if vars.len() == 1 {
        Ok(Variable(vars[0].clone()))
    } else {
        vars = vars.into_iter().map(|v| format!("\"{v}\"")).collect();
        Ok(recurse(vars.join("*"))?)
    }
}

fn split_at_major_operand(input: &str) -> Option<(char, String, String)> {
    let mut level = 100;
    let mut operand = ('v', 0, level * 100);
    for (index, char) in input.chars().enumerate() {
        match char {
            '(' => {
                level += 1;
                continue;
            }
            ')' => {
                level -= 1;
                continue;
            }
            _ => {}
        }
        let operand_value = (level * 10) + operand_value(char);
        if operand_value <= operand.2 {
            operand = (char, index, operand_value);
        }
    }
    if operand.0 == 'v' {
        None
    } else {
        let (left, right) = input.split_at(operand.1);
        Some((operand.0, left.into(), right[1..].into()))
    }
}

fn operand_value(c: char) -> usize {
    match c {
        '+' => 5,
        '-' => 6,
        '*' => 7,
        '/' => 8,
        '^' => 10,
        _ => 100,
    }
}

fn trim(input: &str) -> String {
    input.replace(['(', ')', ' '], "")
}
