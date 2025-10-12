use std::sync::Arc;

use crate::{domain::{entites::quests::QuestEntity, repo::quest_viewing::QuestViewingRepository, value_object::board_checking_filter::BoardCheckingFilter}, infrastructure::postgres::postgres_connection::Pgpoolsquad};
use anyhow::Result;
use axum::async_trait;
pub struct QuestViewingPostgres{
    dbpool : Arc<Pgpoolsquad>
}
impl QuestViewingPostgres {
    pub fn new(dbpool: Arc<Pgpoolsquad>) -> Self {
        Self { dbpool }
    }
}
#[async_trait]
impl QuestViewingRepository for QuestViewingPostgres {
     async fn view_details(&self, quest_id: i32) -> Result<QuestEntity>{
        unimplemented!()
     }
    async fn board_checking(&self, filter:&BoardCheckingFilter) -> Result<Vec<QuestEntity>>{
        unimplemented!()
    }
    async fn adventurer_counting_by_quest_id(&self, quest_id: i32) -> Result<i64>{
        unimplemented!()
    }
}