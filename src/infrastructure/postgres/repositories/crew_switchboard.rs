use std::sync::Arc;

use crate::{
    domain::{
        repo::crew_switchboard::CrewSwitchBoardRepository,
        value_object::quest_adventurer_junction::QuestAdventurerJunction,
    },
    infrastructure::postgres::postgres_connection::Pgpoolsquad,
};
use anyhow::Result;
use axum::async_trait;
pub struct CrewSwitchBoardPostgres {
    dbpool: Arc<Pgpoolsquad>,
}
impl CrewSwitchBoardPostgres {
    pub fn new(dbpool: Arc<Pgpoolsquad>) -> Self {
        Self { dbpool }
    }
}
#[async_trait]
impl CrewSwitchBoardRepository for CrewSwitchBoardPostgres {
    async fn join(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        unimplemented!()
    }
    async fn leave(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        unimplemented!()
    }
}
