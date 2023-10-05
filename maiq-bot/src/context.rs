use std::ops::Deref;

use crate::commands::*;
use crate::format::*;
use crate::Result;
use crate::SnapshotParser;

use teloxide::payloads::SendMessage;
use teloxide::prelude::*;
use teloxide::requests::JsonRequest;

#[derive(Clone)]
pub struct Handler {
  bot: Bot,
  message: Message,
  parser: SnapshotParser,
}

impl Handler {
  pub fn new(bot: Bot, message: Message, parser: SnapshotParser) -> Self {
    Self { bot, message, parser }
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

  async fn start(&self, arg1: String, arg2: i32) -> Result<()> {
    self
      .send_message(self.message.chat.id, format!("arg: {arg1}; arg2: {arg2}"))
      .await?;
    Ok(())
  }

  async fn about(&self) -> Result<()> {
    todo!()
  }

  async fn show_config(&self) -> Result<()> {
    todo!()
  }

  async fn today(&self) -> Result<()> {
    if let Some(snapshot) = self.parser.read().await.latest_today() {
      self
        .reply(FormatSnapshot::select_group(snapshot, "Ир3-21").unwrap().to_string())
        .await?;
    } else {
      self.reply("Нет расписания").await?;
    }
    Ok(())
  }

  async fn next(&self) -> Result<()> {
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
}

impl DeveloperCommands for Handler {
  fn executor(&self) -> String {
    self.executor()
  }

  async fn test(&self) -> Result<()> {
    todo!()
  }
}
