use teloxide::RequestError;

#[derive(Debug)]
pub enum Error {
  Request(RequestError),
  Parser(maiq_parser_next::error::Error),
  Command(CommandError),
  Database(maiq_db::error::Error),
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

impl From<maiq_db::error::Error> for Error {
  fn from(value: maiq_db::error::Error) -> Self {
    Self::Database(value)
  }
}

#[derive(Debug)]
pub enum CommandError {
  NoUserProvided,
}

impl From<CommandError> for Error {
  fn from(value: CommandError) -> Self {
    Error::Command(value)
  }
}
