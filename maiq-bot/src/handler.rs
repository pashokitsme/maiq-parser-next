use std::ops::Deref;
use std::sync::Arc;

use maiq_db::models::*;
use maiq_parser_next::prelude::*;
use teloxide::prelude::*;

use maiq_db::Pool;
use teloxide::payloads::SendMessage;
use teloxide::requests::JsonRequest;

use crate::format::*;
use crate::reply;
use crate::Caller;
use crate::Result;
use crate::SnapshotParser;

#[derive(Clone)]
pub struct Handler {
  pub bot: Bot,
  pub user: User,
  pub message: Message,
  pub parser: SnapshotParser,
  pub pool: Arc<Pool>,
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

  pub async fn reply_snapshot(&self, snapshot: &Snapshot) -> Result<()> {
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

  pub fn reply<S: Into<String>>(&self, message: S) -> JsonRequest<SendMessage> {
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

impl Caller for Handler {
  fn caller(&self) -> String {
    if let Some(user) = self.message.from() {
      format!("{} @{} <{}>", user.full_name(), user.username.as_deref().unwrap_or("<none>"), user.id.0)
    } else {
      let chat = &self.message.chat;
      format!("unknown user; chat {} <{}>", chat.title().unwrap_or("<none title>"), chat.id.0)
    }
  }
}
