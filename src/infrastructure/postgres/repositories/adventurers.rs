use std::sync::Arc;

use axum::async_trait;
use anyhow::Result;
use crate::{domain::{entites::adventurers::{AdventurerEntity, RegisterAdventurerEntity}, repo::adventurers::AdventurersRepository}, infrastructure::postgres::postgres_connection::Pgpoolsquad};

pub struct AdventurerPostgres{
    db_pool:Arc<Pgpoolsquad>
}
impl AdventurerPostgres {
    pub fn new(db_pool: Arc<Pgpoolsquad>) -> Self {
        Self { db_pool }
    }
}
#[async_trait]
impl AdventurersRepository for AdventurerPostgres {
        async fn register(&self,register_adventurer_entity:RegisterAdventurerEntity) -> Result<i32>{
            unimplemented!()
        }
    async fn find_by_username(&self,username:String) -> Result<AdventurerEntity>{
        unimplemented!()
    }
}