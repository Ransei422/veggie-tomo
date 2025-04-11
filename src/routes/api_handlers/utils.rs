// == File for functions not primary called, but used by sockets ==

use axum::extract::State;
use std::sync::Arc;
use sqlx::query_as;

use crate::state::AppState;
use super::sockets::VegetableData;



pub async fn search_registered(
    State(app_state): &State<Arc<AppState>>,
    vegetable_name: &str,
) -> Result<VegetableData, sqlx::Error> {

    let pool = &app_state.db_pool;
    let name_pattern = format!("%{}%", vegetable_name);

    let result = query_as::<_, VegetableData>(
        r#"
        SELECT id, vegetable_name, vegetable_family, vegetable_species
        FROM vegetables
        WHERE vegetable_name ILIKE $1
        "#
    )
    .bind(name_pattern)
    .fetch_one(pool).await;

    result
}