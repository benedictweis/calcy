use clap::Parser;
use std::path::PathBuf;

/// Evaluate simple algebraic equations fast, that's it!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Equations to evaluate
    pub equations: Vec<String>,

    /// Evaluation times of each equation
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// Interactive mode (REPL for algebra)
    #[arg(short, long, default_value_t = false)]
    pub interactive: bool,

    /// Evaluation times of each equation
    #[arg(short, long, default_value_t = false)]
    pub benchmark: bool,
}
