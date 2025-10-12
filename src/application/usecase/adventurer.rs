use std::sync::Arc;

use crate::domain::{repo::adventurers::AdventurersRepository, value_object::adventurer_model::RegisterAdventurerModel};
use anyhow::Result;
pub struct AdventurerUseCase<T>
where 
   T : AdventurersRepository+ Send + Sync 
{
    adventurers_repository: Arc<T>,
}
impl<T> AdventurerUseCase<T>
where 
    T : AdventurersRepository+ Send + Sync
{
    pub fn new(adventurers_repository:Arc<T>) -> Self {
        Self { adventurers_repository }
    }
    pub async fn register(&self,mut register_adventurer_model: RegisterAdventurerModel) -> anyhow::Result<i32>{
        unimplemented!()
    }
}
