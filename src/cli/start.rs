//! Start

extern crate clap;
extern crate log;
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
use log::info;
use std::error::Error;

#[derive(Args, Debug)]
pub struct StartCommandArguments {
    #[arg(long)]
    #[arg(help = "Hide the startup banner")]
    #[arg(default_value_t = false)]
    pub no_banner: bool,

    #[arg(long)]
    #[arg(help = "Database name")]
    pub database_name: Option<String>,

    #[arg(long)]
    #[arg(help = "Database namespace")]
    pub database_namespace: Option<String>,

    #[arg(long)]
    #[arg(help = "Database resource")]
    pub database_resource: Option<String>,

    #[command(subcommand)]
    pub command: ServerCommands,
}

pub async fn init(args: StartCommandArguments) -> Result<(), Box<dyn Error>> {
    // show/hide banner
    if !args.no_banner {
        println!("{LOGO}");
    }

    info!("🚀 Starting up!");

    let config = surreal_impl::connect(
        args.database_name,
        args.database_namespace,
        args.database_resource,
    )
    .await?;

    info!("💾 Established database connection");

    // setup the datastore
    let db = SurrealDB {
        connection: config.connection,
        database_name: config.database_name,
        namespace: config.namespace,
        resource: config.resource,
    };

    let store = Datastore(db.into());

    info!("💾 Started the datastore");

    // start the server
    match args.command {
        ServerCommands::Http(args) => net::http::init::<SurrealDB>(&args, store.into()).await,
        ServerCommands::Multi(args) => net::multi::init::<SurrealDB>(args, store.into()).await,
        ServerCommands::Rpc(args) => net::rpc::init(&args, store.into()).await,
    }
    .unwrap();

    info!("✋ Server stopped. Bye!");
    Ok(())
}
