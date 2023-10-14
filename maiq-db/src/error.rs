#[derive(Debug)]
pub enum Error {
  Sqlx(sqlx::Error),
  Migrate(sqlx::migrate::MigrateError),
}

impl From<sqlx::Error> for Error {
  fn from(value: sqlx::Error) -> Self {
    Self::Sqlx(value)
  }
}

impl From<sqlx::migrate::MigrateError> for Error {
  fn from(value: sqlx::migrate::MigrateError) -> Self {
    Self::Migrate(value)
  }
}
