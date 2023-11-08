//! Database Backends

extern crate async_trait;
extern crate std;

pub mod surreal_impl;

use super::model::Insert;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Backend: Sync + Send {
    async fn set(&self, key: String, val: Insert) -> Result<(), Box<dyn Error>>;
    async fn get(&self, key: &str) -> Result<Vec<Insert>, Box<dyn Error>>;
    async fn del(&self, key: &str) -> Result<(), Box<dyn Error>>;
}
