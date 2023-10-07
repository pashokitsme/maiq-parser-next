#[derive(Debug)]
pub enum Error {
  Sqlx(sqlx::Error),
}

impl From<sqlx::Error> for Error {
  fn from(value: sqlx::Error) -> Self {
    Self::Sqlx(value)
  }
}
