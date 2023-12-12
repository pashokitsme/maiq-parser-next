use maiq_parser_next::parser::GROUP_NAMES;
use teloxide::payloads::EditMessageTextSetters;
use teloxide::requests::Requester;
use teloxide::types::CallbackQuery;
use teloxide::types::InlineKeyboardButton;
use teloxide::types::InlineKeyboardMarkup;
use teloxide::types::ReplyMarkup;

use anyhow::Result;

use crate::handler::Handler;
use crate::make_callbacks;
use crate::markup;
use crate::reply;

make_callbacks! {
  Test(arg: i32) => test,
  SetMyGroups => show_my_groups,
  ShowConfig => show_config,
  SetGroup(name: String) => set_group,
  GetStartLink => get_start_link,
  ToggleNotifications => toggle_notifications,
  Close => close
}

impl Handler {
  async fn get_my_groups(&self) -> InlineKeyboardMarkup {
    let user = self.user().await;
    let into_button = |group: &&str| {
      let name = if user.config().has_group(group) { format!("✅ {}", group) } else { group.to_string() };
      Callback::SetGroup { name: group.to_string() }.with_text(name).into()
    };

    let buttons = GROUP_NAMES
      .iter()
      .step_by(3)
      .zip(GROUP_NAMES.iter().skip(1).step_by(3).map(into_button))
      .zip(GROUP_NAMES.iter().skip(2).step_by(3).map(into_button))
      .map(|((group, btn1), btn2)| [into_button(group), btn1, btn2]);

    markup!(buttons)
  }
}

impl Callbacks for Handler {
  async fn test(&self, arg: i32) -> Result<()> {
    self.reply(format!("Тык! {}", arg)).await?;
    self.answer().await?;
    Ok(())
  }

  async fn show_my_groups(&self) -> Result<()> {
    let markup = self
      .get_my_groups()
      .await
      .append_row([Callback::ShowConfig.with_text("OK").into()]);
    self.edit(reply!(const "set_groups.md")).reply_markup(markup).await?;
    Ok(())
  }

  async fn get_start_link(&self) -> Result<()> {
    let me = self.get_me().await?;
    let mut link = String::new();
    for idx in self
      .user()
      .await
      .config()
      .groups()
      .iter()
      .filter_map(|group| GROUP_NAMES.iter().position(|g| g == group))
    {
      link.push('g');
      link.push_str(&idx.to_string());
    }

    let link = format!("https://t.me/{me}?start={link}", me = me.username.as_ref().unwrap(), link = link);
    self.answer().await?;
    self.reply(reply!("start_make_link.md", link = link)).await?;
    Ok(())
  }

  async fn toggle_notifications(&self) -> Result<()> {
    let mut user = self.user().await;
    let config = user.config_mut();
    config.set_is_notifies_enabled(!config.is_notifies_enabled());
    self.user().await.update(&self.pool).await?;
    self.answer().await?;
    self.show_config().await?;
    Ok(())
  }

  async fn set_group(&self, name: String) -> Result<()> {
    let mut user = self.user().await;
    match user.config().has_group(&name) {
      true => user.config_mut().remove_group(name, &self.pool).await?,
      false => user.config_mut().add_group(name, &self.pool).await?,
    }
    self.show_my_groups().await
  }

  async fn show_config(&self) -> Result<()> {
    self
      .edit(reply!(const "config.md"))
      .reply_markup(self.config_markup().await)
      .await?;
    Ok(())
  }

  async fn close(&self) -> Result<()> {
    self.delete_message(self.message.chat.id, self.message.id).await?;
    Ok(())
  }
}
