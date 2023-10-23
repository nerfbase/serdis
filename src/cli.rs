//! CLI Parser
//! Parse CLI Arguments

extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 1081)]
    pub port: u16,
}
