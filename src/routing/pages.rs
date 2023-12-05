use crate::html_stringify;
use anyhow::{Context, Result};
use axum::response::Html;
use axum::routing::{get, post};
use axum::{middleware, Extension, Router};
use clap::Parser;
use leptos::{view, IntoView};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

mod leagues;
use leagues::leagues_router;

pub fn page_router() -> Router {
    Router::new()
        .route("/login", get(login()))
        .route("/activity", get(activity()))
        .route("/settings", get(settings()))
        .nest("/leagues", leagues_router())
}

fn login() -> Html<String> {
    let contents = view! {
        <h1> "Login" </h1>
        <p> "oAuth stuff here" </p>
    };

    html_stringify(contents)
}

fn activity() -> Html<String> {
    let contents = view! {
        <h1> "Activity" </h1>
        <p> "Feed here" </p>
    };

    html_stringify(contents)
}

fn settings() -> Html<String> {
    let contents = view! {
        <h1> "Settings" </h1>
        <p> "Settings here" </p>
    };

    html_stringify(contents)
}
