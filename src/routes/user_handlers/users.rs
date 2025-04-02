use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
};
use std::sync::Arc;
use sqlx::query;

use crate::state::AppState;

#[derive(Template)]
#[template(path="partials/register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
}

pub async fn register_page(State(app_state): State<Arc<AppState>>,) -> impl axum::response::IntoResponse {
    let page_template: RegisterTemplate<'_> = RegisterTemplate {
        title: "Register Page"
    };

    (StatusCode::OK, Html(page_template.render().unwrap()))
}