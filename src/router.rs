// == Main file for routing ==

use std::sync::Arc;
use routes::root::root;
use tower_http::{services::{ServeDir, ServeFile}, set_status::SetStatus};

use crate::jwt::jwt;
use axum::{routing::{get, post},
        Router
};
use crate::{fallback::fallback,
    routes,
    state::AppState
};

use routes::api_handlers::api_router::api_routes;
use routes::user_handlers::user_router::user_routes;



pub fn setup_routes(app_state: Arc<AppState>, serve_dir: ServeDir<SetStatus<ServeFile>>) -> Router {
    let route = Router::new()
        .fallback(fallback)
        // Main Page
        .route("/", get(root))
        .route("/signin", post(jwt::sign_in_page))
        // APIs
        .merge(api_routes(app_state.clone()))
        .nest_service("/assets", serve_dir.clone());

    route.with_state(app_state)
}

pub fn setup_private_routes(app_state: Arc<AppState>, serve_dir: ServeDir<SetStatus<ServeFile>>) -> Router {
    let route = Router::new()
    .fallback(fallback)
    .merge(user_routes())
        .nest_service("/assets", serve_dir.clone());
    route.with_state(app_state)
}