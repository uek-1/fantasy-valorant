use crate::html_stringify;
use anyhow::{Context, Result};
use axum::response::Html;
use axum::routing::{get, post};
use axum::{middleware, Extension, Router};
use clap::Parser;
use leptos::{view, IntoView};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

pub fn leagues_router() -> Router {
    Router::new().route("/", get(leagues_home()))
}

fn leagues_home() -> Html<String> {
    let contents = view! {
        <h1> "My leagues" </h1>
        <p> "Leagues table here" </p>
    };

    html_stringify(contents)
}
