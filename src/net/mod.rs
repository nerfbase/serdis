//! Net

extern crate std;

use std::error::Error;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};

use crate::certs::temp_tls_cfg;

pub async fn init() -> Result<(), Box<dyn Error>> {
    let tls_config = temp_tls_cfg();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").route("test", web::get().to(test)))
            .wrap(middleware::NormalizePath::default())
    })
    .bind_rustls(format!("localhost:{}", 8082), tls_config)?
    .run()
    .await;

    Ok(())
}

async fn test() -> impl Responder {
    HttpResponse::Ok().json("ok")
}
