//! Serdis - Service Discovery

extern crate std;

mod certs;
mod cli;
mod cnf;
mod net;

use std::{future::Future, process::ExitCode};

fn main() -> ExitCode {
    init(cli::init())
}

fn init<T>(fut: impl Future<Output = T>) -> T {
    actix_web::rt::System::new().block_on(fut)
}
