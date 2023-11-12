use crate::callbacks::Callback;
use crate::commands::*;
use crate::handler::Handler;
use crate::markup;
use crate::reply;
use crate::Result;

use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;

impl Commands for Handler {
  async fn start(self) -> Result<()> {
    let username = self.message.from().map(|u| u.full_name()).unwrap_or_default();
    self.reply(reply!("start.md", username = username)).await?;
    Ok(())
  }

  async fn about(self) -> Result<()> {
    self.reply(reply!(const "about.md")).await?;
    Ok(())
  }

  async fn show_config(self) -> Result<()> {
    self
      .reply(reply!(const "config.md"))
      .reply_markup(markup!([
        [Callback::SetMyGroups.with_text("Настроить группы").into()],
        [Callback::Close.with_text("Закрыть").into()]
      ]
      .into_iter()))
      .await?;

    self.delete_message(self.message.chat.id, self.message.id).await?;
    Ok(())
  }

  async fn today(self) -> Result<()> {
    if let Some(snapshot) = self.parser.read().await.latest_today() {
      self.reply_snapshot(snapshot).await?;
    } else {
      self.reply(reply!(const "err/no_timetable.md")).await?;
    }
    Ok(())
  }

  async fn next(self) -> Result<()> {
    if let Some(snapshot) = self.parser.read().await.latest_next() {
      self.reply_snapshot(snapshot).await?;
    } else {
      self.reply(reply!(const "err/no_timetable.md")).await?;
    }

    Ok(())
  }

  async fn version(self) -> Result<()> {
    self.reply(crate::build_info::build_info()).await?;
    Ok(())
  }
}

impl DeveloperCommands for Handler {
  async fn test(self) -> Result<()> {
    todo!()
  }
}
