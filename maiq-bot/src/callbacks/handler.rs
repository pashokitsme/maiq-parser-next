use crate::handler::Handler;
use crate::Result;

use super::Callbacks;

impl Callbacks for Handler {
  fn data(&self) -> Option<&str> {
    self.data()
  }

  async fn test(self, arg: i32) -> Result<()> {
    self.reply(format!("Тык! {}", arg)).await?;
    self.answer().await?;
    Ok(())
  }
}
