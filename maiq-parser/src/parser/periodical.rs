use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::time::Interval;

use super::default_lectures::DefaultLectures;
use super::Parser;
use super::GROUP_NAMES;

use crate::error::*;
use crate::parser::table::*;
use crate::snapshot::*;
use crate::utils::time::*;

use url::Url;

pub type ParserPair<P> = (SnapshotParser<P>, Receiver<Result<(Snapshot, Changes), Error>>);
pub type LoopedParserPair<P> = (LoopedSnapshotParser<P>, Receiver<Result<(Snapshot, Changes), Error>>);

type Changes = Vec<String>;
type UpdateCallback = Result<(Snapshot, Changes), Error>;

#[derive(Default)]
pub struct SnapshotParserBuilder {
  today_remote_url: Option<Url>,
  next_remote_url: Option<Url>,
  default_lectures: Option<Arc<DefaultLectures>>,
}

impl SnapshotParserBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_today_url<U: AsRef<str>>(self, url: U) -> Result<Self, url::ParseError> {
    Ok(Self { today_remote_url: Some(url.as_ref().parse()?), ..self })
  }

  pub fn with_next_url<U: AsRef<str>>(self, url: U) -> Result<Self, url::ParseError> {
    Ok(Self { next_remote_url: Some(url.as_ref().parse()?), ..self })
  }

  pub fn with_default_lectures(self, lectures: DefaultLectures) -> Self {
    Self { default_lectures: Some(Arc::from(lectures)), ..self }
  }

  pub fn build<P: Parser + Send + Sync + 'static>(self) -> Result<ParserPair<P>, Error> {
    let (tx, rx) = mpsc::channel(8);
    let parser = SnapshotParser {
      default_lectures: self.default_lectures.unwrap_or_else(|| {
        warn!(target: "parser", "default lectures not set");
        Arc::from(DefaultLectures::default())
      }),
      on_update: tx,
      today_remote_url: self.today_remote_url,
      next_remote_url: self.next_remote_url,
      prev_today_snapshot: None,
      prev_next_snapshot: None,
      _marker: PhantomData,
    };

    Ok((parser, rx))
  }
}

#[derive(Debug)]
pub struct LoopedSnapshotParser<P: Parser + Send + Sync + 'static> {
  interval: Interval,
  parser: Arc<RwLock<SnapshotParser<P>>>,
}

impl<P: Parser + Send + Sync + 'static> LoopedSnapshotParser<P> {
  pub fn new(parser: Arc<RwLock<SnapshotParser<P>>>) -> Self {
    Self { interval: tokio::time::interval(Duration::from_secs(60 * 5)), parser }
  }

  pub fn with_interval(parser: Arc<RwLock<SnapshotParser<P>>>, interval: Duration) -> Self {
    Self { interval: tokio::time::interval(interval), parser }
  }

  pub async fn start(mut self) {
    loop {
      self.interval.tick().await;
      info!(target: "parser", "tick!");
      let (today, next) = self.parser.read().await.check().await;
      let mut parser = self.parser.write().await;
      parser.prev_today_snapshot = today;
      parser.prev_next_snapshot = next;
    }
  }
}

#[derive(Debug)]
pub struct SnapshotParser<P: Parser + Send + Sync> {
  default_lectures: Arc<DefaultLectures>,
  on_update: Sender<UpdateCallback>,
  today_remote_url: Option<Url>,
  next_remote_url: Option<Url>,
  prev_today_snapshot: Option<Snapshot>,
  prev_next_snapshot: Option<Snapshot>,
  _marker: PhantomData<P>,
}

impl<P: Parser + Send + Sync + 'static> SnapshotParser<P> {
  pub fn latest_today(&self) -> Option<&Snapshot> {
    self.prev_today_snapshot.as_ref()
  }

  pub fn latest_next(&self) -> Option<&Snapshot> {
    self.prev_next_snapshot.as_ref()
  }

  pub async fn check(&self) -> (Option<Snapshot>, Option<Snapshot>) {
    let today = if let Some(url) = self.today_remote_url.as_ref().cloned() {
      self
        .parse_and_notify(&url, self.prev_today_snapshot.as_ref())
        .await
        .ok()
    } else {
      None
    };

    let next = if let Some(url) = self.next_remote_url.as_ref().cloned() {
      self
        .parse_and_notify(&url, self.prev_next_snapshot.as_ref())
        .await
        .ok()
    } else {
      None
    };
    (today, next)
  }

  pub fn looped(self, interval: Duration) -> LoopedSnapshotParser<P> {
    LoopedSnapshotParser { interval: tokio::time::interval(interval), parser: Arc::new(RwLock::new(self)) }
  }

  async fn parse_and_notify(&self, url: &Url, prev: Option<&Snapshot>) -> Result<Snapshot, Error> {
    let snapshot = self.parse(url.clone()).await;

    if let Err(ref err) = snapshot {
      error!(target: "parser", "can't parse snapshot from {}: {:?}", url.as_str(), err);
    }

    let changes = prev.distinct(snapshot.as_ref().ok(), &GROUP_NAMES);

    if let Err(err) = self.on_update.send(snapshot.clone().map(|s| (s, changes))).await {
      error!(target: "parser", "can't send parsed snapshot: {:?}", err)
    }

    snapshot
  }

  async fn parse(&self, url: Url) -> Result<Snapshot, Error> {
    let table = self.fetch_table(url).await?.ok_or(Error::NoHtmlTable)?;
    println!("{table:?}");
    let parser = P::new(DateTime::now())
      .with_groups(GROUP_NAMES.iter())
      .with_default_lectures(self.default_lectures.clone());
    Ok(parser.parse(table))
  }

  async fn fetch_table(&self, url: Url) -> Result<Option<Table>, ureq::Error> {
    let mut reader = ureq::get(url.as_str())
      .timeout(Duration::from_secs(5))
      .call()?
      .into_reader();
    let mut buf = vec![];
    reader.read_to_end(&mut buf)?;
    let html_raw = encoding_rs::WINDOWS_1251.decode(&buf).0;

    Ok(parse_last_table(&html_raw))
  }
}
