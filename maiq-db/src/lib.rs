pub mod error;
pub mod models;
pub mod queries;
mod utils;

use error::Error;
use log::*;
use sqlx::migrate::MigrateDatabase;
use sqlx::Connection;

pub type Pool = sqlx::SqlitePool;
pub type Result<T> = std::result::Result<T, Error>;

type PoolOptions = sqlx::sqlite::SqlitePoolOptions;
type Db = sqlx::Sqlite;
type DbConnection = sqlx::sqlite::SqliteConnection;

pub async fn pool() -> Result<Pool> {
  info!(target: "db", "init sqlite database");
  let url = std::env::var("SQLITE_PATH").expect("SQLITE_PATH");
  validate(&url).await?;
  let pool = PoolOptions::new().max_connections(4).connect(&url).await?;
  Ok(pool)
}

async fn validate(url: &str) -> Result<()> {
  if !Db::database_exists(url).await? {
    warn!(target: "db", "database not exists at {}; creating", url);
    Db::create_database(url).await?;
  }

  debug!(target: "db", "validating migrations");
  let mut conn = DbConnection::connect(url).await?;
  sqlx::migrate!().run(&mut conn).await?;

  Ok(())
}
