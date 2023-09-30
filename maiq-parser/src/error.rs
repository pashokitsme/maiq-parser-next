use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Error {
  Network(String),
  NoHtmlTable,
}

impl From<reqwest::Error> for Error {
  fn from(value: reqwest::Error) -> Self {
    Error::Network(value.without_url().to_string())
  }
}
