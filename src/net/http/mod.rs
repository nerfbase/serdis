//! HTTP Server

extern crate actix_web;
extern crate log;
extern crate std;

mod handler;

use self::handler::{deregister, info, register};
use super::tls::tls_cfg;
use crate::{
    cli::server::ServerCommandArguments,
    cnf::DEFAULT_HOST,
    db::{backend::Backend, Datastore},
};
use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use log::{error, info};
use std::{error::Error, sync::Arc};

pub async fn init<T: Backend>(
    ServerCommandArguments { port, cert, key }: &ServerCommandArguments,
    store: Arc<Datastore<T>>,
) -> Result<(), Box<dyn Error>> {
    let store = Data::new(store);
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .route("test", web::get().to(test))
                    .route("register", web::post().to(register::<T>))
                    .route("info", web::get().to(info::<T>))
                    .route("deregister", web::delete().to(deregister::<T>)),
            )
            .app_data(store.clone())
    });

    let addr = format!("{DEFAULT_HOST}:{port}");
    if let (Some(crt), Some(key)) = (&cert, &key) {
        let tls = tls_cfg(crt, key);
        info!("🌐 Started HTTPS Server");

        match server.bind_rustls(&addr, tls).unwrap().run().await {
            Ok(_) => info!("🌐 Stopped HTTPS Server"),
            Err(error) => error!("{error}"),
        };
    } else {
        info!("🌐 Started HTTP Server");

        match server.bind(&addr).unwrap().run().await {
            Ok(_) => info!("🌐 Stopped HTTP Server"),
            Err(error) => error!("🌐 HTTP Server Error: {error}"),
        };
    };

    Ok(())
}

async fn test() -> impl Responder {
    HttpResponse::Ok().json("ok")
}
