use anyhow::{Context, Result};
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::{middleware, Extension, Router};
use clap::Parser;
use leptos::{view, IntoView};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

mod routing;

#[derive(Parser)]
#[command()]
struct Cli {
    #[arg(short, long)]
    database_path: Option<String>,

    #[arg(long)]
    host: Option<String>,

    #[arg(short, long)]
    port: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let options = SqliteConnectOptions::new()
        .filename(cli.database_path.unwrap_or(String::from("default.db")))
        .create_if_missing(true);

    let connection = SqlitePool::connect_with(options)
        .await
        .context("Couldn't connect to database_path!")?;

    // Routes
    let app = Router::new()
        .route("/", get(home))
        .merge(routing::pages::page_router());

    // Start server
    let host = cli.host.unwrap_or(String::from("localhost"));
    let port = cli
        .port
        .clone()
        .unwrap_or(String::from("3000"))
        .parse::<u16>()
        .context(format!(
            "The port was invalid! Couldn't parse a u16 from {:?}",
            &cli.port
        ))?;

    let listener = tokio::net::TcpListener::bind((host.as_str(), port))
        .await
        .context("Failed to start tcp listener")?;

    println!("Starting listener on: http://{host}:{port}");
    axum::serve(listener, app)
        .await
        .context("Failed to serve app!")?;

    Ok(())
}

fn html_stringify(view: impl IntoView) -> Html<String> {
    let html_string = view.into_view().render_to_string().to_string();
    Html(html_string)
}

async fn home() -> impl IntoResponse {
    Redirect::to("/login").into_response()
}
