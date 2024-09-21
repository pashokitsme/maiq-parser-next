use std::fmt::Display;
use std::iter::Peekable;

use crate::parser::SnapshotParserAgent;

use crate::parser::default_lectures::*;
use crate::parser::parse_date::*;
use crate::parser::table::*;
use crate::snapshot::*;
use crate::utils::time::*;

macro_rules! empty_to_none {
  ($e: expr) => {
    match $e {
      Some(x) if !x.is_empty() => Some(x.into()),
      _ => None,
    }
  };
}

const PREVIOUS_ORDER_PLACEHOLDER: &str = "-1";

#[derive(Clone, Default, Debug)]
struct RawLecture {
  order: Option<Box<str>>,
  group_name: Option<Box<str>>,
  subgroup: Option<Box<str>>,
  name: Option<Box<str>>,
  teacher: Option<Box<str>>,
  classroom: Option<Box<str>>,
}

pub struct SnapshotParser4 {
  default_lectures: DefaultLectures,
  fallback_date: DateTime,
  group_names: Vec<Box<str>>,
}

impl SnapshotParserAgent for SnapshotParser4 {
  fn new(fallback_date: DateTime) -> Self {
    Self { default_lectures: DefaultLectures::default(), fallback_date, group_names: vec![] }
  }

  fn with_default_lectures(self, lectures: DefaultLectures) -> Self {
    Self { default_lectures: lectures, ..self }
  }

  fn with_groups<S: AsRef<str>, I: Iterator<Item = S>>(self, group_names: I) -> Self {
    let group_names = group_names
      .map(|name| name.as_ref().into())
      .collect::<Vec<Box<str>>>();
    Self { group_names, ..self }
  }

  fn parse(self, table: Table) -> Snapshot {
    let mut rows = table.rows.into_iter();
    let date = parse_date(&mut rows).unwrap_or(self.fallback_date);
    let is_week_even = date.iso_week().week0() % 2 == 0;

    let raw_lectures = self.parse_raw_lectures(rows);
    let groups = self.assign_to_groups(raw_lectures.into_iter(), is_week_even);
    Snapshot::new(date, groups)
  }
}

impl SnapshotParser4 {
  fn parse_raw_lectures<I: Iterator<Item = Vec<String>>>(&self, rows: I) -> Vec<RawLecture> {
    let mut raw_lectures = vec![];

    todo!();

    raw_lectures
  }

  fn assign_to_groups<I: Iterator<Item = RawLecture>>(self, lectures: I, is_week_even: bool) -> Vec<Group> {
    let mut prev: Option<RawLecture> = None;

    let mut groups = self
      .group_names
      .iter()
      .map(|name| Group::new(name, vec![]))
      .collect::<Vec<Group>>();

    lectures
      .map(|mut lecture| {
        if matches!(lecture.order.as_deref(), Some(PREVIOUS_ORDER_PLACEHOLDER)) {
          lecture.order = prev.as_ref().and_then(|p| p.order.clone())
        }
        prev = Some(lecture.clone());
        lecture
      })
      .filter(|l| l.group_name.is_some() && !matches!(l.name.as_deref(), None | Some("Нет") | Some("нет")))
      .for_each(|lecture| {
        let group_name = lecture.group_name.as_deref().unwrap();
        let group = groups.iter_mut().find(|x| x.name() == group_name);
        if group.is_none() {
          return;
        }
        let lectures = self.expand_raw_lecture(lecture, is_week_even);
        group.unwrap().push_lectures(lectures.into_iter());
      });

    groups.retain(|g| g.has_lectures());
    groups
  }

  fn expand_raw_lecture(&self, lecture: RawLecture, is_week_even: bool) -> Vec<Lecture> {
    if matches!(lecture.name.as_deref(), None | Some("По расписанию") | Some("по расписанию")) {
      if let Some(default_lecture) = self
        .default_lectures
        .group(lecture.group_name.unwrap())
        .and_then(|mut lectures| {
          lectures.find(|lecture| match lecture.week {
            LectureWeek::Even => is_week_even,
            LectureWeek::Odd => !is_week_even,
            LectureWeek::Every => true,
          })
        })
      {
        return lecture
          .order
          .unwrap_or_else(|| default_lecture.order().unwrap_or_default().into())
          .split(',')
          .map(|order| {
            Lecture::new(
              Some(order.trim().into()),
              default_lecture.name().into(),
              lecture
                .classroom
                .clone()
                .or_else(|| default_lecture.classroom().map(Into::into)),
              lecture
                .subgroup
                .clone()
                .or_else(|| default_lecture.subgroup().map(Into::into)),
              lecture
                .teacher
                .clone()
                .or_else(|| default_lecture.teacher().map(Into::into)),
            )
          })
          .collect();
      }
    }

    lecture
      .order
      .unwrap_or_default()
      .split(',')
      .map(|order| {
        Lecture::new(
          Some(order.trim().into()),
          lecture.name.clone().unwrap_or_default(),
          lecture.classroom.clone(),
          lecture.subgroup.clone(),
          lecture.teacher.clone(),
        )
      })
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[rstest]
  #[case("1")]
  #[case("1,2,3,")]
  #[case("2,3")]
  #[case("1,2,3(1ч)")]
  #[case("")]
  fn correct_order(#[case] order: &str) {
    assert!(is_correct_order(order))
  }
}
