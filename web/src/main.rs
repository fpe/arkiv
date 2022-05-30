#![feature(let_chains)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use anyhow::Context as AnyhowContext;
use axum::{extract::Extension, routing::get, Router, Server};
use http::StatusCode;
use std::net::SocketAddr;
use tera::Tera;
use tracing::{error, info};

use crate::{handler::{get_index, get_board, get_thread, cdn}, util::html_decode};

#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate serde;

mod error;
mod handler;
mod util;

const THREADS_PER_PAGE: i32 = 40;

#[derive(Debug, Serialize)]
struct BoardListing {
    board: String,
    thread_count: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    before: Option<i64>,
    after: Option<i64>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();
    tracing_subscriber::fmt::init();
    let mut t = Tera::new("web/templates/**/*.html").context("failed to compile templates")?;
    t.register_filter("html_decode", html_decode);
    info!("templates done");

    let data_dir = std::env::var_os("DATA_DIR").context("missing data dir var")?;
    let storage = arkiv_storage::local::LocalStorage::new(&data_dir);

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").context("missing db env var")?)
        .await
        .context("failed to connect to database")?;

    let app = Router::new()
        .route("/", get(get_index))
        .route("/:board", get(get_board))
        .route(
            "/:board/thread/:thread_id",
            get(get_thread),
        )
        .route(
            "/cdn/:board/:key",
            get(cdn::<arkiv_storage::local::LocalStorage>),
        )
        .nest(
            "/static",
            axum::routing::get_service(tower_http::services::ServeDir::new("./static/"))
                .handle_error(|err| async move {
                    error!("error occured: {}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                }),
        )
        .layer(Extension(t.clone()))
        .layer(Extension(storage.clone()))
        .layer(Extension(pool.clone()))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = "0.0.0.0:8080"
        .parse::<SocketAddr>()
        .context("failed to parse address")?;

    info!("listening on {:?}", &addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
