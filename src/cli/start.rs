//! Start

extern crate clap;
extern crate std;

use crate::{cli::validator, cnf::LOGO, net};
use clap::Args;
use std::{error::Error, path::PathBuf};

#[derive(Args, Debug)]
pub struct StartCommandArguments {
    #[arg(help = "Port number for the server")]
    #[arg(long = "port", short = 'p')]
    pub port: u16,

    #[arg(help = "Path to the CERT certificate")]
    #[arg(long = "cert", value_name = "FILE", value_parser = validator::file_exists)]
    pub cert_file: Option<PathBuf>,

    #[arg(help = "Path to the KEY certificate")]
    #[arg(long = "key", value_name = "FILE", value_parser = validator::file_exists)]
    pub key_file: Option<PathBuf>,

    #[arg(long)]
    #[arg(help = "Hide the startup banner")]
    #[arg(default_value_t = false)]
    pub no_banner: bool,
}

pub async fn init(args: StartCommandArguments) -> Result<(), Box<dyn Error>> {
    // show/hide banner
    if !args.no_banner {
        println!("{LOGO}");
    }

    // start the server
    net::init(args).await?;

    Ok(())
}
