use std::future::Future;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::error::*;
use crate::snapshot::*;

use tokio::time::Interval;
use tokio_util::sync::CancellationToken;
use url::Url;

use super::default_lectures::DefaultLectures;
use super::table::parse_last_table;
use super::table::Table;

type Changes = ();

#[derive(Default)]
pub struct PeriodicalParserBuilder {
  remote_urls: Option<Vec<Url>>,
  interval: Option<Interval>,
  default_lectures: Option<DefaultLectures>,
  on_update: Option<fn(Snapshot, Changes)>,
}

impl PeriodicalParserBuilder {
  pub fn add_url<U: AsRef<str>>(self, url: U) -> Result<Self, url::ParseError> {
    let mut urls = self.remote_urls.unwrap_or(vec![]);
    urls.push(Url::parse(url.as_ref())?);
    Ok(Self { remote_urls: Some(urls), ..self })
  }

  pub fn with_interval(self, interval: Interval) -> Self {
    Self { interval: Some(interval), ..self }
  }

  pub fn with_default_lectures(self, lectures: DefaultLectures) -> Self {
    Self { default_lectures: Some(lectures), ..self }
  }

  pub fn on_update(self, on_update: fn(Snapshot, Changes) -> ()) -> Self {
    Self { on_update: Some(on_update), ..self }
  }

  pub fn build(self) -> Result<PeriodicalParser, ParserError> {
    Ok(PeriodicalParser {
      remote_urls: self.remote_urls.ok_or(BuilderError::UrlNotSet)?,
      interval: self.interval.ok_or(BuilderError::IntervalNotSet)?,
      default_lectures: self.default_lectures.unwrap_or_else(|| {
        warn!("default lectures not set");
        DefaultLectures::default()
      }),
      on_update: self.on_update.unwrap_or(|_, _| warn!("parser update not handled")),
    })
  }
}

pub struct PeriodicalParser {
  remote_urls: Vec<Url>,
  interval: Interval,
  default_lectures: DefaultLectures,
  on_update: fn(Snapshot, Changes),
}

impl PeriodicalParser {
  pub fn start(self) -> (JoinHandle<impl Future<Output = ()>>, CancellationToken) {
    let token = CancellationToken::new();
    let out_token = token.clone();

    let handle = std::thread::spawn(move || async { self.main_loop(token).await });
    (handle, out_token)
  }

  async fn main_loop(mut self, token: CancellationToken) {
    while !token.is_cancelled() {
      self.interval.tick().await;
      for url in self.remote_urls.iter() {
        self.parse(url.clone()).await
      }
    }
  }

  async fn parse(&self, url: Url) {
    let table = self.fetch_table(url).await.unwrap().unwrap();
    let parser = ();
  }

  async fn fetch_table(&self, url: Url) -> reqwest::Result<Option<Table>> {
    let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).build()?;
    let html_raw = client
      .get(url)
      .send()
      .await?
      .text_with_charset("windows-1251")
      .await?;
    Ok(parse_last_table(&html_raw))
  }
}
