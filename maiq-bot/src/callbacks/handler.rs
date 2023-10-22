use teloxide::payloads::EditMessageTextSetters;
use teloxide::types::InlineKeyboardMarkup;

use maiq_parser_next::prelude::*;

use crate::callbacks::Callback;
use crate::handler::Handler;
use crate::markup;
use crate::reply;
use crate::Result;

use super::Callbacks;

impl Handler {
  fn get_my_groups(&self) -> InlineKeyboardMarkup {
    let into_button = |group: &&str| {
      let name = if self.user.config().groups().any(|g| g == group) { format!("✅ {}", group) } else { group.to_string() };
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
    self.edit("test").reply_markup(markup).await?;
    Ok(())
  }

  async fn set_group(self, name: String) -> Result<()> {
    Ok(())
  }

  async fn show_config(self) -> Result<()> {
    self
      .edit(reply!("config.md"))
      .reply_markup(Callback::SetMyGroups.with_text("Настроить группы").into())
      .await?;
    Ok(())
  }
}
