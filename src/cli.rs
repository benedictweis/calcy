use clap::Parser;

/// Evaluate simple algebraic equations fast, that's it!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Equations to evaluate
    #[clap(required_unless_present = "interactive")]
    pub equations: Vec<String>,

    /// Interactive mode (REPL for algebra)
    #[arg(short, long, default_value_t = false)]
    pub interactive: bool,

    /// Evaluation times of each equation
    #[arg(short, long, default_value_t = false)]
    pub benchmark: bool,
}
