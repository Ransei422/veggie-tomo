mod app;
mod state;
mod jwt;
mod routes;
mod router;
mod answers;
mod fallback;
mod enviroment;


use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;



#[tokio::main]
async fn main() {
    let file_appender = rolling::daily("logs", "veggie-tomo.log");

    tracing_subscriber::fmt()
        .with_writer(file_appender)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("error"))  // fallback to "error"
        )
        .with_ansi(false) // disables escape codes
        .init();

    app::run().await;
}