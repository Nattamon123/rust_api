use std::sync::Arc;

use crate::domain::{entites::guild_commanders::{GuildCommanderEntity, RegisterGuildCommanderEntity}, repo::guild_commanders::GuildCommandersRepository, value_object::guild_commander_model::RegisterGuildCommanderModel};
use anyhow::Result;
pub struct GuildCommanderUsecase<T>
where 
   T : GuildCommandersRepository+ Send + Sync 
{
    guild_commanders_repository: Arc<T>,

}
impl<T> GuildCommanderUsecase<T>
where 
   T : GuildCommandersRepository+ Send + Sync 
   {
    pub fn new(guild_commanders_repository: Arc<T>) -> Self {
        Self { guild_commanders_repository }
    }
    pub async fn  register( &self,mut  register_guild_commander_model: RegisterGuildCommanderModel) -> Result<i32>{
        unimplemented!()
    }

}