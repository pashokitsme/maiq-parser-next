use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::Interval;
use tokio_util::sync::CancellationToken;

use super::default_lectures::DefaultLectures;
use super::ParserContext;
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
  remote_urls: Option<Vec<Url>>,
  interval: Option<Interval>,
  default_lectures: Option<Arc<DefaultLectures>>,
  on_update: Option<UpdateSender>,
}

impl PeriodicalParserBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_url<U: AsRef<str>>(self, url: U) -> Result<Self, url::ParseError> {
    let mut urls = self.remote_urls.unwrap_or(vec![]);
    urls.push(Url::parse(url.as_ref())?);
    Ok(Self { remote_urls: Some(urls), ..self })
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

  pub fn build(self) -> Result<PeriodicalParser, ParserError> {
    Ok(PeriodicalParser {
      http_client: self.reqwest_client()?,
      remote_urls: self.remote_urls.ok_or(BuilderError::UrlNotSet)?,
      interval: self
        .interval
        .unwrap_or_else(|| tokio::time::interval(Duration::from_secs(60 * 5))),
      default_lectures: self.default_lectures.unwrap_or_else(|| {
        warn!("default lectures not set");
        Arc::from(DefaultLectures::default())
      }),
      on_update: self.on_update.unwrap(),
      prev_snapshot: None,
    })
  }

  fn reqwest_client(&self) -> reqwest::Result<Client> {
    Client::builder().timeout(Duration::from_secs(10)).build()
  }
}

pub struct PeriodicalParser {
  remote_urls: Vec<Url>,
  interval: Interval,
  default_lectures: Arc<DefaultLectures>,
  on_update: UpdateSender,
  http_client: Client,
  prev_snapshot: Option<Snapshot>,
}

impl PeriodicalParser {
  pub fn start(self) -> CancellationToken {
    let token = CancellationToken::new();
    let out_token = token.clone();

    tokio::spawn(self.main_loop(token));
    out_token
  }

  async fn main_loop(mut self, token: CancellationToken) {
    while !token.is_cancelled() {
      self.interval.tick().await;
      for url in self.remote_urls.iter() {
        let snapshot = self.parse(url.clone()).await.ok();
        let changes = self.prev_snapshot.as_ref().distinct(snapshot.as_ref(), &GROUP_NAMES);
        self.prev_snapshot = snapshot.clone();
        if let Err(err) = self.on_update.send((snapshot, changes)).await {
          error!("can't send parsed snapshot: {:?}", err)
        }
      }
    }
  }

  async fn parse(&self, url: Url) -> Result<Snapshot, ParserError> {
    let table = self.fetch_table(url).await?.unwrap();
    let parser = ParserContext::new(true, DateTime::now())
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
