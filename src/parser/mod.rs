mod table;

use tokio::time::Interval;
use url::Url;

use crate::error::*;
use table::Table;

#[derive(Default)]
pub struct PeriodicalParserBuilder {
  remote_urls: Option<Vec<Url>>,
  interval: Option<Interval>,
  default_lectures: Option<()>,
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

  pub fn with_default_lectures(self, lectures: ()) -> Self {
    Self { default_lectures: Some(lectures), ..self }
  }

  pub fn build(self) -> Result<PeriodicalParser, ParserError> {
    Ok(PeriodicalParser {
      remote_urls: self.remote_urls.ok_or(BuilderError::UrlNotSet)?,
      interval: self.interval.ok_or(BuilderError::IntervalNotSet)?,
      default_lectures: self.default_lectures.unwrap_or_else(|| {
        warn!("default lectures not set");
      }),
    })
  }
}

pub struct PeriodicalParser {
  remote_urls: Vec<Url>,
  interval: Interval,
  default_lectures: (),
}

pub struct ParserContext {
  is_week_even: bool,
  table: Table,
  default_lectures: (),
}
