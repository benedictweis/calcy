use log::trace;
use std::any::type_name;
use std::fmt::{Debug, Display, Formatter};
use std::iter::{Enumerate, Peekable};
use std::str::{Chars, FromStr};

use crate::parse::Expr::{Add, Div, Mul, Pow, Sub};
use crate::parse::Token::{AddSymbol, ClosingBrackets, DivSymbol, MulSymbol, OpeningBrackets, PowSymbol, SubSymbol};

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
pub enum Token<T> {
    Value(T),
    Variable(String),
    OpeningBrackets,
    ClosingBrackets,
    AddSymbol,
    SubSymbol,
    MulSymbol,
    DivSymbol,
    PowSymbol,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<T> {
    Value(T),
    Variable(String),
    Add(Box<Expr<T>>, Box<Expr<T>>),
    Sub(Box<Expr<T>>, Box<Expr<T>>),
    Mul(Box<Expr<T>>, Box<Expr<T>>),
    Div(Box<Expr<T>>, Box<Expr<T>>),
    Pow(Box<Expr<T>>, Box<Expr<T>>),
}

pub fn tokenize<T: Debug + FromStr>(input: String) -> Result<Vec<Token<T>>, ParseError> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().enumerate().peekable();
    while let Some((i, c)) = iter.next() {
        match c {
            '+' | '-' | '*' | '/' | '^' => {
                if tokens.is_empty() {
                    tokens.push(parse_num(c, &mut iter)?);
                } else if !tokens.is_empty() && matches!(tokens.last(), Some(Token::OpeningBrackets)) {
                    tokens.pop();
                    tokens.push(parse_num(c, &mut iter)?);
                    iter.next();
                } else if !tokens.is_empty()
                    && !matches!(tokens.last(), Some(Token::Value(_)))
                    && !matches!(tokens.last(), Some(Token::Variable(_)))
                    && !matches!(tokens.last(), Some(Token::ClosingBrackets))
                {
                    return Err(ParseError::UnexpectedTokenError(i, c));
                } else {
                    match c {
                        '+' => tokens.push(Token::AddSymbol),
                        '-' => tokens.push(Token::SubSymbol),
                        '*' => tokens.push(Token::MulSymbol),
                        '/' => tokens.push(Token::DivSymbol),
                        '^' => tokens.push(Token::PowSymbol),
                        _ => unreachable!(),
                    }
                }
            }
            '(' => {
                if !tokens.is_empty() && (matches!(tokens.last(), Some(Token::Value(_))) || matches!(tokens.last(), Some(Token::Variable(_)))) {
                    tokens.push(Token::MulSymbol);
                }
                tokens.push(Token::OpeningBrackets);
            }
            ')' => tokens.push(Token::ClosingBrackets),
            '0'..='9' | '.' => tokens.push(parse_num(c, &mut iter)?),
            'a'..='z' | 'A'..='Z' | '"' => {
                if !tokens.is_empty() && (matches!(tokens.last(), Some(Token::Value(_))) || matches!(tokens.last(), Some(Token::Variable(_)))) {
                    tokens.push(MulSymbol);
                }
                tokens.push(parse_variable(c, &mut iter)?);
            }
            ' ' => continue,
            _ => return Err(ParseError::UnexpectedTokenError(i, c)),
        };
    }
    Ok(tokens)
}

fn parse_num<T: Debug + FromStr>(first: char, iter: &mut Peekable<Enumerate<Chars>>) -> Result<Token<T>, ParseError> {
    let mut num_str = String::from(first);
    while let Some((_, c)) = iter.peek() {
        match c {
            '0'..='9' | '.' => num_str.push(iter.next().unwrap().1),
            _ => break,
        }
    }
    match T::from_str(&num_str) {
        Ok(v) => Ok(Token::Value(v)),
        Err(_) => Err(ParseError::ValueError(num_str, type_name::<T>().into())),
    }
}

fn parse_variable<T: Debug + FromStr>(first: char, iter: &mut Peekable<Enumerate<Chars>>) -> Result<Token<T>, ParseError> {
    match first {
        'a'..='z' | 'A'..='Z' => Ok(Token::Variable(first.into())),
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
            Ok(Token::Variable(var))
        }
        _ => Err(ParseError::UnexpectedTokenError(iter.peek().unwrap().0 - 1, first)),
    }
}

pub fn parse_string<T: Debug + FromStr + PartialEq + Clone>(input: Vec<Token<T>>) -> Result<Expr<T>, ParseError> {
    trace!("Parsing {input:?}");
    if input.is_empty() {
        return Err(ParseError::EmptyError);
    } else if input.len() == 1 {
        if let Some(Token::Value(v)) = input.get(0) {
            return Ok(Expr::Value(v.clone()));
        } else if let Some(Token::Variable(v)) = input.get(0) {
            return Ok(Expr::Variable(v.clone()));
        }
    } else {
        let operand = split_at_major_operand(input);
        let recurse = |d: Vec<Token<T>>| -> Result<Box<Expr<T>>, ParseError> { Ok(Box::new(parse_string::<T>(d)?)) };
        return match operand {
            (AddSymbol, left, right) => Ok(Add(recurse(left)?, recurse(right)?)),
            (SubSymbol, left, right) => Ok(Sub(recurse(left)?, recurse(right)?)),
            (MulSymbol, left, right) => Ok(Mul(recurse(left)?, recurse(right)?)),
            (DivSymbol, left, right) => Ok(Div(recurse(left)?, recurse(right)?)),
            (PowSymbol, left, right) => Ok(Pow(recurse(left)?, recurse(right)?)),
            _ => unreachable!(),
        };
    }
    unreachable!();
}

type OperandRest<T> = (Token<T>, Vec<Token<T>>, Vec<Token<T>>);

#[derive(Debug)]
struct Operand<T> {
    token: Token<T>,
    index: usize,
    value: usize,
    level: usize,
}

fn split_at_major_operand<T: Debug + PartialEq + Clone>(input: Vec<Token<T>>) -> OperandRest<T> {
    let mut level = 0;
    let mut operand = Operand {
        token: Token::Variable("no operands".into()),
        index: 0,
        value: 0,
        level: 1000,
    };
    for (index, token) in input.iter().enumerate() {
        match token {
            OpeningBrackets => {
                level += 1;
            }
            ClosingBrackets => {
                level -= 1;
            }
            _ => {
                let value = operand_value(token);
                if level < operand.level || (level == operand.level && value >= operand.value) {
                    operand = Operand {
                        token: token.clone(),
                        index,
                        value,
                        level,
                    };
                }
            }
        }
    }
    let (left, right) = input.split_at(operand.index);
    let left = left[operand.level..].to_vec();
    let right = right[1..right.len() - operand.level].to_vec();
    trace!("Split at {operand:?} with left {left:?} and right {right:?}");
    (operand.token, left, right)
}

fn operand_value<T: Debug>(o: &Token<T>) -> usize {
    match o {
        AddSymbol => 10,
        SubSymbol => 9,
        MulSymbol => 8,
        DivSymbol => 7,
        PowSymbol => 6,
        _ => 0,
    }
}
