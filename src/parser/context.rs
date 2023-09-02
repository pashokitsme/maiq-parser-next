use std::ops::Deref;

use super::table::*;
use crate::snapshot::*;
use crate::utils::time::*;

use super::parse_date::parse_date;

macro_rules! empty_to_none {
  ($e: expr) => {
    match $e {
      Some(x) if !x.is_empty() => Some(x.into()),
      _ => None,
    }
  };
}

const PREVIOS_ORDER_PLACEHOLDER: &str = "-1";

#[derive(Clone, Default, Debug)]
struct RawLecture {
  order: Option<Box<str>>,
  group_name: Option<Box<str>>,
  subgroup: Option<Box<str>>,
  name: Option<Box<str>>,
  teacher: Option<Box<str>>,
  classroom: Option<Box<str>>,
}

pub struct ParserContext {
  is_week_even: bool,
  default_lectures: (),
  fallback_date: DateTime,
  group_names: Vec<Box<str>>,
}

impl ParserContext {
  pub fn with_groups<S: AsRef<str>, I: Iterator<Item = S>>(self, group_names: I) -> Self {
    let group_names = group_names
      .map(|name| name.as_ref().into())
      .collect::<Vec<Box<str>>>();
    Self { group_names, ..self }
  }

  pub fn parse(self, table: Table) -> Snapshot {
    let mut rows = table.rows.into_iter();
    let date = parse_date(&mut rows).unwrap_or(self.fallback_date);

    let raw_lectures = self.parse_raw_lectures(rows);
    let mut groups = self.repair_and_assign(raw_lectures.into_iter());
    // replace_all_default(&mut groups, date);
    groups.retain(|g| g.has_lectures());
    Snapshot::new(date, vec![])
  }

  fn parse_raw_lectures<S: AsRef<str>, I: Iterator<Item = Vec<S>>>(&self, rows: I) -> Vec<RawLecture> {
    let mut anchor = "Unknown group".into();
    rows
      .map(|row| self.parse_raw_lecture(row.into_iter(), &mut anchor))
      .collect()
  }

  fn parse_raw_lecture<S: AsRef<str>, I: Iterator<Item = S>>(&self, mut row: I, anchor: &mut Box<str>) -> RawLecture {
    let ([group_name, subgroup], order) = {
      match row.next() {
        Some(cell) if self.is_group_name(cell.as_ref()) => {
          if &**anchor != cell.as_ref() {
            *anchor = Box::from(cell.as_ref());
          }
          (split_group_name(Some(cell)), parse_order(&mut row))
        }
        Some(cell) => (split_group_name(Some(&anchor)), Some(Box::from(cell.as_ref()))),
        _ => return RawLecture::default(),
      }
    };

    let [name, teacher] = split_teacher(row.next());
    let classroom = match row.next() {
      Some(x) if !x.as_ref().trim().is_empty() => Some(Box::from(x.as_ref().trim())),
      _ => None,
    };

    RawLecture { order, group_name, subgroup, name, teacher, classroom }
  }

  fn repair_and_assign<I: Iterator<Item = RawLecture>>(self, lectures: I) -> Vec<Group> {
    let mut prev: Option<RawLecture> = None;

    let mut groups = self
      .group_names
      .iter()
      .map(|name| Group::new(name, vec![]))
      .collect::<Vec<Group>>();

    for lecture in lectures
      .map(|mut lecture| {
        if matches!(lecture.order.as_deref(), Some(PREVIOS_ORDER_PLACEHOLDER)) {
          lecture.order = prev.as_ref().and_then(|p| p.order.clone())
        }
        prev = Some(lecture.clone());
        lecture
      })
      .filter(|l| l.group_name.is_some() && !matches!(l.name.as_deref(), None | Some("Нет") | Some("нет")))
    {
      let group_name = lecture.group_name.as_deref().unwrap();
      let group = groups.iter_mut().find(|x| x.name() == group_name).unwrap();
      let lectures = lecture
        .order
        .unwrap_or_default()
        .split(',')
        .map(|x| Some(Box::from(x.trim())))
        .map(|order| {
          Lecture::new(
            order,
            lecture.name.clone().unwrap_or_default(),
            lecture.classroom.clone(),
            lecture.subgroup.clone(),
            lecture.teacher.clone(),
          )
        })
        .collect::<Vec<Lecture>>();

      group.set_lectures(lectures);
    }

    groups
  }

  fn is_group_name(&self, name: &str) -> bool {
    let name = name.split(' ').next().unwrap_or_default();
    self.group_names.iter().any(|group| group.as_ref() == name)
  }
}

fn parse_order<'o, S: AsRef<str>, I: Iterator<Item = S>>(row: &mut I) -> Option<Box<str>> {
  match row.peekable().peek().map(is_correct_order).unwrap_or(false) {
    true => match row.next() {
      Some(x) if !x.as_ref().trim().is_empty() => Some(x.as_ref().trim().into()),
      _ => None,
    },
    false => Some(Box::from(PREVIOS_ORDER_PLACEHOLDER)),
  }
}

fn is_correct_order<S: AsRef<str>>(raw: S) -> bool {
  const SKIP_CHARS: [char; 6] = ['(', ')', ',', '.', 'ч', ' '];
  raw
    .as_ref()
    .chars()
    .all(|c| SKIP_CHARS.contains(&c) || c.is_numeric())
}

fn split_teacher<S: AsRef<str>>(raw: Option<S>) -> [Option<Box<str>>; 2] {
  let raw = match raw {
    Some(x) => x,
    None => return [None, None],
  };

  if let Some((name, teacher)) = raw.as_ref().rsplit_once(',') {
    return [Some(name.into()), empty_to_none!(Some(teacher.trim()))];
  }
  [empty_to_none!(Some(raw.as_ref())), None]
}

fn split_group_name<'s, S: AsRef<str>>(raw: Option<S>) -> [Option<Box<str>>; 2] {
  let raw = match raw {
    Some(x) => x,
    None => return [None, None],
  };

  let mut split = raw.as_ref().split(' ').map(|x| x.trim());
  [empty_to_none!(split.next()), empty_to_none!(split.next())]
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

  #[rstest]
  #[case("asdf")]
  #[case("Информационные технологии, Иванов И.Л.")]
  fn incorrect_order(#[case] order: &str) {
    assert!(!is_correct_order(order))
  }
}
