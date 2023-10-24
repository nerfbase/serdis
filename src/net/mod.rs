//! Net

extern crate actix_web;
extern crate std;

mod tls;

use crate::cli::start::StartCommandArguments;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use std::error::Error;
use tls::tls_cfg;

pub async fn init(
    StartCommandArguments {
        port,
        cert_file,
        key_file,
        no_banner: _,
    }: StartCommandArguments,
) -> Result<(), Box<dyn Error>> {
    let server = HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").route("test", web::get().to(test)))
            .wrap(middleware::NormalizePath::default())
    });

    if let (Some(crt), Some(key)) = (&cert_file, &key_file) {
        let tls = tls_cfg(crt, key);
        server
            .bind_rustls(format!("localhost:{}", port), tls)
            .unwrap()
            .run()
            .await?;
    } else {
        server
            .bind(format!("localhost:{}", port))
            .unwrap()
            .run()
            .await?;
    };

    println!("Server stopped. Bye!");
    Ok(())
}

async fn test() -> impl Responder {
    HttpResponse::Ok().json("ok")
}
