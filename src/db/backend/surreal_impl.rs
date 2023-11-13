//! SurrealDB Backend

extern crate async_trait;
extern crate serde;
extern crate std;
extern crate surrealdb;

use crate::db::backend::Backend;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

pub const DEFAULT_NAMESPACE: &str = "serdis";
pub const DEFAULT_DATABASE_NAME: &str = "services";
pub const DEFAULT_RESOURCE: &str = "serdis_resource";

pub async fn connect(
    database_name: Option<String>,
    namespace: Option<String>,
    resource: Option<String>,
) -> Result<SurrealConfig, Box<dyn Error>> {
    let connection = Surreal::new::<Mem>(()).await?;
    let namespace = namespace.unwrap_or(DEFAULT_NAMESPACE.into());
    let database_name = database_name.unwrap_or(DEFAULT_DATABASE_NAME.into());
    let resource = resource.unwrap_or(DEFAULT_RESOURCE.into());

    connection.use_ns(&namespace).await?;
    connection.use_db(&database_name).await?;

    let config = SurrealConfig::new()
        .await
        .with_connection(connection)
        .with_namespace(namespace)
        .with_database_name(database_name)
        .with_resource(resource)
        .build();

    Ok(config)
}

#[derive(Debug)]
pub struct SurrealDB {
    pub connection: Surreal<Db>,
    pub resource: String,
    pub namespace: String,
    pub database_name: String,
}

#[async_trait]
impl Backend for SurrealDB {
    async fn set<V: Send + Serialize + for<'de> Deserialize<'de>>(
        &self,
        key: String,
        val: V,
    ) -> Result<(), Box<dyn Error>> {
        let value: Result<Vec<V>, surrealdb::Error> =
            self.connection.create(key).content(val).await;

        if let Err(err) = value {
            return Err(err.into());
        };

        Ok(())
    }

    async fn get<V: Send + for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Vec<V>, Box<dyn Error>> {
        let value: Result<Vec<V>, surrealdb::Error> = self.connection.select(key).await;

        match value {
            Ok(val) => Ok(val),
            Err(err) => Err(err.into()),
        }
    }

    async fn del<V: Send + for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<(), Box<dyn Error>> {
        let value: Result<Vec<V>, surrealdb::Error> = self.connection.delete(key).await;

        match value {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}

pub struct SurrealConfig {
    pub connection: Surreal<Db>,
    pub namespace: String,
    pub database_name: String,
    pub resource: String,
}

impl SurrealConfig {
    pub async fn new() -> Self {
        Self {
            connection: Surreal::new::<Mem>(()).await.unwrap(),
            namespace: Default::default(),
            database_name: Default::default(),
            resource: Default::default(),
        }
    }

    pub fn with_connection(mut self, connection: Surreal<Db>) -> Self {
        self.connection = connection;
        self
    }

    pub fn with_namespace(mut self, namespace: String) -> Self {
        self.namespace = namespace;
        self
    }

    pub fn with_database_name(mut self, database_name: String) -> Self {
        self.database_name = database_name;
        self
    }

    pub fn with_resource(mut self, resource: String) -> Self {
        self.resource = resource;
        self
    }

    pub fn build(self) -> Self {
        Self {
            connection: self.connection,
            namespace: self.namespace,
            database_name: self.database_name,
            resource: self.resource,
        }
    }
}
