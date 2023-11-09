//! Database Backends

extern crate async_trait;
extern crate std;

pub mod surreal_impl;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[async_trait]
pub trait Backend: Sync + Send {
    async fn set<V: Send + Serialize + for<'de> Deserialize<'de>>(
        &self,
        key: String,
        val: V,
    ) -> Result<(), Box<dyn Error>>;

    async fn get<V: Send + for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Vec<V>, Box<dyn Error>>;

    async fn del<V: Send + for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<(), Box<dyn Error>>;
}
