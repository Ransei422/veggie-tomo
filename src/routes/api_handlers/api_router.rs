// == File for API routing ==

use std::sync::Arc;
use axum::{
    middleware,
    routing::post,
    Router
};


use crate::{
    state::AppState,
    jwt
};


use super::sockets::*;



pub fn api_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        // Vegetable registration
        .route("/api/register_vegetable", post(register_vegetables))
        .layer(middleware::from_fn_with_state(app_state.clone() ,jwt::sockets::authorize))
        // Check registered vegetable
        .route("/api/check_registered_vegetable", post(check_registered_vegetable))
        .layer(middleware::from_fn_with_state(app_state.clone() ,jwt::sockets::authorize))
        // Check registerable vegetable
        .route("/api/check_available_vegetable", post(check_available_vegetable))
        .layer(middleware::from_fn_with_state(app_state.clone() ,jwt::sockets::authorize))
        // Register vegetable relationship
        .route("/api/register_vegetable_relationship", post(register_relationship))
        .layer(middleware::from_fn_with_state(app_state.clone() ,jwt::sockets::authorize))
}