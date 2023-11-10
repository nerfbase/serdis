//! HTTP Server

mod handler;

use self::handler::{deregister, info, register};
use super::tls::tls_cfg;
use crate::{
    cli::start::StartCommandArguments,
    db::{backend::Backend, Datastore},
};
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use std::error::Error;

pub async fn init<T: Backend>(
    StartCommandArguments {
        port,
        cert_file,
        key_file,
        no_banner: _,
        db_name: _,
        db_ns: _,
        mode: _,
    }: &StartCommandArguments,

    backend: Datastore<T>,
) -> Result<(), Box<dyn Error>> {
    let store = Data::new(backend);
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .route("test", web::get().to(test))
                    .route("register", web::post().to(register::<T>))
                    .route("info", web::get().to(info::<T>))
                    .route("deregister", web::delete().to(deregister::<T>)),
            )
            .wrap(middleware::NormalizePath::default())
            .app_data(store.clone())
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

    Ok(())
}

async fn test() -> impl Responder {
    HttpResponse::Ok().json("ok")
}
