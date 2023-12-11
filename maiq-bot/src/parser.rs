use std::sync::Arc;
use std::time::Duration;

use teloxide::prelude::*;
use tokio::task::JoinSet;

use crate::format::FormatSnapshot;
use crate::reply;
use crate::Result;
use crate::SnapshotParser;

use maiq_db::models::User;
use maiq_db::Pool;
use maiq_parser_next::prelude::*;

macro_rules! run_shapshot_handler {
  ($e: expr, $bot: expr, $pool: expr) => {
    let res = match $e {
      Ok(Some((snapshot, changes))) if !changes.is_empty() => on_update($bot.clone(), $pool.clone(), snapshot, changes).await,
      Ok(Some(_)) => Ok(()),
      Ok(None) => {
        warn!("snapshot is None; is url set?");
        Ok(())
      },
      Err(err) => on_error(err),
    };

    if let Err(err) = res {
      error!(target: "rx-parser", "error during handling update: {:?}", err);
    }
  };
}

pub fn start_parser_service(bot: Bot, parser: SnapshotParser, pool: Arc<Pool>) -> SnapshotParser {
  let delay_secs = std::env::var("DELAY")
    .ok()
    .and_then(|v| v.parse().ok())
    .unwrap_or_else(|| {
      warn!(target: "rx-parser", "env-var DELAY not set; using 300s");
      300
    });

  let parser_clone = parser.clone();

  tokio::spawn(async move {
    let repeating = RepeatingSnapshotParser::with_interval(parser_clone, Duration::from_secs(delay_secs)).with_update_handler(
      Box::new(move |today, next| {
        let bot = bot.clone();
        let pool = pool.clone();
        tokio::spawn(async move {
          run_shapshot_handler!(today, bot, pool);
          run_shapshot_handler!(next, bot, pool);
        });
      }),
    );
    repeating.start().await
  });

  parser
}

async fn on_update(bot: Bot, pool: Arc<Pool>, snapshot: Snapshot, changes: Vec<String>) -> Result<()> {
  info!(target: "rx-parser", "snapshot: {} changes: {:?}", snapshot.id(), changes);
  let users = User::get_all_notified(&pool).await?;
  let snapshot = Arc::new(snapshot);
  let mut tasks = JoinSet::new();
  users
    .into_iter()
    .map(|(id, mut groups)| {
      groups.retain(|g| changes.contains(g));
      (id, groups)
    })
    .filter(|(_, groups)| !groups.is_empty())
    .map(|(id, groups)| send_snapshot(bot.clone(), snapshot.clone(), id, groups))
    .for_each(|task| {
      tasks.spawn(task);
    });

  let mut ok = 0usize;
  let mut total = 0usize;

  while let Some(task) = tasks.join_next().await {
    total += 1;
    match task {
      Ok(Ok(_)) => ok += 1,
      Ok(Err(err)) => warn!(target: "rx-parser", "unable to send snapshot: {:?}", err),
      Err(err) => error!(target: "rx-target", "task join error: {:?}", err),
    }
  }

  info!(target: "rx-parser", "sent {} ok / {} total", ok, total);
  Ok(())
}

fn on_error(err: maiq_parser_next::error::Error) -> Result<()> {
  if !err.can_be_skipped() {
    warn!(target: "rx-parser", "error during parsing: {:?}", err);
  }
  Ok(())
}

async fn send_snapshot(bot: Bot, snapshot: Arc<Snapshot>, chat_id: i64, groups: Vec<String>) -> Result<()> {
  let send = |msg| {
    bot
      .send_message(ChatId(chat_id), msg)
      .parse_mode(teloxide::types::ParseMode::Html)
      .disable_web_page_preview(true)
  };

  match groups.len() {
    1 => {
      let group_name = groups.first().unwrap();
      if let Some(format) = FormatSnapshot::select_group(&snapshot, group_name) {
        send(format.to_string()).await?;
      }
    }
    _ => {
      for format in groups
        .iter()
        .filter_map(|group| FormatSnapshot::select_group(&snapshot, group))
      {
        send(reply!("snapshot/many_groups.md", group_name = format.group_name(), formatted = format.to_string())).await?;
      }
    }
  }
  Ok(())
}
