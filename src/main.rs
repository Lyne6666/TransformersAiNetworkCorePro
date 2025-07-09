// src/main.rs
/*
 * Main executable for TransformersAiNetworkCorePro
 */

use clap::Parser;
use transformersainetworkcorepro::{Result, run};

#[derive(Parser)]
#[command(version, about = "TransformersAiNetworkCorePro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
