use std::fmt::Display;

use maiq_parser_next::prelude::*;

pub struct FormatSnapshot<'a>(&'a Snapshot, FormatGroup<'a>);
pub struct FormatGroup<'a>(pub &'a Group);
pub struct FormatLecture<'a>(pub &'a Lecture);

impl<'a> FormatSnapshot<'a> {
  pub fn select_group(snapshot: &'a Snapshot, name: &str) -> Option<Self> {
    snapshot.group(name).map(|group| Self(snapshot, FormatGroup(group)))
  }
}

impl<'a> Display for FormatSnapshot<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let date = self.0.date();
    // if date == DateTime::now_date() {
    writeln!(f, "{} {}, {}", random_emoji(), date.weekday(), date.format("%d.%m.%Y"))?;
    // }
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

const EMOJIES: [&str; 21] =
  ["🥭", "🥩", "🥝", "🌵", "🥞", "🧀", "🍖", "🍌", "🍍", "🥓", "🧃", "🍒", "🍓", "🍇", "🥕", "🐷", "🍺", "🍪", "🍁", "🍉", "🍋"];

fn random_emoji<'a>() -> &'a str {
  EMOJIES[fastrand::usize(0..EMOJIES.len())]
}
