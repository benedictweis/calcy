use crate::cli::{Args, PossibleDataType};
use calcy::decimal::Decimal;
use clap::Parser;
use console::style;
use log::{debug, warn};
use num::traits::Pow;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use std::time::Instant;
use std::{fs, process};

mod cli;

fn main() {
    let mut args = Args::parse();

    env_logger::init();

    if args.file.is_none() && args.equations.is_empty() {
        args.interactive = true;
    }

    if args.exact {
        args.datatype = PossibleDataType::decimal;
    }

    match args.datatype {
        PossibleDataType::usize => calcy::<usize>(args),
        PossibleDataType::u8 => calcy::<u8>(args),
        PossibleDataType::u16 => calcy::<u16>(args),
        PossibleDataType::u32 => calcy::<u32>(args),
        PossibleDataType::f32 => calcy::<f32>(args),
        PossibleDataType::f64 => calcy::<f64>(args),
        PossibleDataType::decimal => calcy::<Decimal>(args),
    }
}

trait TypeConstraint<T>: Debug + Display + FromStr + Copy + PartialEq + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Pow<T, Output = T> {}
impl<T: Debug + Display + FromStr + Copy + PartialEq + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Pow<T, Output = T>> TypeConstraint<T> for T {}

fn calcy<T>(args: Args)
where
    T: TypeConstraint<T>
{
    let mut variables: HashMap<String, T> = HashMap::new();
    let mut exit_code = 0;

    if let Some(file_path) = args.file {
        debug!("Attempting to read from file {}", file_path.display());
        let contents = fs::read_to_string(file_path).expect("could not read from file");
        let lines = contents.lines();
        lines.for_each(|l| interpret_statement(l.into(), args.benchmark, &mut variables, &mut exit_code));
    }

    args.equations.into_iter().for_each(|e| interpret_statement(e, args.benchmark, &mut variables, &mut exit_code));

    if args.interactive {
        println!("Calcy (v{}), have fun!", env!("CARGO_PKG_VERSION"));
        repl(&mut variables, args.benchmark);
        exit_code = 0;
    }
    process::exit(exit_code);
}

fn interpret_statement<T>(statement: String, benchmark: bool, variables: &mut HashMap<String, T>, exit_code: &mut i32)
where
    T: TypeConstraint<T>,
{
    if statement.to_lowercase() == "exit" {
        println!("Exiting...");
        process::exit(*exit_code);
    }

    if statement.to_lowercase() == "vars" {
        println!("{:?}", variables);
        return;
    }

    if statement.contains('=') {
        retrieve_variable(&statement, variables);
        return;
    }

    eval(statement, benchmark, variables, exit_code);
}

fn retrieve_variable<T>(input: &str, variables: &mut HashMap<String, T>)
where
    T: TypeConstraint<T>,
{
    let (name, value) = input.split_once('=').unwrap();
    variables.insert(name.into(), calcy::solve_vars_with(value.into(), variables).unwrap());
}

fn eval<T>(equation: String, benchmark: bool, variables: &mut HashMap<String, T>, exit_code: &mut i32)
where
    T: TypeConstraint<T>,
{
    let start = Instant::now();
    let result = calcy::solve_vars_with::<T>(equation, variables);
    let duration = start.elapsed();
    match result {
        Ok(r) => {
            if benchmark {
                println!("{r} (took {}μs)", duration.as_micros());
            } else {
                println!("{r}");
            }
            variables.insert("ans".into(), r);
        }
        Err(e) => {
            eprintln!("{}", style(format!("error: {e}")).red());
            *exit_code = 1;
        }
    }
}

fn repl<T>(variables: &mut HashMap<String, T>, benchmark: bool)
where
    T: TypeConstraint<T>,
{
    let mut rl = DefaultEditor::new().expect("cannot start repl");
    let history_path = std::env::temp_dir().join("calcy-history.txt");
    debug!("Using {history_path:?} as history file");
    if rl.load_history(&history_path).is_err() {
        warn!("No previous history could be found at {history_path:?}")
    }
    loop {
        let readline = rl.readline("?: ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).expect("could not add history entry");
                rl.save_history(&history_path).expect("could not save history");
                interpret_statement(line, benchmark, variables, &mut 0);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break;
            }
            Err(err) => {
                eprintln!("{}", style(format!("error: {err:?}")).red());
                break;
            }
        }
    }
}
