use std::sync::Arc;

use crate::{domain::{entites::guild_commanders::{GuildCommanderEntity, RegisterGuildCommanderEntity}, repo::guild_commanders::GuildCommandersRepository}, infrastructure::postgres::postgres_connection::Pgpoolsquad};
use anyhow::Result;
use axum::async_trait;
pub struct GuildCommandersPostgres{
    dbpool : Arc<Pgpoolsquad>
}
 impl GuildCommandersPostgres {
    pub fn new(dbpool: Arc<Pgpoolsquad>) -> Self {
        Self { dbpool }
    }
}
#[async_trait]
impl GuildCommandersRepository for GuildCommandersPostgres {
       async fn register(&self, register_guild_commander_entity: RegisterGuildCommanderEntity) -> Result<i32>{
        unimplemented!()
       }
        async fn find_by_username(&self, username: String) -> Result<GuildCommanderEntity>{
            unimplemented!()
        }
}