use std::sync::Arc;

use crate::{domain::{entites::quests::{AddQuestEntity, EditQuestEntity}, repo::quest_ops::{self, QuestOpsRepository}}, infrastructure::postgres::postgres_connection::Pgpoolsquad};
use anyhow::Result;
use axum::async_trait;
pub struct QuestOpsPostgres{
    dbpool: Arc<Pgpoolsquad>
}
impl QuestOpsPostgres{
    pub fn new(dbpool: Arc<Pgpoolsquad>) -> Self {
        Self { dbpool }
    }
}
#[async_trait]
impl QuestOpsRepository for QuestOpsPostgres{
        async fn add(&self, add_quest_entity: AddQuestEntity) -> Result<i32>{
            unimplemented!()
        }
    async fn edit(&self, quest_id:i32, edit_quest_entity:EditQuestEntity) -> Result<i32>{
            unimplemented!()
    }
    async fn remove(&self, quest_id:i32,guild_commander_id:i32) -> Result<()>{
            unimplemented!()
    }
}