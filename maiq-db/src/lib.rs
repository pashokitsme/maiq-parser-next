pub mod error;
pub mod models;
pub mod queries;
pub mod schema;
mod utils;

use diesel::r2d2;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use log::*;

use diesel::migration::Migration;
use diesel::prelude::*;

pub type Pool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
pub type Result<T> = std::result::Result<T, crate::error::Error>;

const MIGRATIONS: EmbeddedMigrations = diesel_migrations::embed_migrations!();

pub fn pool(connection_url: &str) -> Result<Pool> {
  let manager = r2d2::ConnectionManager::new(connection_url);
  let pool = Pool::builder().test_on_check_out(false).max_size(3).build(manager)?;

  let mut conn = pool.get()?;
  let pending_migrations = conn.pending_migrations(MIGRATIONS)?;
  if !pending_migrations.is_empty() {
    info!(target: "db", "run pending migrations: {}", pending_migrations.iter().map(|m| m.name().to_string()).collect::<String>());
    conn.run_pending_migrations(MIGRATIONS)?;
  }
  Ok(pool)
}