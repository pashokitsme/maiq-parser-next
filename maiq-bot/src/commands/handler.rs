use crate::callbacks::Callback;
use crate::commands::*;
use crate::handler::Handler;
use crate::markup;
use crate::reply;
use crate::Result;

use maiq_parser_next::parser::GROUP_NAMES;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;

impl Commands for Handler {
  async fn start(mut self, group_indexes: String) -> Result<()> {
    let username = self.message.from().map(|u| u.full_name()).unwrap_or_default();
    self.reply(reply!("start.md", username = username)).await?;

    let groups = group_indexes
      .split(',')
      .map(|s| s.parse().map(|idx: usize| GROUP_NAMES.get(idx)))
      .filter_map(|s| s.ok().flatten().map(|s| s.to_string()))
      .collect::<Vec<String>>();

    if !groups.is_empty() {
      for group in &groups {
        self.user.config_mut().add_group(&group, &self.pool).await?;
      }

      self
        .reply(reply!("start_add_group.md", groups = groups.join(", ")))
        .await?;
    }

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
