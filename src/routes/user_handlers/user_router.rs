use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::state::AppState;

use super::users::*;


pub fn user_routes()  -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", get(register_page))
        .route("/register", post(register_form))
}