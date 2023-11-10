//! Parse Server Args

extern crate clap;
extern crate std;

use super::validator;
use clap::{command, Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand, Clone)]
pub enum ServerCommands {
    #[command(about = "Start HTTP server")]
    Http(ServerCommandArguments),

    #[command(
        about = "Start both HTTP and RPC servers \nwith a shared database \nand configuration"
    )]
    Multi(MutliServerCommandArguments),

    #[command(about = "Start RPC server")]
    Rpc(ServerCommandArguments),
}

#[derive(Args, Debug, Clone)]
pub struct MutliServerCommandArguments {
    // HTTP Server Args
    #[arg(help = "Port number for the HTTP server")]
    #[arg(long = "http-port")]
    pub http_port: u16,

    #[arg(help = "Path to the HTTP CERT certificate")]
    #[arg(long = "http-cert", value_name = "FILE", value_parser = validator::file_exists)]
    pub http_cert: Option<PathBuf>,

    #[arg(help = "Path to the HTTP KEY certificate")]
    #[arg(long = "http-key", value_name = "FILE", value_parser = validator::file_exists)]
    pub http_key: Option<PathBuf>,

    // RPC Server Args
    #[arg(help = "Port number for the RPC server")]
    #[arg(long = "rpc-port")]
    pub rpc_port: u16,

    #[arg(help = "Path to the RPC CERT certificate")]
    #[arg(long = "rpc-cert", value_name = "FILE", value_parser = validator::file_exists)]
    pub rpc_cert: Option<PathBuf>,

    #[arg(help = "Path to the RPC KEY certificate")]
    #[arg(long = "rpc-key", value_name = "FILE", value_parser = validator::file_exists)]
    pub rpc_key: Option<PathBuf>,
}

#[derive(Args, Debug, Clone)]
pub struct ServerCommandArguments {
    #[arg(help = "Port number for the server")]
    #[arg(long = "port", short = 'p')]
    pub port: u16,

    #[arg(help = "Path to the CERT certificate")]
    #[arg(long = "cert", value_name = "FILE", value_parser = validator::file_exists)]
    pub cert: Option<PathBuf>,

    #[arg(help = "Path to the KEY certificate")]
    #[arg(long = "key", value_name = "FILE", value_parser = validator::file_exists)]
    pub key: Option<PathBuf>,
}

impl ServerCommandArguments {
    pub fn new() -> Self {
        Self {
            port: Default::default(),
            cert: None,
            key: None,
        }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn cert(mut self, cert: Option<PathBuf>) -> Self {
        self.cert = cert;
        self
    }

    pub fn key(mut self, key: Option<PathBuf>) -> Self {
        self.key = key;
        self
    }
}
