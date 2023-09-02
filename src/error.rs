pub enum ParserError {
  Network(reqwest::Error),
  Builder(BuilderError),
}

pub enum BuilderError {
  IntervalNotSet,
  UrlNotSet,
}

impl From<BuilderError> for ParserError {
  fn from(value: BuilderError) -> Self {
    ParserError::Builder(value)
  }
}

impl From<reqwest::Error> for ParserError {
  fn from(value: reqwest::Error) -> Self {
    ParserError::Network(value)
  }
}
