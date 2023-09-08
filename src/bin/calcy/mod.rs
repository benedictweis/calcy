use crate::cli::Args;
use clap::Parser;
use console::style;
use log::{debug, warn};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::collections::HashMap;
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

fn interpret_statement(statement: String, benchmark: bool, variables: &mut HashMap<String, f64>, exit_code: &mut i32) {
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

fn retrieve_variable(input: &str, variables: &mut HashMap<String, f64>) {
    let (name, value) = input.split_once('=').unwrap();
    variables.insert(name.into(), calcy::solve_vars(value.into(), variables).unwrap());
}

fn eval(equation: String, benchmark: bool, variables: &mut HashMap<String, f64>, exit_code: &mut i32) {
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
        Err(e) => {
            eprintln!("{}", style(format!("error: {e}")).red());
            *exit_code = 1;
        }
    }
}

fn repl(variables: &mut HashMap<String, f64>, benchmark: bool) {
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
