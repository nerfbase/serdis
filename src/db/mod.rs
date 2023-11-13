//! Database

extern crate async_trait;
extern crate serde;
extern crate std;

pub mod backend;
pub mod model;

use self::backend::Backend;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

#[derive(Debug)]
pub struct Datastore<T>(pub Arc<T>)
where
    T: Backend + Send + Sync;

#[async_trait]
impl<T> Backend for Datastore<T>
where
    T: Backend + Sync + Send,
{
    async fn set<V: Send + Serialize + for<'de> Deserialize<'de>>(
        &self,
        key: String,
        val: V,
    ) -> Result<(), Box<dyn Error>> {
        self.0.set(key, val).await
    }

    async fn get<V: Send + for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Vec<V>, Box<dyn Error>> {
        self.0.get(key).await
    }

    async fn del<V: Send + for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.0.del::<V>(key).await
    }
}

#[cfg(test)]
mod tests {
    use crate::db::{
        backend::{
            surreal_impl::{self, SurrealDB},
            Backend,
        },
        model::{Insert, MetaData},
        Datastore,
    };
    use actix_web::rt::System;
    use std::sync::Arc;

    #[test]
    fn test_db_connection() {
        System::new().block_on(async {
            let db =
                surreal_impl::connect(Some("name".into()), Some("ns".into()), Some("res".into()))
                    .await;
            assert!(db.is_ok())
        });
    }

    #[test]
    fn test_db_insert_retrieve_delete() {
        System::new().block_on(async {
            let config =
                surreal_impl::connect(Some("name".into()), Some("ns".into()), Some("res".into()))
                    .await
                    .unwrap();

            let model = Insert {
                name: "service-xyz".into(),
                ip: "192.168.1.100".into(),
                port: 8080,
                health: "ok".into(),
                metadata: Some(MetaData {
                    env: Some("prod".into()),
                    version: Some("1.0.1".into()),
                    region: Some("us-east".into()),
                    team: Some("ops".into()),
                    tags: Some(vec!["web".into(), "backend".into()]),
                }),
            };

            let backend = SurrealDB {
                connection: config.connection,
                resource: config.resource,
                namespace: config.namespace,
                database_name: config.database_name,
            };

            let store = Datastore(Arc::new(backend));

            // Test data insertion
            let setter = store.set("service-xyz".into(), model.clone()).await;
            assert!(setter.is_ok());

            // Test data retrieval
            let getter = store.get::<Insert>("service-xyz").await;
            assert_eq!(getter.unwrap(), vec![model]);

            // Test data deletion
            let delete = store.del::<Insert>("service-xyz").await;
            assert!(delete.is_ok());

            // Test data retrieval after deletion
            let get_deleted = store.get::<Insert>("service-xyz").await;
            assert_eq!(get_deleted.unwrap(), vec![]);
        });
    }
}
