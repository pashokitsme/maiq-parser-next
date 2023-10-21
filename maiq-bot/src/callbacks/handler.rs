use crate::Caller;
use crate::Result;

use super::Callbacks;

struct Handler;

impl Handler {
  pub async fn new() -> Self {
    todo!();
  }
}

impl Caller for Handler {
  fn caller(&self) -> String {
    todo!()
  }
}

impl Callbacks for Handler {
  async fn test(self) -> Result<()> {
    todo!()
  }
}
