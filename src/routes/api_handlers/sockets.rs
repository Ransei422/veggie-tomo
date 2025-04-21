// == File for primary called APIs ==

use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use std::sync::Arc;
use serde_json::{json, Value};
use sqlx::FromRow;

use crate::state::AppState;
use crate::answers;
use super::relations::{search_by_family, search_by_name};
use super::utils;



#[derive(Deserialize)]
pub struct VegetableNameJsonRequest {
    name: String,
}


#[derive(Deserialize)]
pub struct VegetableFamilyJsonRequest {
    family_name: String,
}


#[derive(Deserialize)]
pub struct VegetableRelationJson {
    vegetable_1_name: String,
    vegetable_2_name: String,
    compatibility: usize,
    explanation: String,
}


#[derive(FromRow, Serialize)]
pub struct VegetableData {
    pub id: i32,
    pub vegetable_name: String,
    pub vegetable_family: String,
    pub vegetable_species: String,
}


#[derive(Serialize)]
pub struct AvailableData<'a> {
    pub id: usize,
    pub vegetable_name: &'a str,
    pub vegetable_family: &'a str,
    pub vegetable_species: &'a str,
}


#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub status_code: String,
    pub status_answer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}



// Register data from relations.rs to DB based on user's input
pub async fn register_vegetables(app_state: State<Arc<AppState>>,
    Json(data): Json<VegetableNameJsonRequest>)
    ->  Json<ApiResponse<Value>> {
    if let Some(veg) = search_by_name(&data.name) {
        let result = utils::register_vegetable_to_db(&app_state, veg)
        .await;

        match result {
            Ok(_) => {
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer1);

                let response = ApiResponse {
                    status_code: answer.code.to_string(),
                    status_answer: answer.message,
                    data: None,
                
                };

                info!(
                    code = "INF",
                    endpoint = "register_vegetables",
                    status = response.status_code,
                    answer = response.status_answer,
                    server_message = "OK"
                );
                return Json(response);
            },

            Err(_err) => {
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer2);

                let response = ApiResponse {
                    status_code: answer.code.to_string(),
                    status_answer: answer.message,
                    data: None,
                };

                error!(
                    code = "ERR-1",
                    endpoint = "register_vegetables",
                    status = response.status_code,
                    answer = response.status_answer,
                    server_message = "Could not register vegetable into DB"
                );
                return Json(response);
            }
        }

    } else {
        let answer = answers::ApiAnswers::new(
            answers::ApiAnswersEnum::Answer3);

        let response = ApiResponse {
            status_code: answer.code.to_string(),
            status_answer: answer.message,
            data: None,
        };

        error!(
            code = "ERR-2",
            endpoint = "register_vegetables",
            status = response.status_code,
            answer = response.status_answer,
            server_message = "Could not find vegetable by its name"
        );
        return Json(response);
    }
}



// Check if vegetable already exists in DB 
pub async fn check_registered_vegetable(app_state: State<Arc<AppState>>,
    Json(data): Json<VegetableNameJsonRequest>) 
    -> Json<ApiResponse<Value>> {
    let result = utils::search_registered(&app_state, &data.name).await;

    match result {
        Ok(veg) => { 
            
            let answer = answers::ApiAnswers::new(
                answers::ApiAnswersEnum::Answer4);

            let response = ApiResponse {
                status_code: answer.code.to_string(),
                status_answer: answer.message,
                data: Some(json!({
                    "vegetable_name": veg.vegetable_name,
                    "vegetable_family": veg.vegetable_family,
                    "vegetable_species": veg.vegetable_species,
                })),
            };

            info!(
                code = "INF",
                endpoint = "check_registered_vegetable",
                status = response.status_code,
                answer = response.status_answer,
                server_message = "OK"
            );
            return Json(response);
        },

        Err(_) => {
            let answer = answers::ApiAnswers::new(
                answers::ApiAnswersEnum::Answer5);

            let response = ApiResponse {
                status_code: answer.code.to_string(),
                status_answer: answer.message,
                data: Some(json!({
                    "vegetable_name": "",
                    "vegetable_family": "",
                    "vegetable_species": "",
                })),
            };

            error!(
                code = "ERR",
                endpoint = "check_registered_vegetable",
                status = response.status_code,
                answer = response.status_answer,
                server_message = "Could not find vegetable by its name"
            );
            return Json(response);
        }
    }
}



