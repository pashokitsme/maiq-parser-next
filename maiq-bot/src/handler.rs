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

use maiq_parser_next::prelude::*;

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

impl Handler {
  async fn reply_snapshot(&self, snapshot: &Snapshot) -> Result<()> {
    match self.user.config().groups().len() {
      0 => {
        self.reply(reply!("err/group_not_set.md")).await?;
      }
      1 => {
        let group_name = self.user.config().groups().next().unwrap();
        let format = FormatSnapshot::select_group(snapshot, group_name)
          .map(|s| s.to_string())
          .unwrap_or_else(|| reply!("err/no_timetable_exact.md", group_name = group_name));
        self.reply(format).await?;
      }
      _ => {
        for format in self
          .user
          .config()
          .groups()
          .filter_map(|group| FormatSnapshot::select_group(snapshot, group))
        {
          self
            .reply(reply!("snapshot/many_groups.md", group_name = format.group_name(), formatted = format.to_string()))
            .await?;
        }
      }
    }
    Ok(())
  }
}

impl Commands for Handler {
  fn executor(&self) -> String {
    self.executor()
  }

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
    self.reply(reply!("config.md")).await?;
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
}

impl DeveloperCommands for Handler {
  fn executor(&self) -> String {
    self.executor()
  }

  async fn test(self) -> Result<()> {
    todo!()
  }
}
