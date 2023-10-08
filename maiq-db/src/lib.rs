pub mod error;
pub mod models;
pub mod queries;
mod utils;

use error::Error;
use log::*;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use sqlx::Postgres;

pub type Pool = PgPool;
pub type Result<T> = std::result::Result<T, Error>;

type PoolOptions = PgPoolOptions;
type Db = Postgres;

pub async fn pool() -> Result<Pool> {
  info!(target: "db", "init pgpool");
  let url = std::env::var("DATABASE_URL").unwrap();
  let pool = PoolOptions::new().max_connections(4).connect(&url).await?;
  Ok(pool)
}
