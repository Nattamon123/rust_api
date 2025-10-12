use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use crate::{application::usecase::adventurer::AdventurerUseCase, domain::{repo::adventurers::AdventurersRepository, value_object::adventurer_model::RegisterAdventurerModel}, infrastructure::postgres::{postgres_connection::Pgpoolsquad, repositories::adventurers::AdventurerPostgres}};

pub fn routes(db_pool:Arc<Pgpoolsquad>) -> Router {
    let adventurer_repository = AdventurerPostgres::new(db_pool);
    let adventurer_usecase = AdventurerUseCase::new(Arc::new(adventurer_repository));

    Router::new()
    .route(("/"), post(register))
    .with_state(Arc::new(adventurer_usecase))
}
pub async fn register<T>(
    State(adventurer_usecase): State<Arc<AdventurerUseCase<T>>>,
    Json(register_adventurer_model): Json<RegisterAdventurerModel>,
) -> impl IntoResponse
where 
    T:AdventurersRepository + Send + Sync,
 {
}