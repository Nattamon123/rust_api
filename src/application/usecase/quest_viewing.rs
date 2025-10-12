use crate::domain::{repo::quest_viewing::QuestViewingRepository, value_object::{board_checking_filter::BoardCheckingFilter, quest_model::QuestModel}};
use std::sync::Arc;
use anyhow::Result;
pub struct QuestViewingUseCase<T> 
where 
   T: QuestViewingRepository + Send + Sync
   {
   quest_viewing_repository: T
   }
impl<T> QuestViewingUseCase<T>
where
   T: QuestViewingRepository + Send + Sync
   {
   pub fn new(quest_viewing_repository:T) -> Self {
      Self { quest_viewing_repository }
   }
   pub async fn view_detail(&self,quest_id:i32) -> Result<QuestModel>{
    unimplemented!()
   }
   pub async fn board_checking(&self,filter: &BoardCheckingFilter) -> Result<Vec<QuestModel>>{
    unimplemented!()
   }
}