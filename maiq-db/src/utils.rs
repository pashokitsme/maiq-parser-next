use chrono::prelude::*;

pub type DateTime = chrono::DateTime<FixedOffset>;

const TIME_OFFSET: i32 = 3600 * 3;

pub trait DateTimeExt {
  fn now() -> Self;
}

impl DateTimeExt for DateTime {
  fn now() -> Self {
    let offset = FixedOffset::east_opt(TIME_OFFSET).unwrap();
    Utc::now().with_timezone(&offset).with_nanosecond(0).unwrap()
  }
}
