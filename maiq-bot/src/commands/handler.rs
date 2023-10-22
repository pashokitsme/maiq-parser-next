use crate::callbacks::Callback;
use crate::commands::*;
use crate::error::*;
use crate::handler::Handler;
use crate::reply;
use crate::Result;

use maiq_parser_next::prelude::*;
use teloxide::payloads::SendMessageSetters;
use teloxide::types::InlineKeyboardMarkup;

impl Commands for Handler {
  async fn start(self) -> Result<()> {
    let username = self.message.from().map(|u| u.full_name()).unwrap_or_default();
    self.reply(reply!("start.md", username = username)).await?;
    Ok(())
  }

  async fn about(self) -> Result<()> {
    self.reply(reply!("about.md")).await?;
    Ok(())
  }

  async fn show_config(self) -> Result<()> {
    self
      .reply(reply!("config.md"))
      .reply_markup(InlineKeyboardMarkup::new([[Callback::SetMyGroups.with_text("Настроить группы").into()]]))
      .await?;
    Ok(())
  }

  async fn today(self) -> Result<()> {
    if let Some(snapshot) = self.parser.read().await.latest_today() {
      self.reply_snapshot(snapshot).await?;
    } else {
      self.reply(reply!("err/no_timetable.md")).await?;
    }
    Ok(())
  }

  async fn next(self) -> Result<()> {
    if let Some(snapshot) = self.parser.read().await.latest_next() {
      self.reply_snapshot(snapshot).await?;
    } else {
      self.reply(reply!("err/no_timetable.md")).await?;
    }

    Ok(())
  }

  async fn add_group(mut self, name: String) -> Result<()> {
    if !GROUP_NAMES.contains(&name.as_str()) {
      return Err(CommandError::GroupNotExists(name).into());
    }

    self.user.config_mut().add_group(name, &self.pool).await?;
    Ok(())
  }

  async fn remove_group(mut self, name: String) -> Result<()> {
    self.user.config_mut().remove_group(name, &self.pool).await?;
    Ok(())
  }

  async fn show_my_groups(self) -> Result<()> {
    self
      .reply(format!("Твои группы: {:?}", self.user.config().groups().as_ref()))
      .await?;
    Ok(())
  }

  async fn version(self) -> Result<()> {
    self.reply(crate::build_info::build_info()).await?;
    Ok(())
  }

  async fn test_callback(self) -> Result<()> {
    let buttons = [[Callback::Test { arg: 15 }.with_text("Кнопка!").into()]];
    self
      .reply("Тест")
      .reply_markup(InlineKeyboardMarkup::new(buttons.into_iter()))
      .await?;
    Ok(())
  }
}

impl DeveloperCommands for Handler {
  async fn test(self) -> Result<()> {
    todo!()
  }
}
