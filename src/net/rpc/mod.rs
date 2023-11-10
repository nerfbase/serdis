//! RPC

extern crate std;
extern crate tonic;

mod serdis_rpc;

use self::serdis_rpc::Parameter;
use crate::{db::Datastore, net::Backend};
use serdis_rpc::{
    serdis_server::{Serdis, SerdisServer},
    Deregister, Info, Insert,
};
use std::{error::Error, net::SocketAddr};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug)]
pub struct SerdisRPC<T: Backend>(pub Datastore<T>);

impl<T> SerdisRPC<T>
where
    T: Backend,
{
    pub fn new(store: Datastore<T>) -> Self {
        Self(store)
    }
}

#[tonic::async_trait]
impl<T> Serdis for SerdisRPC<T>
where
    T: Backend + 'static,
{
    async fn register(&self, payload: Request<Insert>) -> Result<Response<Parameter>, Status> {
        let payload = payload.into_inner();
        let key = payload.name.clone();

        if let Err(error) = self.0.set::<Insert>(key, payload).await {
            return Err(Status::internal(error.to_string()));
        };

        Ok(Response::new(Parameter::new(
            "Service registered successfully!".into(),
        )))
    }

    async fn info(&self, payload: Request<Info>) -> Result<Response<Insert>, Status> {
        let key = payload.into_inner().service;

        if let Ok(value) = self.0.get::<Insert>(&key).await {
            return Ok(Response::new(value[0].clone()));
        }

        todo!("after fixing surreal_impl.rs")
    }

    async fn deregister(
        &self,
        payload: Request<Deregister>,
    ) -> Result<Response<Parameter>, Status> {
        let key = payload.into_inner().service;

        if let Err(error) = self.0.del::<Insert>(&key).await {
            return Err(Status::internal(error.to_string()));
        }

        Ok(Response::new(Parameter::new(
            "Service deregistered successfully!".into(),
        )))
    }
}

impl Parameter {
    fn new(message: String) -> Self {
        Self { message }
    }
}

pub async fn init<T>(port: u16, store: Datastore<T>) -> Result<(), Box<dyn Error>>
where
    T: Backend + 'static,
{
    let addr = format!("[::1]:{}", port).parse::<SocketAddr>()?;
    let rpc_service = SerdisRPC::new(store);

    Server::builder()
        .add_service(SerdisServer::new(rpc_service))
        .serve(addr)
        .await?;

    Ok(())
}
