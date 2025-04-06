use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Html,
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


// GET method to server with database counter increase
pub async fn _main_page_get(State(app_state): State<Arc<AppState>>,) -> impl axum::response::IntoResponse {
    let pool = &app_state.db_pool;
    
    let rows = query!("SELECT * FROM users")
        .fetch_all(pool)
        .await
        .unwrap_or_else(|_| vec![]);

    (StatusCode::OK, Html(format!("<h1>Rows: {}</h1>", rows.len())))
}



pub async fn register_vegetables(Json(data): Json<VegetableJson>) -> String {
    if let Some(veg) = search_by_name(&data.name) {
        let meta = veg.get_metadata();
        println!("✅ Found: {}-種 / {}科 / {}目", meta.name, meta.genus, meta.family);
    } else {
        println!("❌ Not found");
    }

    String::new()
}


// GET Demo Json data
pub async fn get_demo_json() -> Json<Value> {
    json!({"a":"b"}).into()
}


// PUT Demo Json data
pub async fn put_demo_json(Json(data): Json<serde_json::Value>) -> String {
    let json_data = format!("Put demo JSON data: {:?}", data);
    println!("{:?}", json_data);
    json_data
}