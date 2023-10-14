use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Error {
  Network(String),
  NoHtmlTable,
}

impl From<ureq::Error> for Error {
  fn from(value: ureq::Error) -> Self {
    Error::Network(value.to_string())
  }
}
