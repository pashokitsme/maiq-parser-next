use teloxide::prelude::*;
use teloxide::RequestError;

pub enum Error {
  Request(RequestError),
}
