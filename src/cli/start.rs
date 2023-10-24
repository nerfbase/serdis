//! Start

extern crate clap;
extern crate std;

use crate::{cli::validator, cnf::LOGO};
use clap::Args;
use std::{error::Error, path::PathBuf};

#[derive(Args, Debug)]
pub struct StartCommandArguments {
    #[arg(help = "Port number for the server")]
    #[arg(long = "port", short = 'p')]
    pub port: u16,

    #[arg(help = "Path to the CERT certificate")]
    #[arg(long = "cert", value_parser = validator::file_exists)]
    pub pem_file: Option<PathBuf>,

    #[arg(help = "Path to the KEY certificate")]
    #[arg(long = "key", value_parser = validator::file_exists)]
    pub key_file: Option<PathBuf>,

    #[arg(long)]
    #[arg(help = "Hide the startup banner")]
    #[arg(default_value_t = false)]
    no_banner: bool,
}

pub async fn init(
    StartCommandArguments {
        port,
        pem_file,
        key_file,
        no_banner,
    }: StartCommandArguments,
) -> Result<(), Box<dyn Error>> {
    if !no_banner {
        println!(r"{LOGO}");
    }

    Ok(())
}
