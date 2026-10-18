pub mod assets;
pub mod history;
pub mod live;
pub mod run;
pub mod views;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/healthz", axum::routing::get(|| async { "ok" }))
        .merge(live::router())
        .merge(history::router())
        .merge(assets::router())
}
