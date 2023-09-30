use maiq_parser_next::parser;
use maiq_parser_next::parser::SnapshotParser4;
use maiq_parser_next::snapshot::Snapshot;
use teloxide::prelude::*;

use crate::Result;

pub fn start_background(bot: Bot) {
  tokio::spawn(async move { start(bot).await.unwrap() });
}

async fn start(bot: Bot) -> Result<()> {
  let (parser, mut rx) = parser::builder()
    .with_today_url("https://rsp.chemk.org/4korp/today.htm")
    .unwrap()
    .with_next_url("https://rsp.chemk.org/4korp/today.htm")
    .unwrap()
    .build::<SnapshotParser4>()?;

  parser.start();

  while let Some(update) = rx.recv().await {
    match update {
      Ok((snapshot, changes)) => on_update(&bot, snapshot, changes).await,
      Err(err) => on_error(&bot, err).await,
    }
  }

  warn!(target: "parser", "parsing is stopped");

  Ok(())
}

async fn on_update(_bot: &Bot, snapshot: Snapshot, changes: Vec<String>) {
  info!(target: "parser", "{:?}", snapshot);
  info!(target: "parser", "changes: {:?}", changes);
}

async fn on_error(_bot: &Bot, err: maiq_parser_next::error::Error) {
  warn!(target: "parser", "error during parsing: {:?}", err)
}
