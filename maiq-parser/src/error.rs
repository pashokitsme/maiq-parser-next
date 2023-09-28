use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ParserError {
  Network(String),
  NoHtmlTable,
}

impl From<reqwest::Error> for ParserError {
  fn from(value: reqwest::Error) -> Self {
    ParserError::Network(value.without_url().to_string())
  }
}
