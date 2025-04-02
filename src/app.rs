use std::time::Duration;
use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::{ServeDir, ServeFile};

use crate::router::{setup_routes, setup_private_routes};
use crate::state::AppState;
use crate::errors;

type InitErrors = errors::InitializationError;

#[warn(dead_code)]
struct Enviroment {
    database_url: String,
    jwt_secret: String,
    open_host: String,
    closed_host: String,
}


impl Enviroment {
    fn new() -> Enviroment {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::DBURLError)).unwrap();
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::JWTKeyError)).unwrap();

        let open_url = String::from("0.0.0.0");
        let open_port = env::var("OPEN_PORT")
            .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::SitePortError)).unwrap();
        let open_host = open_url + ":" + &open_port;

        let closed_url = String::from("127.0.0.1");
        let closed_port = env::var("CLOSED_PORT")
            .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::SitePortError)).unwrap();

        let closed_host = closed_url + ":" + &closed_port;

        Enviroment {
            database_url,
            jwt_secret,
            open_host,
            closed_host
        }
    }
}


pub async fn run() {
    let env_vals = Enviroment::new();
    let jwt_secret = env_vals.jwt_secret.clone().to_string();
    let open_env_host = env_vals.open_host;
    let closed_env_host = env_vals.closed_host;

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&env_vals.database_url)
        .await
        .map_err(|_| InitErrors::new(errors::InitializationErrorEnum::DBConnectionError))
        .unwrap();

    let app_state: std::sync::Arc<AppState> = AppState::new(db_pool, jwt_secret);
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    
    let private_app = setup_private_routes(app_state.clone(), serve_dir.clone());
    let app = setup_routes(app_state.clone(), serve_dir);

    let private_listener = tokio::net::TcpListener::bind(closed_env_host)
        .await
        .unwrap();
    
    let listener = tokio::net::TcpListener::bind(open_env_host)
        .await
        .unwrap();

    // Localhost only for registration
    tokio::spawn(async {
        axum::serve(private_listener, private_app)
            .await
            .unwrap();
    });

    // Open for API
    axum::serve(listener, app)
        .await
        .unwrap();
}