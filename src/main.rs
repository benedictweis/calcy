use crate::cli::Args;
use clap::Parser;
use log::debug;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::time::Instant;
use std::{fs, process};

mod cli;

fn main() {
    let mut args = Args::parse();

    env_logger::init();

    if args.file.is_none() && args.equations.is_empty() {
        args.interactive = true;
    }

    let mut variables = HashMap::new();

    if let Some(file_path) = args.file {
        debug!("Attempting to read from file {}", file_path.display());
        let contents = fs::read_to_string(file_path).expect("could not read from file");
        let lines = contents.lines();
        lines.for_each(|l| interpret_statement(l.into(), args.benchmark, &mut variables));
    }

    args.equations.into_iter().for_each(|e| interpret_statement(e, args.benchmark, &mut variables));

    if args.interactive {
        println!("Calcy (v{}), have fun!", env!("CARGO_PKG_VERSION"));
        repl(&mut variables, args.benchmark);
    }
}

fn interpret_statement(statement: String, benchmark: bool, variables: &mut HashMap<String, f64>) {
    if statement.to_lowercase() == "exit" {
        process::exit(0);
    }

    if statement.to_lowercase() == "vars" {
        println!("{:?}", variables);
        return;
    }

    if statement.contains('=') {
        retrieve_variable(&statement, variables);
        return;
    }

    eval(statement, benchmark, variables);
}

fn retrieve_variable(input: &str, variables: &mut HashMap<String, f64>) {
    let (name, value) = input.split_once('=').unwrap();
    variables.insert(name.into(), calcy::solve_vars(value.into(), variables).unwrap());
}

fn eval(equation: String, benchmark: bool, variables: &mut HashMap<String, f64>) {
    let start = Instant::now();
    let result = calcy::solve_vars(equation, variables);
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
        Err(e) => eprintln!("error: {e}"),
    }
}

fn repl(variables: &mut HashMap<String, f64>, benchmark: bool) {
    loop {
        print!("?: ");
        stdout().flush().unwrap();
        interpret_statement(read_line(), benchmark, variables);
    }
}

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.replace('\n', "")
}
