use serde::Deserialize;
use serde::Serialize;

#[derive(Debug)]
pub enum ParserError {
  Network(reqwest::Error),
  Builder(BuilderError),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
