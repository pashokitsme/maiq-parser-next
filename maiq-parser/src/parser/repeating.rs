use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use crate::Error;
use tokio::sync::RwLock;
use tokio::time::Interval;

use super::default_lectures::DefaultLectures;
use super::SnapshotParserAgent;
use super::DEFAULT_TIME_BOUNDS;
use super::GROUP_NAMES;

use crate::parser::table::*;
use crate::snapshot::*;
use crate::utils::time::*;

use url::Url;

type SnapshotParseResult = Result<Option<(Snapshot, Changes)>, Error>;
type SnapshotUpdateCallback = Box<dyn Fn(SnapshotParseResult, SnapshotParseResult) + Send>;

pub struct RepeatingSnapshotParser<P: SnapshotParserAgent + Send + Sync + 'static> {
  parser: Arc<RwLock<SnapshotParser<P>>>,
  interval: Interval,
  time_bounds: std::ops::Range<u32>,
  callback: Option<SnapshotUpdateCallback>,
}

impl<P: SnapshotParserAgent + Send + Sync + 'static> RepeatingSnapshotParser<P> {
  pub fn new(parser: Arc<RwLock<SnapshotParser<P>>>) -> Self {
    Self {
      interval: tokio::time::interval(Duration::from_secs(60 * 5)),
      parser,
      time_bounds: DEFAULT_TIME_BOUNDS,
      callback: None,
    }
  }

  pub fn with_interval(parser: Arc<RwLock<SnapshotParser<P>>>, interval: Duration) -> Self {
    Self { interval: tokio::time::interval(interval), parser, time_bounds: DEFAULT_TIME_BOUNDS, callback: None }
  }

  pub fn with_time_bounds(self, time_bounds: std::ops::Range<u32>) -> Self {
    Self { time_bounds, ..self }
  }

  pub fn with_update_handler(mut self, f: SnapshotUpdateCallback) -> Self {
    self.callback.replace(f);
    self
  }

  pub async fn start(mut self) {
    let mut should_invoke_handler = false;
    let callback = self
      .callback
      .unwrap_or_else(|| Box::from(|t, n| warn!("unused update: {t:?} & {n:?}")));
    loop {
      self.interval.tick().await;
      if !self.time_bounds.contains(&DateTime::now().time().hour()) {
        should_invoke_handler = false;
        continue;
      }

      debug!(target: "parser", "tick!");
      let today = self.parser.read().await.fetch_today().await;
      let next = self.parser.read().await.fetch_next().await;
      let mut parser = self.parser.write().await;

      if let Ok(today) = today.as_ref() {
        parser.prev_today_snapshot = today.as_ref().map(|t| t.0.clone());
      }

      if let Ok(next) = next.as_ref() {
        parser.prev_next_snapshot = next.as_ref().map(|n| n.0.clone());
      }

      if should_invoke_handler {
        callback(today, next);
      }

      should_invoke_handler = true;
    }
  }
}

#[derive(Debug)]
pub struct SnapshotParser<P: SnapshotParserAgent + Send + Sync> {
  pub(crate) default_lectures: DefaultLectures,
  pub(crate) today_remote_url: Option<Url>,
  pub(crate) next_remote_url: Option<Url>,
  pub(crate) prev_today_snapshot: Option<Snapshot>,
  pub(crate) prev_next_snapshot: Option<Snapshot>,
  pub(crate) _marker: PhantomData<P>,
}

impl<P: SnapshotParserAgent + Send + Sync + 'static> SnapshotParser<P> {
  pub fn latest_today(&self) -> Option<&Snapshot> {
    self.prev_today_snapshot.as_ref()
  }

  pub fn latest_next(&self) -> Option<&Snapshot> {
    self.prev_next_snapshot.as_ref()
  }

  pub async fn fetch_today(&self) -> SnapshotParseResult {
    if let Some(url) = self.today_remote_url.as_ref().cloned() {
      self
        .parse_exact(url, self.prev_today_snapshot.as_ref())
        .await
        .map(Some)
    } else {
      Ok(None)
    }
  }

  pub async fn fetch_next(&self) -> SnapshotParseResult {
    if let Some(url) = self.next_remote_url.as_ref().cloned() {
      self
        .parse_exact(url, self.prev_next_snapshot.as_ref())
        .await
        .map(Some)
    } else {
      Ok(None)
    }
  }

  async fn parse_exact(&self, url: Url, prev: Option<&Snapshot>) -> Result<(Snapshot, Changes), Error> {
    let table = self
      .fetch_table(url)
      .await
      .map_err(Box::from)?
      .ok_or(Error::NoHtmlTable)?;
    let parser = P::new(DateTime::now())
      .with_groups(GROUP_NAMES.iter())
      .with_default_lectures(self.default_lectures.clone());
    let snapshot = parser.parse(table);
    let changes = prev.changes(Some(&snapshot), &GROUP_NAMES);
    Ok((snapshot, changes))
  }

  async fn fetch_table(&self, url: Url) -> Result<Option<Table>, ureq::Error> {
    let mut reader = ureq::get(url.as_str())
      .timeout(Duration::from_secs(15))
      .call()?
      .into_reader();
    let mut buf = vec![];
    reader.read_to_end(&mut buf)?;
    let html_raw = encoding_rs::WINDOWS_1251.decode(&buf).0;

    Ok(parse_last_table(&html_raw))
  }
}
