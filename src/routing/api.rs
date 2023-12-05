use crate::html_stringify;
use axum::routing::{get, post, Router};
use leptos::view;

fn api_router() -> Router {
    Router::new().nest("/api", Router::new())
}
