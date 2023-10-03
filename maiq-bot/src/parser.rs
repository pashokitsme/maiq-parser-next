use std::sync::Arc;
use std::time::Duration;

use crate::SnapshotParserImpl;
use maiq_parser_next::parser::LoopedSnapshotParser;
use maiq_parser_next::prelude::*;
use teloxide::prelude::*;
use tokio::sync::RwLock;

// pub fn start_background(parser: Parser<SnapshotParser4>) {}

pub fn start_parser_service(bot: Bot, parser: ParserPair<SnapshotParserImpl>) -> Arc<RwLock<SnapshotParser<SnapshotParserImpl>>> {
  let mut rx = parser.1;
  let parser = Arc::new(RwLock::new(parser.0));
  let parser_looped = LoopedSnapshotParser::with_interval(parser.clone(), Duration::from_secs(10));
  tokio::spawn(async move { parser_looped.start().await });
  tokio::spawn(async move {
    while let Some(update) = rx.recv().await {
      match update {
        Ok((snapshot, changes)) => on_update(&bot, snapshot, changes).await,
        Err(err) => on_error(&bot, err).await,
      }
    }
    warn!(target: "parser", "parsing is stopped");
  });

  parser
}

async fn on_update(_bot: &Bot, snapshot: Snapshot, changes: Vec<String>) {
  info!(target: "parser", "{:?}", snapshot);
  info!(target: "parser", "changes: {:?}", changes);
}

async fn on_error(_bot: &Bot, err: maiq_parser_next::error::Error) {
  warn!(target: "parser", "error during parsing: {:?}", err)
}
