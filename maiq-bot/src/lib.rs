mod error;

use dptree as dp;
use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;
use teloxide::RequestError;

#[macro_use]
extern crate log;

pub const DEVELOPER_ID: u64 = 949248728;

pub async fn setup_bot() -> Result<(), RequestError> {
  let bot = Bot::from_env();
  let me = bot.get_me().await?;
  info!(target: "setup", "developer id: {}", DEVELOPER_ID);
  info!(target: "setup", "logged-in as {} [@{}] [#{}]", me.full_name(), me.username(), me.id.0);
  ensure_webhook_not_set(&bot).await?;

  Ok(())
}

async fn ensure_webhook_not_set(bot: &Bot) -> Result<(), RequestError> {
  let info = bot.get_webhook_info().await?;
  if let Some(url) = info.url {
    warn!(target: "setup", "webhook was set to `{}`; removing", url);
    bot.delete_webhook().await?;
  }

  Ok(())
}

fn build_dispatcher<E: 'static>() -> UpdateHandler<E> {
  dp::entry()
}
