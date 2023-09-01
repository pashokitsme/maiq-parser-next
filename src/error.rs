pub enum ParserError {
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
