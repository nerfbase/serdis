//! CLI Parser
//! Parse CLI Arguments

extern crate clap;
extern crate log;
extern crate std;

pub mod server;
pub mod start;
mod validator;

use crate::cnf::log_cfg;
use clap::{Parser, Subcommand};
use log::error;
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
    // init logger
    log_cfg();

    // parse args
    let args = Cli::parse();
    let output = match args.command {
        Commands::Start(args) => start::init(args).await,
    };

    if let Err(error) = output {
        error!("{error}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
