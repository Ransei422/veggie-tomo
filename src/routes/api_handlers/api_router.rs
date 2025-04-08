use axum::{
    middleware,
    routing::post,
    Router
};
use std::sync::Arc;


use crate::{state::AppState,
    jwt::jwt};

use super::sockets::*;


pub fn api_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/register_vegetable", post(register_vegetables))
        .layer(middleware::from_fn_with_state(app_state.clone() ,jwt::authorize))
}
