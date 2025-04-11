// == File for primary called APIs ==

use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use serde_json::{json, Value};
use sqlx::{query, FromRow};

use crate::state::AppState;
use super::relations::{search_by_family, search_by_name};
use super::answers;
use super::utils;



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



// Register data from relations.rs to DB based on user's input
pub async fn register_vegetables(State(app_state): State<Arc<AppState>>,
                                Json(data): Json<VegetableNameJson>)
                                -> Json<Value> {
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
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer1);
                return json!({
                    "status_code": answer.code.as_str(),
                    "status_answer":answer.message}).into()
            },

            Err(_err) => {
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer2);
                    return json!({
                        "status_code": answer.code.as_str(),
                        "status_answer":answer.message}).into()
            }
        }

    } else {
        let answer = answers::ApiAnswers::new(
            answers::ApiAnswersEnum::Answer3);
            return json!({
                "status_code": answer.code.as_str(),
                "status_answer":answer.message}).into()
    }
}



// Check if vegetable already exists in DB 
pub async fn check_registered_vegetable(app_state: State<Arc<AppState>>, Json(data): Json<VegetableNameJson>) -> Json<Value> {
    let result = utils::search_registered(&app_state, &data.name).await;

    match result {
        Ok(veg) => { 
            
            let answer = answers::ApiAnswers::new(
                answers::ApiAnswersEnum::Answer4);

            return json!({
                    "status_code": answer.code.to_string(),
                    "status_answer": answer.message,
                    "vegetable_name": veg.vegetable_name,
                    "vegetable_family": veg.vegetable_family,
                    "vegetable_species": veg.vegetable_species
            }).into()
        },
        Err(_) => {
            let answer = answers::ApiAnswers::new(
                answers::ApiAnswersEnum::Answer5);

            return json!({
                    "status_code": answer.code.to_string(),
                    "status_answer": answer.message,
                    "vegetable_name": "",
                    "vegetable_family": "",
                    "vegetable_species": ""
            }).into()
        }
    }
}



// Check if vegetable is registerable by it's family name
pub async fn check_available_vegetable(Json(data): Json<VegetableFamilyJson>) -> Json<Value> {
    let family_name = data.family_name;
    let result = search_by_family(&family_name);

    if result.len() == 0 {
        let answer = answers::ApiAnswers::new(
            answers::ApiAnswersEnum::Answer6);
        return json!({
                "status_code": answer.code.to_string(),
                "status_answer": answer.message,
                "vegetable_list": ""
        }).into()

    } else {
        let answer = answers::ApiAnswers::new(
            answers::ApiAnswersEnum::Answer7);

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
                "status_code": answer.code.to_string(),
                "status_answer": answer.message,
                "vegetable_list": veg_list
        }).into()
    }
}