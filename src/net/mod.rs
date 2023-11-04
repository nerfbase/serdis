//! Net

extern crate actix_web;
extern crate std;

mod tls;

use crate::{cli::start::StartCommandArguments, db::DB};
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
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
    let db = Data::new(DB::new());

    let server = HttpServer::new(move || {
        App::new()
            .service(web::scope("/api").route("test", web::get().to(test)))
            .wrap(middleware::NormalizePath::default())
            .app_data(db.clone())
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
