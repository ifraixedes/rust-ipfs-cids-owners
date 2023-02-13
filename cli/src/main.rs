//! Command-line application for uploading files to IPFS and register the corresponding CID to
//! CIDsOwners Ethereum smart contract.

mod cli;
mod cmd;
mod error;
mod ethereum;
mod ipfs;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = cli::App::parse();
    match cmd::upload_and_register(args).await {
        Ok(summary) => println!("{}", summary),
        Err(err) => println!("{}", err),
    };
}
