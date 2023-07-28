use crate::cli::Args;
use clap::Parser;
use std::io::{stdin, stdout, Write};
use std::time::Instant;

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
    equations.into_iter().for_each(move |e| {
        let input = move || {
            println!("{e}");
            e.clone()
        };
        repl_eval(benchmark, &input);
    });

    let line = || {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.replace('\n', "")
    };

    loop {
        let next = repl_eval(benchmark, &line);
        if !next {
            return;
        }
    }
}

fn repl_eval(benchmark: bool, input_fn: &dyn Fn() -> String) -> bool {
    print!("?: ");
    stdout().flush().unwrap();

    let input = input_fn();

    if input.to_lowercase() == "exit" {
        return false;
    }

    if benchmark {
        benchmark_eval(input);
    } else {
        eval(input);
    }

    true
}
