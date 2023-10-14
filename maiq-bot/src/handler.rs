use std::ops::Deref;
use std::sync::Arc;

use crate::commands::*;
use crate::error::*;
use crate::format::*;
use crate::reply;
use crate::Result;
use crate::SnapshotParser;

use maiq_db::models::*;
use maiq_db::Pool;

use maiq_parser_next::prelude::GROUP_NAMES;
use teloxide::payloads::SendMessage;
use teloxide::prelude::*;
use teloxide::requests::JsonRequest;

#[derive(Clone)]
pub struct Handler {
  bot: Bot,
  user: User,
  message: Message,
  parser: SnapshotParser,
  pool: Arc<Pool>,
}

impl Handler {
  pub async fn new(bot: Bot, message: Message, parser: SnapshotParser, pool: Arc<Pool>) -> Option<Self> {
    match User::get_by_id_or_create(message.chat.id.0, &pool).await {
      Ok(user) => Some(Self { bot, message, parser, user, pool }),
      Err(err) => {
        error!(target: "commands", "can't query user model id {}; error: {:?}", message.chat.id.0, err);
        None
      }
    }
  }

  fn executor(&self) -> String {
    if let Some(user) = self.message.from() {
      format!("{} @{} <{}>", user.full_name(), user.username.as_deref().unwrap_or("<none>"), user.id.0)
    } else {
      let chat = &self.message.chat;
      format!("unknown user; chat {} <{}>", chat.title().unwrap_or("<none title>"), chat.id.0)
    }
  }

  fn reply<S: Into<String>>(&self, message: S) -> JsonRequest<SendMessage> {
    let send = self
      .send_message(self.message.chat.id, message)
      .disable_web_page_preview(true)
      .parse_mode(teloxide::types::ParseMode::Html);
    if let Some(thread_id) = self.message.thread_id {
      send.message_thread_id(thread_id)
    } else {
      send
    }
  }
}

impl Deref for Handler {
  type Target = Bot;

  fn deref(&self) -> &Self::Target {
    &self.bot
  }
}

impl Commands for Handler {
  fn executor(&self) -> String {
    self.executor()
  }

  async fn start(self) -> Result<()> {
    let username = self.message.from().map(|u| u.full_name()).unwrap_or_default();
    self.reply(reply!("replies/start.md", username = username)).await?;
    Ok(())
  }

  async fn about(self) -> Result<()> {
    self.reply(reply!("replies/about.md")).await?;
    Ok(())
  }

  async fn show_config(self) -> Result<()> {
    self.reply(reply!("replies/config.md")).await?;
    Ok(())
  }

  async fn today(self) -> Result<()> {
    if let Some(snapshot) = self.parser.read().await.latest_today() {
      self
        .reply(FormatSnapshot::select_group(snapshot, "Ир3-21").unwrap().to_string())
        .await?;
    } else {
      self.reply("Нет расписания").await?;
    }
    Ok(())
  }

  async fn next(self) -> Result<()> {
    if let Some(snapshot) = self.parser.read().await.latest_next() {
      self
        .reply(
          FormatSnapshot::select_group(snapshot, "Ир3-21")
            .map(|s| s.to_string())
            .unwrap_or("Нет расписания".into()),
        )
        .await?;
    } else {
      self.reply("Нет расписания").await?;
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
}

impl DeveloperCommands for Handler {
  fn executor(&self) -> String {
    self.executor()
  }

  async fn test(self) -> Result<()> {
    todo!()
  }
}
