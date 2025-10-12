use std::sync::Arc;

use axum::{extract::{Path, State}, response::IntoResponse, Extension, Router};

use crate::{application::usecase::crew_switchboard::{self, CrewSwitchBoardUseCase}, config::stage::Stage, domain::repo::{crew_switchboard::CrewSwitchBoardRepository, quest_viewing::QuestViewingRepository}, infrastructure::postgres::{postgres_connection::Pgpoolsquad, repositories::{crew_switchboard::CrewSwitchBoardPostgres, quest_viewing::QuestViewingPostgres}}};

pub fn routes(db_pool:Arc<Pgpoolsquad>) -> Router {
    let crew_switchboard_repository = CrewSwitchBoardPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let crew_switchboard_usecase = CrewSwitchBoardUseCase::new(Arc::new(crew_switchboard_repository),Arc::new(quest_viewing_repository)
);
    Router::new()
}
pub async fn join<T1,T2>(
    State(crew_switchboard_usecase): State<Arc<CrewSwitchBoardUseCase<T1,T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>
) -> impl IntoResponse
where 
    T1:CrewSwitchBoardRepository + Send +Sync,
    T2: QuestViewingRepository + Send + Sync

{
  
}