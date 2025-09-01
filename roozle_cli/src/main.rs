use clap::{Parser, command, Subcommand, ValueEnum};
use std::time::Instant;

use roozle as rz;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// File to perform search on
    file: String,

    #[command(subcommand)]
    search_type: SearchType,

    #[arg(short, long, global = true, num_args=1..=4)]
    output: Vec<AnalysisType>,

    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone, Subcommand)]
enum SearchType {
    /// Exact search
    Exact {
        /// Pattern for exact search
        pattern: String,
    },
    /// Regex search
    Regex {
        /// Pattern for regex search
        pattern: String
    }
}

#[derive(Debug, Clone, ValueEnum)]
enum AnalysisType {
    #[value(alias = "u")]
    Unique,
    #[value(alias = "f")]
    Frequency,
    #[value(alias = "i")]
    Indices,
    #[value(alias = "c")]
    Count,
}

fn main() {
    let cli = Cli::parse();
    let start_time = Instant::now();

    if cli.verbose {
        println!("time (total): {:?}", start_time.elapsed());
    }
}
