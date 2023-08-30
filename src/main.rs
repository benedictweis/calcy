use std::collections::HashMap;
use crate::cli::Args;
use clap::Parser;
use std::io::{stdin, stdout, Write};
use std::time::Instant;
use calcy::solve;

mod cli;

fn main() {
    let args = Args::parse();

    if args.interactive {
        repl(args.equations, args.benchmark);
    } else if args.benchmark {
        args.equations.into_iter().for_each(benchmark_eval);
    } else {
        args.equations.into_iter().for_each(eval);
    }
}

fn eval(equation: String) {
    let result = calcy::solve(equation);
    match result {
        Ok(r) => println!("{r}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn benchmark_eval(equation: String) {
    let start = Instant::now();
    let result = calcy::solve(equation);
    let duration = start.elapsed();

    match result {
        Ok(r) => println!("{r} (took {}μs)", duration.as_micros()),
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn repl(equations: Vec<String>, benchmark: bool) {

    let mut variables = HashMap::new();

    equations.into_iter().for_each( |e| {
        let input = move || {
            println!("{e}");
            e.clone()
        };
        repl_eval(benchmark, &mut variables, &input);
    });

    let line = || {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.replace('\n', "")
    };

    loop {
        let next = repl_eval(benchmark, &mut variables, &line);
        if !next {
            return;
        }
    }
}

fn repl_eval(benchmark: bool, variables: &mut HashMap<String,f64>, input_fn: &dyn Fn() -> String) -> bool {
    print!("?: ");
    stdout().flush().unwrap();

    let mut input = input_fn();

    if input.to_lowercase() == "exit" {
        return false;
    }

    if input.to_lowercase() == "vars" {
        println!("{:?}", variables);
        return true;
    }

    if input.contains('=') {
        retrieve_variable(&input, variables);
        return true;
    }

    for (key, value) in variables {
        input = input.replace(key, &value.to_string());
    }

    if benchmark {
        benchmark_eval(input);
    } else {
        eval(input);
    }

    true
}

fn retrieve_variable(input: &str, variables: &mut HashMap<String, f64>) {
    let (name, value) = input.split_once('=').unwrap();
    variables.insert(name.into(), solve(value.into()).unwrap());
}
