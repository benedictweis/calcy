use clap::Parser;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(clap::ValueEnum, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum PossibleDataType {
    usize,
    u8,
    u16,
    u32,
    f32,
    f64,
    decimal,
}

impl Display for PossibleDataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Evaluate simple algebraic equations fast, that's it!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Equations to evaluate
    pub equations: Vec<String>,

    /// Evaluation a file line by line
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// Interactive mode (REPL for algebra)
    #[arg(short, long, default_value_t = false)]
    pub interactive: bool,

    /// Evaluation times of each equation
    #[arg(short, long, default_value_t = false)]
    pub benchmark: bool,

    /// Evaluate expressions with a specific datatype
    #[arg(short, long, default_value_t = PossibleDataType::f64)]
    pub datatype: PossibleDataType,

    /// Evaluate expressions with an exact (decimal) datatype, alias for '--datatype decimal'
    #[arg(short, long, default_value_t = false)]
    pub exact: bool,
}
