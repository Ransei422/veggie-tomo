use axum::{
    extract::State,
    http::StatusCode,
    response::Html, Form,
};
use validator::Validate;

use std::sync::Arc;

use sqlx::query;
use askama::Template;
use serde::{Deserialize, Serialize};

use crate::state::AppState;
use crate::jwt::jwt::hash_password;


#[allow(dead_code)]
#[derive(Template)]
#[template(path="partials/register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
    number_of_users: usize,
    messages: Vec<String>,
    color: String
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterData {
    #[validate (email(message="無効なEメールアドレス"))]
    pub email: String,

    #[validate (length(min=8, message="パスワードが7文字以上が必要"))]
    pub password: String,

    #[validate (must_match(other=password, message="パスワードが一致していない"))]
    pub confirm_password: String,
}


pub async fn register_page(State(app_state): State<Arc<AppState>>,) -> impl axum::response::IntoResponse {
    let pool = &app_state.db_pool;
    
    let rows = query!("SELECT * FROM users")
        .fetch_all(pool)
        .await
        .unwrap_or_else(|_| vec![]);


    let page_template: RegisterTemplate<'_> = RegisterTemplate {
        title: "Register Page",
        number_of_users: rows.len(),
        messages: Vec::new(),
        color: String::from("text-red-500")
    };

    (StatusCode::OK, Html(page_template.render().unwrap()))
}

pub async fn register_form(State(app_state): State<Arc<AppState>>,Form(data): Form<RegisterData>) -> impl axum::response::IntoResponse {
    
    let pool = &app_state.db_pool;
    let rows = query!("SELECT * FROM users")
        .fetch_all(pool)
        .await
        .unwrap_or_else(|_| vec![]);


    if let Err(errors) = data.validate() {
        let errors_data: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_field, errors)| {
                errors.iter().map(move |err| {
                    format!("・{}", err.to_string())
                })
            })
            .collect();

        let page_template: RegisterTemplate<'_> = RegisterTemplate {
            title: "Register Page",
            number_of_users: rows.len(),
            messages: errors_data,
            color: String::from("text-red-500")
        };
    
        (StatusCode::OK, Html(page_template.render().unwrap()))

    } else {
        let mut msgs = Vec::new();
        let password_hash = hash_password(&data.password).unwrap();
        let result = sqlx::query!(
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) ON CONFLICT (email) DO NOTHING;",
            data.email,
            password_hash
        )
        .execute(pool)
        .await;
    

        if result.expect("EXISTING USER").rows_affected() == 0 {
            let message = String::from("・すでに登録済みのメールアドレスが登録できない");
            msgs.push(message);


            let page_template: RegisterTemplate<'_> = RegisterTemplate {
                title: "Register Page",
                number_of_users: rows.len(),
                messages: msgs,
                color: String::from("text-red-500")
            };
            (StatusCode::OK, Html(page_template.render().unwrap()))
        } else {
            let success_message = String::from("新規管理ユーザーの登録を完了した");
            msgs.push(success_message);

            let page_template: RegisterTemplate<'_> = RegisterTemplate {
                title: "Register Page",
                number_of_users: rows.len(),
                messages: msgs,
                color: String::from("text-green-500")
            };
            (StatusCode::OK, Html(page_template.render().unwrap()))
        }
    }
}