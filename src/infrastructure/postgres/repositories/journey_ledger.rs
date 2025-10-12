use std::sync::Arc;

use crate::{domain::repo::journey_ledger::JourneyLedgerRepository, infrastructure::postgres::postgres_connection::Pgpoolsquad};
use anyhow::Result;
use axum::async_trait;
pub struct JourneyLedgerPostgres{
    dbpool : Arc<Pgpoolsquad>,
}
impl JourneyLedgerPostgres {
    pub fn new(dbpool: Arc<Pgpoolsquad>) -> Self {
        Self { dbpool }
    }
}
#[async_trait]
impl JourneyLedgerRepository for JourneyLedgerPostgres {
        async fn in_journey(&self, quest_id: i32,guild_commander_id: i32) -> Result<i32>{
            unimplemented!()
        }
 async fn to_completed(&self, quest_id: i32,guild_commander_id: i32) -> Result<i32>{
            unimplemented!()
         }
 async fn to_failed(&self,quest_id: i32,guild_commander_id: i32) -> Result<i32>{
            unimplemented!()
        }
}