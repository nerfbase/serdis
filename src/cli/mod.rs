//! CLI Parser
//! Parse CLI Arguments

extern crate clap;
extern crate std;

pub mod server;
pub mod start;
mod validator;

use clap::{Parser, Subcommand};
use start::StartCommandArguments;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "Serdis command-line interface and server", bin_name = "serdis")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Start the server")]
    Start(StartCommandArguments),
}

pub async fn init() -> ExitCode {
    let args = Cli::parse();
    let output = match args.command {
        Commands::Start(args) => start::init(args).await,
    };

    if let Err(e) = output {
        eprintln!("{}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
