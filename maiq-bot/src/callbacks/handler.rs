use maiq_parser_next::prelude::GROUP_NAMES;
use teloxide::payloads::SendMessageSetters;
use teloxide::types::InlineKeyboardMarkup;

use crate::callbacks::Callback;
use crate::handler::Handler;
use crate::Result;

use super::Callbacks;

impl Handler {
  fn get_my_groups(&self) -> InlineKeyboardMarkup {
    let into_button = |group: &&str| {
      let name = if self.user.config().groups().any(|g| g == group) { format!("[✳️] {}", group) } else { group.to_string() };
      Callback::SetGroup { name: group.to_string() }.with_text(name).into()
    };

    let buttons = GROUP_NAMES
      .iter()
      .step_by(2)
      .zip(GROUP_NAMES.iter().step_by(2).skip(1).map(into_button))
      .map(|(group, btn)| [into_button(group), btn]);

    InlineKeyboardMarkup::new(buttons)
  }
}

impl Callbacks for Handler {
  async fn test(self, arg: i32) -> Result<()> {
    self.reply(format!("Тык! {}", arg)).await?;
    self.answer().await?;
    Ok(())
  }

  async fn show_my_groups(self) -> Result<()> {
    let markup = self.get_my_groups();
    self.reply("test").reply_markup(markup).await?;
    Ok(())
  }

  async fn set_group(self, name: String) -> Result<()> {
    info!("set: {}", name);
    Ok(())
  }
}
