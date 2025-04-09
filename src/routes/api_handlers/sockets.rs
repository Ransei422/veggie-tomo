use axum::{
    extract::{Json, State},
    http::StatusCode
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use serde_json::{json, Value};
use sqlx::{query, query_as, FromRow};

use crate::state::AppState;
use super::relations::{search_by_family, search_by_name};



#[derive(Deserialize)]
pub struct VegetableNameJson {
    name: String,
}

#[derive(Deserialize)]
pub struct VegetableFamilyJson {
    family_name: String,
}


#[derive(Debug, FromRow, Serialize)]
pub struct VegetableData {
    pub id: i32,
    pub vegetable_name: String,
    pub vegetable_family: String,
    pub vegetable_species: String,
}


#[derive(Debug, FromRow, Serialize)]
pub struct AvailableData<'a> {
    pub id: usize,
    pub vegetable_name: &'a str,
    pub vegetable_family: &'a str,
    pub vegetable_species: &'a str,
}


pub async fn register_vegetables(State(app_state): State<Arc<AppState>>, Json(data): Json<VegetableNameJson>) -> Json<Value> {
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
                return json!({
                    "status_code": status_code,
                    "answer":"登録を完了した"}).into()
            },

            Err(err) => {
                println!("{}", err);
                let status_code = format!("{}", StatusCode::NOT_ACCEPTABLE);
                return json!({
                    "status_code": status_code,
                    "answer":"データが既にある"}).into()
            }
        }

    } else {
        let status_code = format!("{}", StatusCode::NOT_IMPLEMENTED);
        return json!({
            "status_code": status_code,
            "answer":"データが登録不可能になっている"}).into();
    }
}



async fn search_registered(
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


pub async fn check_registered_vegetable(app_state: State<Arc<AppState>>, Json(data): Json<VegetableNameJson>) -> Json<Value> {
    let result = search_registered(&app_state, &data.name).await;

    match result {
        Ok(veg) => { 
            let status_code = format!("{}", StatusCode::OK);

            return json!({
                    "status":status_code,
                    "vegetable_name": veg.vegetable_name,
                    "vegetable_family": veg.vegetable_family,
                    "vegetable_species": veg.vegetable_species
            }).into()
        },
        Err(_) => {
            let status_code = format!("{}", StatusCode::NOT_FOUND);

            return json!({
                    "status":status_code,
                    "vegetable_name": "",
                    "vegetable_family": "",
                    "vegetable_species": ""
            }).into()
        }
    }
}


pub async fn check_available_vegetable(Json(data): Json<VegetableFamilyJson>) -> Json<Value> {
    let family_name = data.family_name;
    let result = search_by_family(&family_name);

    if result.len() == 0 {
        let status_code = format!("{}", StatusCode::NOT_FOUND);
        return json!({
                "status":status_code,
                "vegetable_list": ""
        }).into()

    } else {
        let status_code = format!("{}", StatusCode::FOUND);

        let mut veg_list: Vec<AvailableData> = Vec::new();
        for (i, value) in result.iter().enumerate() {
            let d = AvailableData {
                id: i,
                vegetable_name: value.get_metadata().name,
                vegetable_family: value.get_metadata().family,
                vegetable_species: value.get_metadata().genus,
            };
            veg_list.push(d);
        }


        return json!({
                "status":status_code,
                "vegetable_list": veg_list
        }).into()
    }


   
}