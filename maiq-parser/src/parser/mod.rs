pub mod default_lectures;
pub mod impls;
pub mod repeating;
pub mod table;

mod parse_date;

pub const GROUP_NAMES: [&str; 34] = [
  "Ит1-23", "Ит3-23", "Ит1-22", "Са1-22", "Са3-22", "С1-22", "С3-22", "Ир1-22", "Ир3-22", "Ир5-22", "Са1-21", "Са3-21", "С1-21",
  "С3-21", "Ип1-21", "Ип3-21", "Ип5-21", "Ир1-21", "Ир3-21", "Ир5-21", "С1-20", "С3-20", "Кс1-20", "Кс3-20", "Кс5-20", "Ип1-20",
  "Ип3-20", "Ир1-20", "Ир3-20", "Ир5-20", "С1-19", "С3-19", "ЗК1-21", "ЗК1-22",
];

pub const DEFAULT_TIME_BOUNDS: std::ops::Range<u32> = 7..18;

use std::marker::PhantomData;

use url::Url;

use self::default_lectures::DefaultLectures;
use self::repeating::SnapshotParser;
use self::table::Table;
use crate::snapshot::*;
use crate::utils::time::DateTime;
use crate::Error;

pub trait SnapshotParserAgent {
  fn new(fallback_date: DateTime) -> Self;
  fn with_groups<S: AsRef<str>, I: Iterator<Item = S>>(self, group_names: I) -> Self;
  fn with_default_lectures(self, lectures: DefaultLectures) -> Self;
  fn parse(self, table: Table) -> Snapshot;
}

#[derive(Default)]
pub struct SnapshotParserBuilder {
  today_remote_url: Option<Url>,
  next_remote_url: Option<Url>,
  default_lectures: Option<DefaultLectures>,
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
    Self { default_lectures: Some(lectures), ..self }
  }

  pub fn build<P: SnapshotParserAgent + Send + Sync + 'static>(self) -> Result<SnapshotParser<P>, Error> {
    let parser = SnapshotParser {
      default_lectures: self.default_lectures.unwrap_or_else(|| {
        warn!(target: "parser", "default lectures not set");
        DefaultLectures::default()
      }),
      today_remote_url: self.today_remote_url,
      next_remote_url: self.next_remote_url,
      prev_today_snapshot: None,
      prev_next_snapshot: None,
      _marker: PhantomData,
    };

    Ok(parser)
  }
}
