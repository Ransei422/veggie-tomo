use axum::{
    extract::{Json, State},
    http::StatusCode
};
use serde::Deserialize;
use std::sync::Arc;
use serde_json::{json, Value};
use sqlx::query;

use crate::state::AppState;
use super::relations::search_by_name;


#[derive(Deserialize)]
pub struct VegetableJson {
    name: String,
}


pub async fn register_vegetables(State(app_state): State<Arc<AppState>>, Json(data): Json<VegetableJson>) -> Json<Value> {
    if let Some(veg) = search_by_name(&data.name) {
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

        match result {
            Ok(_) => {
                let status_code = format!("{}", StatusCode::OK);
                return json!({status_code:"登録を完了した"}).into()
            },

            Err(err) => {
                println!("{}", err);
                let status_code = format!("{}", StatusCode::NOT_ACCEPTABLE);
                return json!({status_code:"データが既にある"}).into()
            }
        }

    } else {
        let status_code = format!("{}", StatusCode::NOT_ACCEPTABLE);
        return json!({status_code:"データが登録不可能になっている"}).into();
    }
}