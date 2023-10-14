pub mod error;
pub mod models;
pub mod queries;
mod utils;

use error::Error;
use log::*;

pub type Pool = sqlx::SqlitePool;
pub type Result<T> = std::result::Result<T, Error>;

type PoolOptions = sqlx::sqlite::SqlitePoolOptions;
type Db = sqlx::Sqlite;

pub async fn pool() -> Result<Pool> {
  info!(target: "db", "init sqlite database");
  let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
  let pool = PoolOptions::new().max_connections(4).connect(&url).await?;
  Ok(pool)
}
