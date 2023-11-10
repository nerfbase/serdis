//! Start

extern crate clap;
extern crate std;

use super::server::ServerCommands;
use crate::{
    cnf::LOGO,
    db::{
        backend::surreal_impl::{self, SurrealDB},
        Datastore,
    },
    net,
};
use clap::Args;
use std::error::Error;

#[derive(Args, Debug)]
pub struct StartCommandArguments {
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

    #[command(subcommand)]
    pub command: ServerCommands,
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
    let store = Datastore(SurrealDB(db).into());

    // start the server
    match args.command {
        ServerCommands::Http(args) => net::http::init::<SurrealDB>(&args, store.into()).await,
        ServerCommands::Multi(args) => net::multi::init::<SurrealDB>(args, store.into()).await,
        ServerCommands::Rpc(args) => net::rpc::init(&args, store.into()).await,
    }
    .unwrap();

    println!("Server stopped. Bye!");
    Ok(())
}
