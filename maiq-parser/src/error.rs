use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("network: {0}")]
  Network(#[from] Box<ureq::Error>),

  #[error("no table in html document")]
  NoHtmlTable,
}

impl Error {
  pub fn can_be_skipped(&self) -> bool {
    matches!(self, Self::NoHtmlTable)
  }
}
