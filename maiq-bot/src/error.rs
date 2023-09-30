use teloxide::RequestError;

#[derive(Debug)]
pub enum Error {
  Request(RequestError),
  Parser(maiq_parser_next::error::Error),
}

impl From<RequestError> for Error {
  fn from(value: RequestError) -> Self {
    Self::Request(value)
  }
}

impl From<maiq_parser_next::error::Error> for Error {
  fn from(value: maiq_parser_next::error::Error) -> Self {
    Self::Parser(value)
  }
}