// Check if vegetable is registerable by it's family name
pub async fn check_available_vegetable(Json(data): Json<VegetableFamilyJsonRequest>)
    -> Json<ApiResponse<Value>> {
    let family_name = data.family_name;
    let result = search_by_family(&family_name);

    if result.len() == 0 {
        let answer = answers::ApiAnswers::new(
            answers::ApiAnswersEnum::Answer6);

        let response = ApiResponse {
            status_code: answer.code.to_string(),
            status_answer: answer.message,
            data: Some(json!({
                "vegetable_list": "",
            })),
        };

        error!(
            code = "ERR",
            endpoint = "check_available_vegetable",
            status = response.status_code,
            answer = response.status_answer,
            server_message = "Could not find vegetable by its family-name"
        );
        return Json(response);

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

        let response = ApiResponse {
            status_code: answer.code.to_string(),
            status_answer: answer.message,
            data: Some(json!({
                "vegetable_list": veg_list,
            })),
        };

        info!(
            code = "INF",
            endpoint = "check_available_vegetable",
            status = response.status_code,
            answer = response.status_answer,
            server_message = "OK"
        );
        return Json(response);

    }
}



// Register relations between vegetables
pub async fn register_relationship(app_state: State<Arc<AppState>>,
    Json(data): Json<VegetableRelationJson>) 
    -> Json<ApiResponse<Value>> {

        let vegetable1 = data.vegetable_1_name;
        let vegetable2 = data.vegetable_2_name;

        let vegetable_1_id = utils::search_registered(
            &app_state, &vegetable1).await;

        let veg_1 = match vegetable_1_id {
            Ok(veg) => veg,
            Err(_) => {
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer5);

                let response = ApiResponse {
                    status_code: answer.code.to_string(),
                    status_answer: answer.message,
                    data: Some(json!({
                        "data_affected": 0
                    })),
                };
        
                error!(
                    code = "ERR-1",
                    endpoint = "register_relationship",
                    status = response.status_code,
                    answer = response.status_answer,
                    server_message = "Could not find relation-registerable vegetable1 in DB"
                );
                return Json(response);
            }
        };
        

        let vegetable_2_id = utils::search_registered(
            &app_state, &vegetable2).await;

        let veg_2 = match vegetable_2_id {
            Ok(veg) => veg,
            Err(_) => {
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer5);
                
                let response = ApiResponse {
                    status_code: answer.code.to_string(),
                    status_answer: answer.message,
                    data: Some(json!({
                        "data_affected": 0
                    })),
                };
        
                error!(
                    code = "ERR-2",
                    endpoint = "register_relationship",
                    status = response.status_code,
                    answer = response.status_answer,
                    server_message = "Could not find relation-registerable vegetable2 in DB"
                );
                return Json(response);
            }
        };


        let registration_result = utils::register_relationship_to_db(&app_state,
            veg_1.id, veg_2.id, data.compatibility, data.explanation).await;


        match registration_result {
            Ok(e) => {
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer1);

                let response = ApiResponse {
                    status_code: answer.code.to_string(),
                    status_answer: answer.message,
                    data: Some(json!({
                        "data_affected": e.rows_affected()
                    })),
                };
        
                info!(
                    code = "INF",
                    endpoint = "register_relationship",
                    status = response.status_code,
                    answer = response.status_answer,
                    server_message = "OK"
                );
                return Json(response);
            },

            Err(_) => {
                let answer = answers::ApiAnswers::new(
                    answers::ApiAnswersEnum::Answer6);

                let response = ApiResponse {
                    status_code: answer.code.to_string(),
                    status_answer: answer.message,
                    data: Some(json!({
                        "data_affected": 0
                    })),
                };
        
                error!(
                    code = "ERR-3",
                    endpoint = "register_relationship",
                    status = response.status_code,
                    answer = response.status_answer,
                    server_message = "Could not register relationship"
                );
                return Json(response);
            }
        }
}