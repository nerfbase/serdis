//! CLI Parser
//! Parse CLI Arguments

extern crate clap;

pub mod start;
mod validator;

use clap::{Parser, Subcommand};
use start::StartCommandArguments;

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
