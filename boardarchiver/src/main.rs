#![feature(let_chains)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use anyhow::Context;
use archiver::Archiver;
use config::Config;
use storage::local::LocalStorage;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate async_trait;

pub mod archiver;
pub mod config;
pub mod storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();
    tracing_subscriber::fmt::init();

    let config_path = std::env::var_os("CONFIG_PATH").unwrap_or_else(|| "config.yml".into());
    let config = Config::load(&config_path).await?;

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").context("missing db env var")?)
        .await
        .context("failed to connect to database")?;

    let data_dir = std::env::var_os("DATA_DIR").context("missing data dir var")?;
    let storage = LocalStorage::new(&data_dir);

    Archiver::new(pool, storage, config).run().await
}
