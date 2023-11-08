//! Start

extern crate clap;
extern crate std;

use crate::{
    cli::validator,
    cnf::LOGO,
    db::{
        backend::surreal_impl::{self, SurrealDB},
        Datastore,
    },
    net,
};
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

    #[arg(long)]
    #[arg(help = "Database name")]
    pub db_name: Option<String>,

    #[arg(long)]
    #[arg(help = "Database namespace")]
    pub db_ns: Option<String>,
}

pub async fn init(args: StartCommandArguments) -> Result<(), Box<dyn Error>> {
    // show/hide banner
    if !args.no_banner {
        println!("{LOGO}");
    }

    // start the database
    let db = match (&args.db_name, &args.db_ns) {
        (Some(name), Some(ns)) => surreal_impl::connect(Some(name), Some(ns)).await,
        (Some(name), None) => surreal_impl::connect(Some(name), None).await,
        (None, Some(ns)) => surreal_impl::connect(None, Some(ns)).await,
        (None, None) => surreal_impl::connect(None, None).await,
    }?;

    // setup the datastore
    let store = Datastore(SurrealDB(&db));

    // start the server
    net::init::<SurrealDB>(args, store).await?;

    Ok(())
}
