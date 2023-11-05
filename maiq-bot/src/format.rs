use std::fmt::Display;

use maiq_parser_next::prelude::*;

pub struct FormatSnapshot<'a>(&'a Snapshot, FormatGroup<'a>);
pub struct FormatGroup<'a>(pub &'a Group);
pub struct FormatLecture<'a>(pub &'a Lecture);
pub struct FormatDate<'a>(pub &'a DateTime);
pub struct FormatWeekday<'a>(pub &'a Weekday);

impl<'a> FormatSnapshot<'a> {
  pub fn select_group(snapshot: &'a Snapshot, name: &str) -> Option<Self> {
    snapshot.group(name).map(|group| Self(snapshot, FormatGroup(group)))
  }

  pub fn group_name(&self) -> &str {
    self.1 .0.name()
  }
}

impl<'a> Display for FormatSnapshot<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let date = self.0.date();
    writeln!(f, "{} {}, {}", random_emoji(), FormatWeekday(&date.weekday()), FormatDate(&date))?;
    writeln!(f)?;
    writeln!(f, "{}", self.1)
  }
}

impl<'a> Display for FormatGroup<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for lecture in self.0.lectures() {
      write!(f, "{}", FormatLecture(lecture))?
    }
    Ok(())
  }
}

impl<'a> Display for FormatLecture<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(order) = self.0.order() {
      write!(f, "<b>#{}</b> ", order)?;
    }

    if let Some(classroom) = self.0.classroom() {
      write!(f, "{} ", classroom)?;
    }

    if let Some(sub) = self.0.subgroup() {
      write!(f, "· п/г <b>{}</b> ", sub)?;
    }

    writeln!(f, "<b>· {}</b>", self.0.name())
  }
}

impl<'a> Display for FormatWeekday<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let weekday = match self.0 {
      Weekday::Mon => "Понедельник",
      Weekday::Tue => "Вторник",
      Weekday::Wed => "Среда",
      Weekday::Thu => "Четверг",
      Weekday::Fri => "Пятница",
      Weekday::Sat => "Суббота",
      Weekday::Sun => "Воскресенье",
    };
    write!(f, "{}", weekday)
  }
}

impl<'a> Display for FormatDate<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let now = DateTime::now_date().date_naive();
    let date = self.0.format("%d.%m.%Y");
    match self.0.date_naive().signed_duration_since(now).num_days() {
      0 => write!(f, "сегодня, {}", date),
      1 => write!(f, "завтра, {}", date),
      2 => write!(f, "послезавтра, {}", date),
      _ => write!(f, "{}", date),
    }
  }
}

const EMOJIES: [&str; 21] =
  ["🥭", "🥩", "🥝", "🌵", "🥞", "🧀", "🍖", "🍌", "🍍", "🥓", "🧃", "🍒", "🍓", "🍇", "🥕", "🐷", "🍺", "🍪", "🍁", "🍉", "🍋"];

fn random_emoji<'a>() -> &'a str {
  EMOJIES[fastrand::usize(0..EMOJIES.len())]
}
