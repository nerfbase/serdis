//! Database

extern crate async_trait;
extern crate std;

pub mod backend;
pub mod model;

use self::{backend::Backend, model::Insert};
use async_trait::async_trait;
use std::error::Error;

#[derive(Debug)]
pub struct Datastore<T: Backend>(pub T);

#[async_trait]
impl<T> Backend for Datastore<T>
where
    T: Backend + Sync + Send,
{
    async fn set(&self, key: String, val: Insert) -> Result<(), Box<dyn Error>> {
        self.0.set(key, val).await
    }

    async fn get(&self, key: &str) -> Result<Vec<Insert>, Box<dyn Error>> {
        self.0.get(key).await
    }

    async fn del(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.0.del(key).await
    }
}

#[cfg(test)]
mod tests {
    use actix_web::rt::System;

    use crate::db::{
        backend::{
            surreal_impl::{self, SurrealDB},
            Backend,
        },
        model::{Insert, MetaData},
        Datastore,
    };

    #[test]
    fn test_db_connection() {
        System::new().block_on(async {
            let db = surreal_impl::connect(Some(&"name".into()), Some(&"ns".into())).await;
            assert!(db.is_ok())
        });
    }

    #[test]
    fn test_db_insert_retrieve_delete() {
        System::new().block_on(async {
            let db = surreal_impl::connect(Some(&"name".into()), Some(&"ns".into()))
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

            let backend = SurrealDB(&db);
            let store = Datastore(backend);

            // Test data insertion
            let setter = store.set("service-xyz".into(), model.clone()).await;
            assert!(setter.is_ok());

            // Test data retrieval
            let getter = store.get("service-xyz").await;
            assert_eq!(getter.unwrap(), vec![model]);

            // Test data deletion
            let delete = store.del("service-xyz").await;
            assert!(delete.is_ok());

            // Test data retrieval after deletion
            let get_deleted = store.get("service-xyz").await;
            assert_eq!(get_deleted.unwrap(), vec![]);
        });
    }
}
