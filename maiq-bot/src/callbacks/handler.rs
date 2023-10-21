use crate::handler::Handler;
use crate::Result;

use super::Callbacks;

impl Callbacks for Handler {
  fn data(&self) -> Option<&str> {
    self.data()
  }

  async fn test(self) -> Result<()> {
    self.reply("Тык!").await?;
    self.answer().await?;
    Ok(())
  }
}
