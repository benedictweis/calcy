use crate::cli::Args;
use calcy::solve;
use clap::Parser;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::time::Instant;

mod cli;

fn main() {
    let args = Args::parse();

    env_logger::init();

    if args.interactive {
        repl(args.equations, args.benchmark);
    } else {
        args.equations
            .into_iter()
            .for_each(|e| eval(e, args.benchmark, &HashMap::new()));
    }
}

fn eval(equation: String, benchmark: bool, variables: &HashMap<String, f64>) {
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
        }
        Err(e) => eprintln!("error: {e}"),
    }
}

fn repl(equations: Vec<String>, benchmark: bool) {
    let mut variables = HashMap::new();

    equations.into_iter().for_each(|e| {
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

fn repl_eval(
    benchmark: bool,
    variables: &mut HashMap<String, f64>,
    input_fn: &dyn Fn() -> String,
) -> bool {
    print!("?: ");
    stdout().flush().unwrap();

    let input = input_fn();

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

    eval(input, benchmark, variables);

    true
}

fn retrieve_variable(input: &str, variables: &mut HashMap<String, f64>) {
    let (name, value) = input.split_once('=').unwrap();
    variables.insert(name.into(), solve(value.into()).unwrap());
}
