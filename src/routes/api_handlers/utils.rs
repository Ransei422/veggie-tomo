// == File for functions not primary called, but used by sockets ==

use axum::extract::State;
use std::sync::Arc;
use sqlx::{
    postgres::PgQueryResult,
    query,
    query_as
};

use crate::state::AppState;

use super::{
    relations::Vegetable,
    sockets::VegetableData
};



pub async fn register_vegetable_to_db(
    State(app_state): &State<Arc<AppState>>,
    veg: Vegetable,
) -> Result<PgQueryResult, sqlx::Error> {

    let meta = veg.get_metadata();
    let pool = &app_state.db_pool;
    let result = query!(
        r#"
        INSERT INTO vegetables (vegetable_name, vegetable_species, vegetable_family)
        VALUES ($1, $2, $3)
        "#,
        meta.name,
        meta.genus,
        meta.family
    )
    .execute(pool)
    .await;

    return result;
}



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



pub async fn register_relationship_to_db(
    State(app_state): &State<Arc<AppState>>,
    veg_1_id: i32,
    veg_2_id: i32,
    compatibility: usize,
    explanation: String,
) -> Result<PgQueryResult, sqlx::Error> {
    let pool = &app_state.db_pool;

    let (v1, v2) = if veg_1_id < veg_2_id {
        (veg_1_id, veg_2_id)
    } else {
        (veg_2_id, veg_1_id)
    };

    sqlx::query!(
        r#"
        INSERT INTO compatibility_relations (
            vegetable_id_1, vegetable_id_2, compatibility, explanation
        ) VALUES ($1, $2, $3, $4)
        ON CONFLICT (
            LEAST(vegetable_id_1, vegetable_id_2),
            GREATEST(vegetable_id_1, vegetable_id_2)
        )
        DO UPDATE SET
            compatibility = EXCLUDED.compatibility,
            explanation = EXCLUDED.explanation
        "#,
        v1,
        v2,
        compatibility as i32,
        explanation,
    )
    .execute(pool)
    .await
}