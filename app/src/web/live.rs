use axum::{Router, routing::get};
use maud::{Markup, html};

use super::views::layout;

// Routes of the Live page. `web::router()` merges them into the main router.
pub fn router() -> Router {
    Router::new()
        .route("/", get(page))
        .route("/fragments/nodes", get(nodes_fragment))
}

// Full page: layout + the fragment inside `#nodes`.
async fn page() -> Markup {
    layout(
        "Live",
        html! {
            h1 { "Live" }
            div #nodes { (nodes_fragment().await) }
        },
    )
}

// Fragment only (no <html>): what htmx will fetch and swap into `#nodes`.
async fn nodes_fragment() -> Markup {
    html! {
        p { "no data yet" }
    }
}
