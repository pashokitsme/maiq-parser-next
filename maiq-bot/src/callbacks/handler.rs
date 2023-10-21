use crate::handler::Handler;
use crate::Result;

use super::Callbacks;

impl Callbacks for Handler {
  async fn test(self, arg: i32) -> Result<()> {
    self.reply(format!("Тык! {}", arg)).await?;
    self.answer().await?;
    Ok(())
  }
}
