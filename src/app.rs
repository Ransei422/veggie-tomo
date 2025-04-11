// == File for running the app ==

use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::{ServeDir, ServeFile};

use crate::router::{setup_routes, setup_private_routes};
use crate::state::AppState;
use crate::errors;
use crate::enviroment::*;


pub async fn run() {
    let env_vals = Enviroment::new();
    let jwt_secret = env_vals.jwt_secret.clone().to_string();
    let open_env_host = env_vals.open_host;
    let closed_env_host = env_vals.closed_host;
    let registration_allowed: bool = env_vals.registration_allowed == "true";

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&env_vals.database_url)
        .await
        .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::DBConnectionError))
        .unwrap();

    let app_state: std::sync::Arc<AppState> = AppState::new(db_pool, jwt_secret);
    let serve_dir = ServeDir::new("assets")
        .not_found_service(ServeFile::new("partials/fallback.html"));
    
    let private_app = setup_private_routes(app_state.clone(), serve_dir.clone());
    let public_app = setup_routes(app_state.clone(), serve_dir);

    let private_listener = tokio::net::TcpListener::bind(closed_env_host)
        .await
        .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::PortError1)).unwrap();
    
    let listener = tokio::net::TcpListener::bind(open_env_host)
        .await
        .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::PortError2)).unwrap();

    if registration_allowed {
        // Localhost only for registration
        tokio::spawn(async {
            axum::serve(private_listener, private_app)
                .await
                .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::ServerError1)).unwrap();
        });
    }

    // Open for API
    axum::serve(listener, public_app)
        .await
        .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::ServerError2)).unwrap();
}