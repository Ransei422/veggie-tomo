use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Redirect}, Form,
};

use std::sync::Arc;

use sqlx::query;
use askama::Template;
use serde::Deserialize;

use crate::state::AppState;
use crate::jwt::jwt::hash_password;


#[allow(dead_code)]
#[derive(Template)]
#[template(path="partials/register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
    number_of_users: usize,
}


#[derive(Debug, Deserialize)]
pub struct RegisterData {
    pub email: String,
    pub password: String,
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
        number_of_users: rows.len()
    };

    (StatusCode::OK, Html(page_template.render().unwrap()))
}

pub async fn register_form(Form(data): Form<RegisterData>) -> Redirect {
    println!("{:#?}", data);
    let x = hash_password(&data.password);
    println!("{:#?}", x);
    Redirect::to("/")

}