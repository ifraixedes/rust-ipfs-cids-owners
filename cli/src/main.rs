mod cli;
mod error;
mod ethereum;
mod ipfs;

use clap::Parser;

fn main() {
    let cli = cli::App::parse();
}
