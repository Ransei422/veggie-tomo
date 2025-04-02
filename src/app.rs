use std::time::Duration;
use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::{ServeDir, ServeFile};

use crate::router::setup_routes;
use crate::state::AppState;
use crate::errors;

type InitErrors = errors::InitializationError;

#[warn(dead_code)]
struct Enviroment {
    database_url: String,
    jwt_secret: String,
    host: String
}


impl Enviroment {
    fn new() -> Enviroment {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::DBURLError)).unwrap();
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::JWTKeyError)).unwrap();
        let url = env::var("HOST")
            .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::SiteURLError)).unwrap();
        let port = env::var("PORT")
            .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::SitePortError)).unwrap();

        let host = url + ":" + &port;

        Enviroment {
            database_url,
            jwt_secret,
            host,
        }
    }
}


pub async fn run() {
    let env_vals = Enviroment::new();
    let jwt_secret = env_vals.jwt_secret.clone().to_string();
    let env_host = env_vals.host;

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&env_vals.database_url)
        .await
        .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::DBConnectionError))
        .unwrap();

    let app_state: std::sync::Arc<AppState> = AppState::new(db_pool, jwt_secret);

    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    let app = setup_routes(app_state.clone(), serve_dir);
    // app.nest_service("/assets", serve_dir.clone());

    let listener = tokio::net::TcpListener::bind(env_host)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}