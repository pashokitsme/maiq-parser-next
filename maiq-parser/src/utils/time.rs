pub use chrono::prelude::*;

pub type DateTime = chrono::DateTime<FixedOffset>;

const TIME_OFFSET: i32 = 3600 * 3;

pub trait DateTimeExt {
  fn now() -> Self;
  fn now_date() -> Self;
}

impl DateTimeExt for DateTime {
  fn now() -> Self {
    let offset = FixedOffset::east_opt(TIME_OFFSET).unwrap();
    Utc::now().with_timezone(&offset).with_nanosecond(0).unwrap()
  }

  fn now_date() -> Self {
    DateTime::now()
      .with_hour(0)
      .unwrap()
      .with_minute(0)
      .unwrap()
      .with_second(0)
      .unwrap()
  }
}
