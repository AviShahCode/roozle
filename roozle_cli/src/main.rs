use clap::{Parser, command};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    
}

fn main() {
    let cli = Cli::parse();
}
