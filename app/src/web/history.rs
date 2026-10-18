use axum::{Router, routing::get};
use maud::{Markup, html};

use super::views::layout;

pub fn router() -> Router {
    Router::new().route("/history", get(page))
}

async fn page() -> Markup {
    layout(
        "History",
        html! {
            h1 { "History" }
            p { "no runs yet" }
        },
    )
}
