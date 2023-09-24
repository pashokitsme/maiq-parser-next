use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::Interval;
use tokio_util::sync::CancellationToken;

use super::default_lectures::DefaultLectures;
use super::SnapshotParser;
use super::GROUP_NAMES;

use crate::error::*;
use crate::parser::table::*;
use crate::snapshot::*;
use crate::utils::time::*;

use reqwest::Client;

use url::Url;

type Changes = Vec<String>;
type UpdateSender = mpsc::Sender<(Option<Snapshot>, Changes)>;

#[derive(Default)]
pub struct PeriodicalParserBuilder {
  today_remote_url: Option<Url>,
  next_remote_url: Option<Url>,
  interval: Option<Interval>,
  default_lectures: Option<Arc<DefaultLectures>>,
  on_update: Option<UpdateSender>,
}

impl PeriodicalParserBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_today_url<U: AsRef<str>>(self, url: U) -> Result<Self, url::ParseError> {
    Ok(Self { today_remote_url: Some(url.as_ref().parse()?), ..self })
  }

  pub fn with_next_url<U: AsRef<str>>(self, url: U) -> Result<Self, url::ParseError> {
    Ok(Self { next_remote_url: Some(url.as_ref().parse()?), ..self })
  }

  pub fn with_interval(self, interval: Duration) -> Self {
    Self { interval: Some(tokio::time::interval(interval)), ..self }
  }

  pub fn with_default_lectures(self, lectures: DefaultLectures) -> Self {
    Self { default_lectures: Some(Arc::from(lectures)), ..self }
  }

  pub fn on_update(self, on_update: UpdateSender) -> Self {
    Self { on_update: Some(on_update), ..self }
  }

  pub fn build<P: SnapshotParser + Send + Sync + 'static>(self) -> Result<PeriodicalParser<P>, ParserError> {
    Ok(PeriodicalParser {
      http_client: self.reqwest_client()?,
      interval: self
        .interval
        .unwrap_or_else(|| tokio::time::interval(Duration::from_secs(60 * 5))),
      default_lectures: self.default_lectures.unwrap_or_else(|| {
        warn!("default lectures not set");
        Arc::from(DefaultLectures::default())
      }),
      on_update: self.on_update.unwrap(),
      today_remote_url: self.today_remote_url,
      next_remote_url: self.next_remote_url,
      prev_today_snapshot: None,
      prev_next_snapshot: None,
      _marker: PhantomData,
    })
  }

  fn reqwest_client(&self) -> reqwest::Result<Client> {
    Client::builder().timeout(Duration::from_secs(10)).build()
  }
}

#[derive(Debug)]
pub struct PeriodicalParser<P: SnapshotParser + Send + Sync> {
  interval: Interval,
  default_lectures: Arc<DefaultLectures>,
  on_update: UpdateSender,
  http_client: Client,
  today_remote_url: Option<Url>,
  next_remote_url: Option<Url>,
  prev_today_snapshot: Option<Snapshot>,
  prev_next_snapshot: Option<Snapshot>,
  _marker: PhantomData<P>,
}

impl<P: SnapshotParser + Send + Sync + 'static> PeriodicalParser<P> {
  pub fn start(self) -> CancellationToken {
    let token = CancellationToken::new();
    let out_token = token.clone();

    tokio::spawn(self.main_loop(token));
    out_token
  }

  pub fn latest_today(&self) -> Option<&Snapshot> {
    self.prev_today_snapshot.as_ref()
  }

  pub fn latest_next(&self) -> Option<&Snapshot> {
    self.prev_next_snapshot.as_ref()
  }

  async fn main_loop(mut self, token: CancellationToken) {
    while !token.is_cancelled() {
      self.interval.tick().await;

      if let Some(url) = self.today_remote_url.as_ref().cloned() {
        self.prev_today_snapshot = self.parse_and_notify(&url).await;
      }

      if let Some(url) = self.next_remote_url.as_ref().cloned() {
        self.prev_next_snapshot = self.parse_and_notify(&url).await;
      }
    }
  }

  async fn parse_and_notify(&mut self, url: &Url) -> Option<Snapshot> {
    let snapshot = self.parse(url.clone()).await.ok();
    let changes = self
      .prev_today_snapshot
      .as_ref()
      .distinct(snapshot.as_ref(), &GROUP_NAMES);
    if let Err(err) = self.on_update.send((snapshot.clone(), changes)).await {
      error!("can't send parsed snapshot: {:?}", err)
    }
    snapshot
  }

  async fn parse(&self, url: Url) -> Result<Snapshot, ParserError> {
    let table = self.fetch_table(url).await?.unwrap();
    let parser = P::new(DateTime::now())
      .with_groups(GROUP_NAMES.iter())
      .with_default_lectures(self.default_lectures.clone());
    Ok(parser.parse(table))
  }

  async fn fetch_table(&self, url: Url) -> reqwest::Result<Option<Table>> {
    let html_raw = self
      .http_client
      .get(url)
      .send()
      .await?
      .text_with_charset("windows-1251")
      .await?;
    Ok(parse_last_table(&html_raw))
  }
}
