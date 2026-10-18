mod config;
mod db;
mod error;
mod k8s;
mod metrics;
mod state;
mod web;

#[tokio::main]
async fn main() {
    let app = web::router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
