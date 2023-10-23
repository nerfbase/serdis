//! Serdis - Service Discovery

mod certs;
mod cli;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use certs::temp_tls_cfg;
use clap::Parser;
use cli::Cli;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let tls_config = temp_tls_cfg();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").route("test", web::get().to(test)))
            .wrap(middleware::NormalizePath::default())
    })
    .bind_rustls(format!("localhost:{}", args.port), tls_config)?
    .run()
    .await
}

async fn test() -> impl Responder {
    HttpResponse::Ok().json("ok")
}
