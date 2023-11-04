//! Serdis - Service Discovery

extern crate actix_web;
extern crate std;

mod certs;
mod cli;
mod cnf;
mod db;
mod net;

use actix_web::rt::System;
use std::{future::Future, process::ExitCode};

fn main() -> ExitCode {
    init(cli::init())
}

fn init<T>(fut: impl Future<Output = T>) -> T {
    System::new().block_on(fut)
}
