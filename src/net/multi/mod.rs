//! Dual

extern crate futures;
extern crate std;

use crate::{
    cli::server::{MutliServerCommandArguments, ServerCommandArguments},
    db::{backend::Backend, Datastore},
};
use std::{error::Error, sync::Arc};

pub async fn init<T>(
    MutliServerCommandArguments {
        http_port,
        http_cert,
        http_key,
        rpc_port,
        rpc_cert,
        rpc_key,
    }: MutliServerCommandArguments,

    store: Arc<Datastore<T>>,
) -> Result<(), Box<dyn Error>>
where
    T: Backend + 'static,
{
    let rpc_store = store.clone();

    let http_args = ServerCommandArguments::new()
        .with_port(http_port)
        .with_cert(http_cert)
        .with_key(http_key);

    let rpc_args = ServerCommandArguments::new()
        .with_port(rpc_port)
        .with_cert(rpc_cert)
        .with_key(rpc_key);

    let http_handler = actix_web::rt::spawn(async move {
        super::http::init::<T>(&http_args, store).await.unwrap();
    });

    let rpc_handler = actix_web::rt::spawn(async move {
        super::rpc::init(&rpc_args, rpc_store).await.unwrap();
    });

    futures::future::try_join(http_handler, rpc_handler)
        .await
        .unwrap();

    Ok(())
}
