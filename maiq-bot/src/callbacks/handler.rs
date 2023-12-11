use maiq_parser_next::parser::GROUP_NAMES;
use teloxide::payloads::EditMessageTextSetters;
use teloxide::requests::Requester;
use teloxide::types::InlineKeyboardMarkup;

use crate::callbacks::Callback;
use crate::handler::Handler;
use crate::markup;
use crate::reply;
use crate::Result;

use super::Callbacks;

impl Handler {
  fn get_my_groups(&self) -> InlineKeyboardMarkup {
    let into_button = |group: &&str| {
      let name = if self.user.config().has_group(group) { format!("✅ {}", group) } else { group.to_string() };
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
  async fn test(self, arg: i32) -> Result<()> {
    self.reply(format!("Тык! {}", arg)).await?;
    self.answer().await?;
    Ok(())
  }

  async fn show_my_groups(self) -> Result<()> {
    let markup = self
      .get_my_groups()
      .append_row([Callback::ShowConfig.with_text("OK").into()]);
    self.edit(reply!(const "set_groups.md")).reply_markup(markup).await?;
    Ok(())
  }

  async fn get_start_link(self) -> Result<()> {
    let me = self.get_me().await?;
    let mut link = String::new();
    for idx in self
      .user
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

  async fn toggle_notifications(mut self) -> Result<()> {
    let config = self.user.config_mut();
    config.set_is_notifies_enabled(!config.is_notifies_enabled());
    self.user.update(&self.pool).await?;
    self.answer().await?;
    self.show_config().await?;
    Ok(())
  }

  async fn set_group(mut self, name: String) -> Result<()> {
    match self.user.config().has_group(&name) {
      true => self.user.config_mut().remove_group(name, &self.pool).await?,
      false => self.user.config_mut().add_group(name, &self.pool).await?,
    }
    self.show_my_groups().await
  }

  async fn show_config(self) -> Result<()> {
    self
      .edit(reply!(const "config.md"))
      .reply_markup(self.config_markup())
      .await?;
    Ok(())
  }

  async fn close(self) -> Result<()> {
    self.delete_message(self.message.chat.id, self.message.id).await?;
    Ok(())
  }
}
