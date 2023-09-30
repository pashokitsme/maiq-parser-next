pub mod default_lectures;
pub mod impl_buildings;
pub mod table;

mod parse_date;
mod periodical;

pub use impl_buildings::*;
pub use periodical::*;

const GROUP_NAMES: [&str; 32] = [
  "Ит1-23", "Ит3-23", "Ит1-22", "Са1-22", "Са3-22", "С1-22", "С3-22", "Ир1-22", "Ир3-22", "Ир5-22", "Са1-21", "Са3-21", "С1-21",
  "С3-21", "Ип1-21", "Ип3-21", "Ип5-21", "Ир1-21", "Ир3-21", "Ир5-21", "С1-20", "С3-20", "Кс1-20", "Кс3-20", "Кс5-20", "Ип1-20",
  "Ип3-20", "Ир1-20", "Ир3-20", "Ир5-20", "С1-19", "С3-19",
];

use self::default_lectures::DefaultLectures;
use self::table::Table;
use crate::snapshot::*;
use crate::utils::time::DateTime;
use std::sync::Arc;

pub trait SnapshotParser {
  fn new(fallback_date: DateTime) -> Self;
  fn with_groups<S: AsRef<str>, I: Iterator<Item = S>>(self, group_names: I) -> Self;
  fn with_default_lectures(self, lectures: Arc<DefaultLectures>) -> Self;
  fn parse(self, table: Table) -> Snapshot;
}

pub fn parser_builder() -> LoopSnapshotParserBuilder {
  LoopSnapshotParserBuilder::new()
}
