use thiserror::Error;  

#[derive(Error, Debug)]
pub enum Error {
  #[error("sqlx: {0}")]
  Sqlx(#[from] sqlx::Error),
  
  #[error("migrate: {0}")]
  Migrate(#[from] sqlx::migrate::MigrateError),
}