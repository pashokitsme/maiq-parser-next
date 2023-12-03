use diesel::r2d2;

#[derive(Debug)]
pub enum Error {
  Diesel(diesel::ConnectionError),
  Pool(r2d2::PoolError),
  R2d2(r2d2::Error),
  Other(Box<dyn std::error::Error + Send + Sync>),
}

impl From<diesel::ConnectionError> for Error {
  fn from(value: diesel::ConnectionError) -> Self {
    Self::Diesel(value)
  }
}

impl From<r2d2::PoolError> for Error {
  fn from(value: r2d2::PoolError) -> Self {
    Self::Pool(value)
  }
}

impl From<r2d2::Error> for Error {
  fn from(value: r2d2::Error) -> Self {
    Self::R2d2(value)
  }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
  fn from(value: Box<dyn std::error::Error + Send + Sync>) -> Self {
    Self::Other(value)
  }
}
