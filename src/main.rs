mod app;
mod state;
mod jwt;
mod routes;
mod router;
mod errors;
mod fallback;

#[tokio::main]
async fn main() {
    app::run().await;
}