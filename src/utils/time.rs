pub use chrono::prelude::*;

use chrono_tz::Europe::Moscow;
use chrono_tz::Tz;

pub type DateTime = chrono::DateTime<Tz>;

pub trait DateTimeExt {
  fn now() -> Self;
}

impl DateTimeExt for DateTime {
  fn now() -> Self {
    Utc::now().with_timezone(&Moscow)
  }
}
