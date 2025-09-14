use clap::{Parser, command, Subcommand, ValueEnum};
use std::time::Instant;

use roozle::{self as rz, AnalysisConfig, MatchCountReport, MatchFrequencyReport, MatchIndicesReport, UniqueMatchesReport};
use rz::Search;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// File to perform search on
    file: String,

    #[command(subcommand)]
    search_type: SearchType,

    #[arg(short, long, global = true, num_args=1..=4)]  // TODO: make required
    output: Vec<AnalysisType>,

    #[arg(short, long)]
    threads: Option<usize>,

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

    let file_name = cli.file;
    let buffer = rz::Buffer::from_file(&file_name).expect("Could not open file");

    let search: Box<dyn Search> = match cli.search_type {
        SearchType::Exact { pattern } => { Box::new(rz::Exact::from_pattern(pattern)) },
        _ => unimplemented!()
    };

    let mut config = AnalysisConfig::new();
    for c in &cli.output {
        match c {
            AnalysisType::Unique => config.add::<UniqueMatchesReport>(),
            AnalysisType::Frequency => config.add::<MatchFrequencyReport>(),
            AnalysisType::Indices => config.add::<MatchIndicesReport>(),
            AnalysisType::Count => config.add::<MatchCountReport>(),
        }
    }

    let search_start_time = Instant::now();
    let analysis;
    if let Some(t) = cli.threads {
        rz::set_thread_count(t);
        analysis = search.search_mt(&buffer, &config);
    } else {
        analysis = search.search(&buffer, &config);
    }
    let search_end_time = Instant::now();

    for c in &cli.output {
        match c {
            AnalysisType::Unique => {
                let u = analysis
                    .report::<UniqueMatchesReport>().unwrap();
                println!("unique: {:?}", u.matches);
            },
            AnalysisType::Frequency => {
                let f = analysis
                    .report::<MatchFrequencyReport>().unwrap();
                println!("frequencies: {:?}", f.frequencies);
            },
            AnalysisType::Indices => {
                let i = analysis
                    .report::<MatchIndicesReport>().unwrap();
                println!("indices: {:?}", i.indices);
            },
            AnalysisType::Count => {
                let c = analysis
                    .report::<MatchCountReport>().unwrap();
                println!("count: {:?}", c.count);
            },
        }
    }

    if cli.verbose {
        println!("\nverbose:");
        println!("\ttime (total): {:?}", start_time.elapsed());
        println!("\ttime (search): {:?}", search_end_time.duration_since(search_start_time));
    }
}
