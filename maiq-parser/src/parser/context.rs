use std::iter::Peekable;

use super::default_lectures::DefaultLectures;
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

pub struct ParserContext {
  is_week_even: bool,
  default_lectures: DefaultLectures,
  fallback_date: DateTime,
  group_names: Vec<Box<str>>,
}

impl ParserContext {
  pub fn new(is_week_even: bool, fallback_date: DateTime) -> Self {
    Self { is_week_even, default_lectures: DefaultLectures::default(), fallback_date, group_names: vec![] }
  }

  pub fn with_default_lectures(self, lectures: DefaultLectures) -> Self {
    Self { default_lectures: lectures, ..self }
  }

  pub fn with_groups<S: AsRef<str>, I: Iterator<Item = S>>(self, group_names: I) -> Self {
    let group_names = group_names
      .map(|name| name.as_ref().into())
      .collect::<Vec<Box<str>>>();
    Self { group_names, ..self }
  }

  pub fn parse(self, table: Table) -> Snapshot {
    let mut rows = table.rows.into_iter();
    let date = parse_date(&mut rows).unwrap_or(self.fallback_date);

    let raw_lectures = self.parse_raw_lectures(rows.skip(1).peekable());
    let mut groups = self.repair_and_assign(raw_lectures.into_iter());
    groups.retain(|g| g.has_lectures());
    Snapshot::new(date, groups)
  }

  fn parse_raw_lectures<S: AsRef<str>, I: Iterator<Item = Vec<S>> + Clone>(&self, rows: Peekable<I>) -> Vec<RawLecture> {
    let mut anchor = "Unknown".into();
    rows
      .map(|row| self.parse_raw_lecture(row.iter().peekable(), &mut anchor))
      .collect()
  }

  fn parse_raw_lecture<S: AsRef<str>, I: Iterator<Item = S> + Clone>(
    &self,
    mut row: Peekable<I>,
    anchor: &mut Box<str>,
  ) -> RawLecture {
    let ((group_name, subgroup), (order, lecture_name)) = match row.next() {
      Some(val) if self.is_group_name(val.as_ref()) => {
        let val = val.as_ref();
        if &**anchor != val {
          *anchor = Box::from(val);
        }

        (parse_group_subgroup_pair(val), parse_order_lecture_pair(row.next(), &mut row))
      }
      Some(val) => (parse_group_subgroup_pair(&anchor), parse_order_lecture_pair(Some(val), &mut row)),
      _ => return RawLecture::default(),
    };

    let (lecture_name, teacher) = split_teacher(lecture_name);

    let classroom = match row.next() {
      Some(x) if !x.as_ref().trim().is_empty() => Some(Box::from(x.as_ref().trim())),
      _ => None,
    };

    RawLecture { order: Some(order), group_name, subgroup, name: lecture_name, teacher, classroom }
  }

  fn repair_and_assign<I: Iterator<Item = RawLecture>>(self, lectures: I) -> Vec<Group> {
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
        let order = lecture.order.unwrap_or_default();
        let lectures = order.split(',').map(|x| Some(Box::from(x.trim()))).map(|order| {
          Lecture::new(
            order,
            lecture.name.clone().unwrap_or_default(),
            lecture.classroom.clone(),
            lecture.subgroup.clone(),
            lecture.teacher.clone(),
          )
        });

        group.unwrap().push_lectures(lectures);
      });
    groups
  }

  fn is_group_name(&self, name: &str) -> bool {
    let name = name.split(' ').next().unwrap_or_default();
    self.group_names.iter().any(|group| group.as_ref() == name)
  }
}

/// `(order?, lecture_name?)`
fn parse_order_lecture_pair<S: AsRef<str>, I: Iterator<Item = S>>(raw: Option<S>, row: &mut I) -> (Box<str>, Option<Box<str>>) {
  match raw {
    Some(val) if is_correct_order(&val) => (Box::from(val.as_ref()), row.next().map(|x| x.as_ref().into())),
    Some(val) => (Box::from(PREVIOUS_ORDER_PLACEHOLDER), Some(val.as_ref().into())),
    None => (Box::from(PREVIOUS_ORDER_PLACEHOLDER), None),
  }
}

fn is_correct_order<S: AsRef<str>>(raw: S) -> bool {
  const SKIP_CHARS: [char; 6] = ['(', ')', ',', '.', 'ч', ' '];
  raw
    .as_ref()
    .chars()
    .all(|c| SKIP_CHARS.contains(&c) || c.is_numeric())
}

/// `(lecture_name, teacher_name)`
fn split_teacher<S: AsRef<str>>(raw: Option<S>) -> (Option<Box<str>>, Option<Box<str>>) {
  let raw = match raw {
    Some(x) => x,
    None => return (None, None),
  };

  if let Some((name, teacher)) = raw.as_ref().rsplit_once(',') {
    return (Some(name.into()), empty_to_none!(Some(teacher.trim())));
  }
  (empty_to_none!(Some(raw.as_ref())), None)
}

/// `(group_name?, subgroup?)`
fn parse_group_subgroup_pair<S: AsRef<str>>(raw: S) -> (Option<Box<str>>, Option<Box<str>>) {
  let mut split = raw.as_ref().split(' ').map(|x| x.trim());
  (empty_to_none!(split.next()), empty_to_none!(split.next()))
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
  #[case("МДК.01.01 Разработка программных модулей, Пикселькина О.И.")]
  fn incorrect_order(#[case] order: &str) {
    assert!(!is_correct_order(order))
  }

  #[rstest]
  #[case("Ир3-21 2 п/г", (Some("Ир3-21".into()), Some("2".into())))]
  fn correct_splitting_group_name(#[case] name: &str, #[case] expect: (Option<Box<str>>, Option<Box<str>>)) {
    assert_eq!(parse_group_subgroup_pair(name), expect)
  }
}