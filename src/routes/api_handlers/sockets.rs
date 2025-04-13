// == File for primary called APIs ==

use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use serde_json::{json, Value};
use sqlx::FromRow;

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


#[derive(Deserialize)]
pub struct VegetableRelationJson {
    vegetable_1_name: String,
    vegetable_2_name: String,
    compatibility: usize,
    explanation: String,
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
pub async fn register_vegetables(app_state: State<Arc<AppState>>,
                                Json(data): Json<VegetableNameJson>)
                                -> Json<Value> {
    if let Some(veg) = search_by_name(&data.name) {
        let result = utils::register_vegetable_to_db(&app_state, veg)
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



// Register relations between vegetables
pub async fn register_relationship(app_state: State<Arc<AppState>>,
    Json(data): Json<VegetableRelationJson>) 
    -> Json<Value> {

        let vegetable1 = data.vegetable_1_name;
        let vegetable2 = data.vegetable_2_name;

        

        let vegetable_1_id = utils::search_registered(
            &app_state, &vegetable1).await;

        let veg_1 = match vegetable_1_id {
            Ok(veg) => veg,
            Err(_) => {
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer5);
    
                return json!({
                        "status_code": answer.code.to_string(),
                        "status_answer": answer.message,
                }).into()
            }
        };
        

        let vegetable_2_id = utils::search_registered(
            &app_state, &vegetable2).await;

        let veg_2 = match vegetable_2_id {
            Ok(veg) => veg,
            Err(_) => {
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer5);
    
                return json!({
                        "status_code": answer.code.to_string(),
                        "status_answer": answer.message,
                }).into()
            }
        };


        let registration_result = utils::register_relationship_to_db(&app_state,
            veg_1.id, veg_2.id, data.compatibility, data.explanation).await;


        match registration_result {
            Ok(_) => {
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer1);
    
                return json!({
                        "status_code": answer.code.to_string(),
                        "status_answer": answer.message,
                }).into()
            },

            Err(_) => {
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer6);

                return json!({
                    "status_code": answer.code.to_string(),
                    "status_answer": answer.message,
            }).into()
            }
        }
}