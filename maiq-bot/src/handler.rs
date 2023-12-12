use std::ops::Deref;
use std::sync::Arc;

use maiq_db::models::*;
use maiq_parser_next::prelude::*;
use teloxide::payloads::EditMessageText;
use teloxide::prelude::*;

use maiq_db::Pool;
use teloxide::payloads::SendMessage;
use teloxide::requests::JsonRequest;
use teloxide::types::InlineKeyboardMarkup;

use anyhow::Result;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

use crate::callbacks::Callback;
use crate::format::*;
use crate::reply;
use crate::Caller;
use crate::SnapshotParser;

#[derive(Clone)]
pub struct Handler {
  pub bot: Bot,
  pub user: Arc<Mutex<User>>,
  pub message: Arc<Message>,
  pub parser: SnapshotParser,
  pub pool: Arc<Pool>,
  caller: Option<teloxide::types::User>,
  callback_id: Option<String>,
}

impl Handler {
  pub async fn with_message(bot: Bot, message: Message, parser: SnapshotParser, pool: Arc<Pool>) -> Option<Arc<Mutex<Self>>> {
    let caller = message.from().cloned();
    match User::get_by_id_or_create(message.chat.id.0, &pool).await {
      Ok(user) => {
        let handler =
          Self { bot, message: Arc::new(message), parser, user: Arc::new(Mutex::new(user)), pool, caller, callback_id: None };
        let name = if handler.message.chat.is_private() {
          handler.caller_name()
        } else {
          handler.message.chat.title().unwrap_or("(none)").into()
        };

        {
          let mut user = handler.user.lock().await;
          user.update_username(name, &handler.pool).await.ok()
        }
        .map(move |_| Arc::new(Mutex::new(handler)))
      }
      Err(err) => {
        error!(target: "commands", "can't query user model id {}; error: {:?}", message.chat.id.0, err);
        None
      }
    }
  }

  pub async fn with_callback(
    bot: Bot,
    query: CallbackQuery,
    parser: SnapshotParser,
    pool: Arc<Pool>,
  ) -> Option<Arc<Mutex<Self>>> {
    let message = match query.message {
      Some(msg) => msg,
      None => {
        error!(target: "callback", "no message in query");
        return None;
      }
    };

    match User::get_by_id_or_create(message.chat.id.0, &pool).await {
      Ok(user) => {
        let handler = Self {
          bot,
          message: Arc::new(message),
          parser,
          user: Arc::new(Mutex::new(user)),
          caller: Some(query.from),
          pool,
          callback_id: Some(query.id),
        };
        let name = if handler.message.chat.is_private() {
          handler.caller_name()
        } else {
          handler.message.chat.title().unwrap_or("(none)").into()
        };
        {
          let mut user = handler.user.lock().await;
          user.update_username(name, &handler.pool).await.ok()
        }
        .map(move |_| Arc::from(Mutex::new(handler)))
      }
      Err(err) => {
        error!(target: "commands", "can't query user model id {}; error: {:?}", query.from.id.0 as i64, err);
        None
      }
    }
  }

  pub async fn reply_snapshot(&self, snapshot: &Snapshot) -> Result<()> {
    let user = self.user().await;
    match user.config().groups().len() {
      0 => {
        self.reply(reply!("err/group_not_set.md")).await?;
      }
      1 => {
        let group_name = user.config().groups().first().unwrap();
        let format = FormatSnapshot::select_group(snapshot, group_name)
          .map(|s| s.to_string())
          .unwrap_or_else(|| reply!("err/no_timetable_exact.md", group_name = group_name));
        self.reply(format).await?;
      }
      _ => {
        let groups = user
          .config()
          .groups()
          .iter()
          .filter_map(|group| FormatSnapshot::select_group(snapshot, group));

        if groups.clone().count() == 0 {
          self.reply(reply!(const "err/no_timetable_many.md")).await?;
          return Ok(());
        }

        for format in groups {
          self
            .reply(reply!("snapshot/many_groups.md", group_name = format.group_name(), formatted = format.to_string()))
            .await?;
        }
      }
    }
    Ok(())
  }

  pub async fn answer(&self) -> Result<()> {
    if let Some(ref callback_id) = self.callback_id {
      self.answer_callback_query(callback_id).await?;
    }
    Ok(())
  }

  pub fn edit<S: Into<String>>(&self, message: S) -> JsonRequest<EditMessageText> {
    self
      .edit_message_text(self.message.chat.id, self.message.id, message)
      .disable_web_page_preview(true)
      .parse_mode(teloxide::types::ParseMode::Html)
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

  pub async fn config_markup(&self) -> InlineKeyboardMarkup {
    let toggle_text = if self.user().await.config().is_notifies_enabled() {
      "Выключить уведомления"
    } else {
      "Включить уведомления"
    };

    crate::markup!([
      [Callback::GetStartLink.with_text("Получить стартовую ссылку").into()],
      [Callback::ToggleNotifications.with_text(toggle_text).into()],
      [Callback::SetMyGroups.with_text("Настроить группы").into()],
      [Callback::Close.with_text("Закрыть").into()]
    ])
  }

  pub async fn user(&self) -> MutexGuard<User> {
    self.user.lock().await
  }
}

impl Deref for Handler {
  type Target = Bot;

  fn deref(&self) -> &Self::Target {
    &self.bot
  }
}

impl Caller for Handler {
  fn caller(&self) -> Option<&teloxide::types::User> {
    self.caller.as_ref()
  }

  fn caller_name(&self) -> String {
    if let Some(user) = self.caller() {
      format!("{} @{} ({})", user.full_name(), user.username.as_deref().unwrap_or("(none)"), user.id.0)
    } else {
      let chat = &self.message.chat;
      format!("unknown user; chat {} ({})", chat.title().unwrap_or("(none title)"), chat.id.0)
    }
  }
}
