//! Payload Models

extern crate prost;
extern crate serde;

use prost::Message;
use serde::{Deserialize, Serialize};

// HTTP

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Insert {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub health: String,
    pub metadata: Option<MetaData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MetaData {
    pub env: Option<String>,
    pub version: Option<String>,
    pub region: Option<String>,
    pub team: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub service: String,
}

// RPC

// #[derive(Serialize, Deserialize, Clone, PartialEq, Message)]
// pub struct InsertProto {
//     #[prost(string, tag = "1")]
//     pub name: String,

//     #[prost(string, tag = "2")]
//     pub ip: String,

//     #[prost(uint32, tag = "3")]
//     pub port: u32,

//     #[prost(string, tag = "4")]
//     pub health: String,

//     #[prost(message, tag = "5")]
//     pub metadata: Option<MetaDataProto>,
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq, Message)]
// pub struct MetaDataProto {
//     #[prost(string, tag = "1")]
//     pub env: ::prost::alloc::string::String,

//     #[prost(string, tag = "2")]
//     pub version: ::prost::alloc::string::String,

//     #[prost(string, tag = "3")]
//     pub region: ::prost::alloc::string::String,

//     #[prost(string, tag = "4")]
//     pub team: ::prost::alloc::string::String,

//     #[prost(string, repeated, tag = "5")]
//     pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq, Message)]
// pub struct ParameterProto {
//     #[prost(string, tag = "1")]
//     pub message: String,
// }
