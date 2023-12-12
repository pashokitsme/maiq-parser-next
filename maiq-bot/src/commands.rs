use crate::callbacks::Callback;
use crate::format::random_greeting;
use crate::handler::Handler;
use crate::make_commands;
use crate::reply;

use anyhow::Result;

use maiq_db::models::User;
use maiq_parser_next::parser::GROUP_NAMES;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;

make_commands! {
  pub: {
    Start[desc: "Стартовая команда", args: (group_indexes: String)] => start,
    Today[desc: "Сегодня"] => today,
    Next[desc: "Завтра"] => next,
    About[desc: "Информация"] => about,
    Config[desc: "Настройки"] => show_config,
    Version[desc: "Версия"] => version
  },
  dev: {
    UserList => userlist,
    TestErr => test_err
  }
}

impl Commands for Handler {
  async fn start(&self, group_indexes: String) -> Result<()> {
    let username = self.message.from().map(|u| u.full_name()).unwrap_or_default();
    self
      .reply(reply!("start.md", greeting = random_greeting(), username = username))
      .await?;

    let groups = group_indexes
      .split('g')
      .map(|s| s.parse().map(|idx: usize| GROUP_NAMES.get(idx)))
      .filter_map(|s| s.ok().flatten().map(|s| s.to_string()))
      .collect::<Vec<String>>();

    if !groups.is_empty() {
      let mut user = self.user().await;
      for group in &groups {
        user.config_mut().add_group(&group, &self.pool).await?;
      }

      self
        .reply(reply!("start_add_group.md", groups = groups.join(", ")))
        .await?;
    }

    self.about().await?;
    Ok(())
  }

  async fn about(&self) -> Result<()> {
    self.reply(reply!(const "about.md")).await?;
    Ok(())
  }

  async fn show_config(&self) -> Result<()> {
    self
      .reply(reply!(const "config.md"))
      .reply_markup(self.config_markup().await)
      .await?;

    self.delete_message(self.message.chat.id, self.message.id).await?;
    Ok(())
  }

  async fn today(&self) -> Result<()> {
    if let Some(snapshot) = self.parser.read().await.latest_today() {
      self.reply_snapshot(snapshot).await?;
    } else {
      self.reply(reply!(const "err/no_timetable.md")).await?;
    }
    Ok(())
  }

  async fn next(&self) -> Result<()> {
    if let Some(snapshot) = self.parser.read().await.latest_next() {
      self.reply_snapshot(snapshot).await?;
    } else {
      self.reply(reply!(const "err/no_timetable.md")).await?;
    }

    Ok(())
  }

  async fn version(&self) -> Result<()> {
    self.reply(crate::build_info::build_info()).await?;
    Ok(())
  }

  async fn finalize(&self, result: Result<()>) -> Result<()> {
    self.default_finalize(result).await
  }
}

impl DeveloperCommands for Handler {
  async fn userlist(&self) -> Result<()> {
    let users = User::get_all(&self.pool).await?;
    let mut reply = format!("Всего пользователей: <code>{}</code>\n\n", users.len());
    users
      .into_iter()
      .map(|user| {
        let enabled = if user.config().is_notifies_enabled() { "[+]" } else { "[-]" };
        format!(
          "{enabled} {username} (chat {chat_id}) <a href=\"tg://user?id={chat_id}\">link</a> \n created {created}; modified {modified}\n{groups}\n\n",
          enabled = enabled,
          chat_id = user.chat_id(),
          username = user.cached_fullname().unwrap_or_default(),
          created = user.created_at(),
          modified = user.modified_at(),
          groups = user.config().groups().join(", ")
        )
      })
      .for_each(|u| reply.push_str(&u));

    self
      .reply(reply)
      .reply_markup(Callback::Close.with_text("Закрыть"))
      .await?;
    Ok(())
  }

  async fn test_err(&self) -> Result<()> {
    Err(anyhow::anyhow!("Test error"))
  }

  async fn finalize(&self, result: Result<()>) -> Result<()> {
    self.default_finalize(result).await
  }
}
