use std::any::type_name;
use std::fmt::{Debug, Display, Formatter};
use std::iter::{Enumerate, Peekable};
use std::str::{Chars, FromStr};

use crate::parse::Expr::{Add, AddSymbol, ClosingBrackets, Div, DivSymbol, Mul, MulSymbol, Nothing, OpeningBrackets, Pow, PowSymbol, Sub, SubSymbol, Value, Variable};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ValueError(String, String),
    UnexpectedTokenError(usize, char),
    EmptyError,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ValueError(input, type_name) => {
                write!(f, "could not parse {input} to {type_name}")
            }
            ParseError::UnexpectedTokenError(pos, char) => {
                write!(f, "found unexpected token {char} at position {pos}")
            }
            ParseError::EmptyError => write!(f, "empty input found while parsing"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<T> {
    Nothing,
    Value(T),
    Variable(String),
    OpeningBrackets,
    ClosingBrackets,
    AddSymbol,
    Add(Box<Expr<T>>, Box<Expr<T>>),
    SubSymbol,
    Sub(Box<Expr<T>>, Box<Expr<T>>),
    MulSymbol,
    Mul(Box<Expr<T>>, Box<Expr<T>>),
    DivSymbol,
    Div(Box<Expr<T>>, Box<Expr<T>>),
    PowSymbol,
    Pow(Box<Expr<T>>, Box<Expr<T>>),
}

pub fn tokenize<T: Debug + FromStr>(input: String) -> Result<Vec<Expr<T>>, ParseError> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().enumerate().peekable();
    while let Some((i, c)) = iter.next() {
        match c {
            '+' => tokens.push(AddSymbol),
            '-' => tokens.push(SubSymbol),
            '*' => tokens.push(MulSymbol),
            '/' => tokens.push(DivSymbol),
            '^' => tokens.push(PowSymbol),
            '(' => tokens.push(OpeningBrackets),
            ')' => tokens.push(ClosingBrackets),
            '0'..='9' | '.' => tokens.push(parse_num(c, &mut iter)?),
            'a'..='z' | 'A'..='Z' | '"' => {
                if !tokens.is_empty() {
                    if let Some(Value(_)) = tokens.last() {
                        tokens.push(MulSymbol);
                    } else if let Some(Variable(_)) = tokens.last() {
                        tokens.push(MulSymbol);
                    }
                }
                tokens.push(parse_variable(c, &mut iter)?);
            }
            ' ' => continue,
            _ => return Err(ParseError::UnexpectedTokenError(i, c)),
        };
    }
    Ok(tokens)
}

fn parse_num<T: Debug + FromStr>(first: char, iter: &mut Peekable<Enumerate<Chars>>) -> Result<Expr<T>, ParseError> {
    let mut num_str = String::from(first);
    while let Some((_, c)) = iter.peek() {
        match c {
            '0'..='9' | '.' => num_str.push(iter.next().unwrap().1),
            _ => break,
        }
    }
    match T::from_str(&num_str) {
        Ok(v) => Ok(Value(v)),
        Err(_) => Err(ParseError::ValueError(num_str, type_name::<T>().into())),
    }
}

fn parse_variable<T: Debug + FromStr>(first: char, iter: &mut Peekable<Enumerate<Chars>>) -> Result<Expr<T>, ParseError> {
    match first {
        'a'..='z' | 'A'..='Z' => Ok(Variable(first.into())),
        '"' => {
            let mut var = String::new();
            while let Some((i, c)) = iter.peek() {
                match c {
                    'a'..='z' | 'A'..='Z' => var.push(iter.next().unwrap().1),
                    '"' => {
                        iter.next();
                        break;
                    }
                    _ => return Err(ParseError::UnexpectedTokenError(*i, first)),
                }
            }
            Ok(Variable(var))
        }
        _ => Err(ParseError::UnexpectedTokenError(iter.peek().unwrap().0 - 1, first)),
    }
}

pub fn parse_string<T: Debug + FromStr + PartialEq + Clone>(input: Vec<Expr<T>>) -> Result<Expr<T>, ParseError> {
    if input.is_empty() {
        return Err(ParseError::EmptyError);
    } else if input.len() == 1 {
        if let Some(Value(_)) = input.get(0) {
            return Ok(input[0].clone());
        } else if let Some(Variable(_)) = input.get(0) {
            return Ok(input[0].clone());
        }
    } else {
        let operand = split_at_major_operand(input);
        let recurse = |d: Vec<Expr<T>>| -> Result<Box<Expr<T>>, ParseError> { Ok(Box::new(parse_string::<T>(d)?)) };
        return match operand {
            Some((AddSymbol, left, right)) => Ok(Add(recurse(left)?, recurse(right)?)),
            Some((SubSymbol, left, right)) => Ok(Sub(recurse(left)?, recurse(right)?)),
            Some((MulSymbol, left, right)) => Ok(Mul(recurse(left)?, recurse(right)?)),
            Some((DivSymbol, left, right)) => Ok(Div(recurse(left)?, recurse(right)?)),
            Some((PowSymbol, left, right)) => Ok(Pow(recurse(left)?, recurse(right)?)),
            _ => unreachable!(),
        };
    }
    unreachable!();
}

fn split_at_major_operand<T: Debug + PartialEq + Clone>(input: Vec<Expr<T>>) -> Option<(Expr<T>, Vec<Expr<T>>, Vec<Expr<T>>)> {
    let mut level = 100;
    let mut operand = (Nothing, 0, level * 100);
    for (index, expr) in input.iter().enumerate() {
        match expr {
            OpeningBrackets => {
                level += 1;
                continue;
            }
            ClosingBrackets => {
                level -= 1;
                continue;
            }
            _ => {}
        }
        let operand_value = (level * 10) + operand_value(&expr);
        if operand_value <= operand.2 {
            operand = (expr.clone(), index, operand_value);
        }
    }
    if operand.0 == Nothing {
        None
    } else {
        let (left, right) = input.split_at(operand.1);
        Some((operand.0, left.to_vec(), right[1..].to_vec()))
    }
}

fn operand_value<T: Debug>(o: &Expr<T>) -> usize {
    match o {
        AddSymbol => 5,
        SubSymbol => 6,
        MulSymbol => 7,
        DivSymbol => 8,
        PowSymbol => 10,
        _ => 100,
    }
}
