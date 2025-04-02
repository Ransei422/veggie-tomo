use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
};
use std::sync::Arc;

use sqlx::query;
use askama::Template;

use crate::state::AppState;


#[allow(dead_code)]
#[derive(Template)]
#[template(path="partials/register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
    number_of_users: usize,
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