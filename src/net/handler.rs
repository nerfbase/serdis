//! Handle Endpoints

extern crate actix_web;
extern crate std;

use crate::db::{
    model::{Insert, Parameter},
    DB,
};
use actix_web::{
    web::{Data, Json, Query},
    HttpResponse, Responder,
};
use std::collections::HashMap;

pub async fn register(payload: Json<Insert>, db: Data<DB>) -> impl Responder {
    if let Err(error) = db.set(payload.name.to_owned(), payload.0) {
        return Response::Conflict(error.to_string()).message();
    }

    Response::Created.message()
}

pub async fn info(payload: Query<Parameter>, db: Data<DB>) -> impl Responder {
    if let Ok(value) = db.get(&payload.service) {
        return HttpResponse::Ok().json(value);
    }

    Response::NotFound.message()
}

pub async fn deregister(payload: Query<Parameter>, db: Data<DB>) -> impl Responder {
    if let Err(error) = db.del(&payload.service) {
        return HttpResponse::InternalServerError().json(format!("Error: {}", error));
    }

    Response::Ok("Service deregistered successfully".to_string()).message()
}

pub enum Response {
    Created,
    Conflict(String),
    NotFound,
    Ok(String),
}

impl Response {
    pub fn message(&self) -> HttpResponse {
        let mut map = HashMap::new();
        match self {
            Response::Created => {
                map.insert("message", "Resource created successfully");
                HttpResponse::Created().json(map)
            }
            Response::Conflict(reason) => {
                map.insert("message", "Conflicting keys");
                map.insert("reason", reason);
                HttpResponse::Conflict().json(map)
            }
            Response::NotFound => {
                map.insert("message", "Unable to get the service info");
                HttpResponse::NotFound().json(map)
            }
            Response::Ok(message) => {
                map.insert("message", message);
                HttpResponse::Ok().json(map)
            }
        }
    }
}
