//! SurrealDB Backend

extern crate async_trait;
extern crate std;
extern crate surrealdb;

use crate::{
    cnf::{DEFAULT_DATABASE_NAME, DEFAULT_NAMESPACE},
    db::backend::Backend,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

pub async fn connect(
    name: Option<&String>,
    ns: Option<&String>,
) -> Result<Surreal<Db>, Box<dyn Error>> {
    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns(ns.unwrap_or(&DEFAULT_NAMESPACE.to_string()))
        .await?;
    db.use_db(name.unwrap_or(&DEFAULT_DATABASE_NAME.to_string()))
        .await?;

    Ok(db)
}

#[derive(Debug)]
pub struct SurrealDB(pub Surreal<Db>);

#[async_trait]
impl Backend for SurrealDB {
    async fn set<V: Send + Serialize + for<'de> Deserialize<'de>>(
        &self,
        key: String,
        val: V,
    ) -> Result<(), Box<dyn Error>> {
        let value: Result<Vec<V>, surrealdb::Error> = self.0.create(key).content(val).await;

        if let Err(err) = value {
            return Err(err.into());
        };

        Ok(())
    }

    async fn get<V: Send + for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Vec<V>, Box<dyn Error>> {
        let value: Result<Vec<V>, surrealdb::Error> = self.0.select(key).await;

        match value {
            Ok(val) => Ok(val),
            Err(err) => Err(err.into()),
        }
    }

    async fn del<V: Send + for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<(), Box<dyn Error>> {
        let value: Result<Vec<V>, surrealdb::Error> = self.0.delete(key).await;

        match value {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}
