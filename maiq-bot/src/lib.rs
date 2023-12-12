mod build_info;
mod callbacks;
mod commands;
mod format;
mod handler;
mod parser;

use anyhow::Result;
use std::sync::Arc;
use teloxide::utils::command::BotCommands;
use tokio::sync::RwLock;

use maiq_parser_next::prelude::*;
use parser::start_parser_service;
use teloxide::dptree::deps;
use teloxide::prelude::*;

use dptree as dp;
use teloxide::dispatching::UpdateHandler;

use handler::Handler;

#[macro_use]
mod macros;

#[macro_use]
extern crate log;

use crate::callbacks::filter_callback;
use crate::callbacks::Callback;
use crate::commands::Command;
use crate::commands::DeveloperCommand;

pub type SnapshotParserImpl = SnapshotParser4;
pub type SnapshotParser = Arc<RwLock<maiq_parser_next::prelude::SnapshotParser<SnapshotParserImpl>>>;

pub const DEVELOPER_ID: u64 = 949248728;

pub trait Caller {
  fn caller(&self) -> Option<&teloxide::types::User>;
  fn caller_name(&self) -> String;
}

pub async fn setup_bot() -> Result<Bot> {
  let bot = Bot::from_env();
  let me = bot.get_me().await?;
  info!(target: "setup", "developer id: {}", DEVELOPER_ID);
  info!(target: "setup", "logged-in as {} [@{}] [#{}]", me.full_name(), me.username(), me.id.0);
  ensure_webhook_not_set(&bot).await?;
  set_commands(&bot).await?;

  Ok(bot)
}

pub fn setup_parser() -> Result<SnapshotParser> {
  let parser = SnapshotParserBuilder::new()
    .with_today_url("https://rsp.chemk.org/4korp/today.htm")
    .unwrap()
    .with_next_url("https://rsp.chemk.org/4korp/tomorrow.htm")
    .unwrap()
    .build()?;
  Ok(Arc::from(RwLock::from(parser)))
}

pub async fn start(bot: Bot, pool: maiq_db::Pool, parser: SnapshotParser) {
  let pool = Arc::new(pool);
  let parser = start_parser_service(bot.clone(), parser, pool.clone());

  Dispatcher::builder(bot, dispatch_tree())
    .worker_queue_size(16)
    .dependencies(deps![parser, pool])
    .build()
    .dispatch()
    .await
}

async fn ensure_webhook_not_set(bot: &Bot) -> Result<()> {
  let info = bot.get_webhook_info().await?;
  if let Some(url) = info.url {
    warn!(target: "setup", "webhook was set to `{}`; removing", url);
    bot.delete_webhook().await?;
  }

  Ok(())
}

async fn set_commands(bot: &Bot) -> Result<()> {
  let commands = Command::bot_commands();
  let commands_str = commands
    .iter()
    .map(|cmd| cmd.command.to_owned())
    .chain(
      DeveloperCommand::bot_commands()
        .iter()
        .map(|cmd| format!("(dev) {}", cmd.command)),
    )
    .collect::<Vec<String>>()
    .join(", ");

  info!(target: "setup", "installed commands: {}", commands_str);

  bot.set_my_commands(commands).await?;
  Ok(())
}

fn dispatch_tree() -> UpdateHandler<anyhow::Error> {
  dp::entry()
    .branch(
      Update::filter_message()
        .filter(|msg: Message| msg.text().is_some())
        .filter_map_async(Handler::with_message)
        .chain(
          dp::entry()
            .branch(
              dp::entry()
                .filter_command::<Command>()
                .endpoint(Command::execute::<Handler>),
            )
            .branch(
              dp::entry()
                .filter(|msg: Message| msg.from().map(|user| user.id.0 == DEVELOPER_ID).unwrap_or(false))
                .filter_command::<DeveloperCommand>()
                .endpoint(DeveloperCommand::execute::<Handler>),
            ),
        ),
    )
    .branch(
      Update::filter_callback_query()
        .filter_map(filter_callback)
        .filter_map_async(Handler::with_callback)
        .endpoint(Callback::execute::<Handler>),
    )
}
