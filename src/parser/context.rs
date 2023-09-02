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
struct RawLecture<'l> {
  order: Option<&'l str>,
  group_name: Option<&'l str>,
  subgroup: Option<&'l str>,
  name: Option<&'l str>,
  teacher: Option<&'l str>,
  classroom: Option<&'l str>,
}

pub struct ParserContext {
  is_week_even: bool,
  default_lectures: (),
  fallback_date: DateTime,
  groups: Vec<Group>,
}

impl<'c> ParserContext {
  pub fn with_groups<S: AsRef<str>, I: Iterator<Item = S>>(self, group_names: I) -> Self {
    let groups = group_names
      .map(|name| Group::new(name.as_ref(), vec![]))
      .collect::<Vec<Group>>();
    Self { groups, ..self }
  }

  pub fn parse(mut self, table: Table) {
    let mut rows = table.rows.into_iter();
    let date = parse_date(&mut rows).unwrap_or(self.fallback_date);
    let mut group_cursor: Option<String> = None;
    let is_name_valid = |name: &str| {
      let name = name.split(' ').next().unwrap_or_default();
      self.groups.iter().any(|g| g.name() == name)
    };

    let mut raw_lectures = self.parse_raw_lectures();
    self.repair_and_assign(raw_lectures.into_iter());
    // replace_all_default(&mut groups, date);
    // groups.retain(|g| !g.lessons.is_empty());
    // groups.iter_mut().for_each(|g| {
    //   g.lessons.sort_by_key(|g| g.subgroup);
    //   g.lessons.sort_by(|a, b| a.num.cmp(&b.num));
    // });
  }

  fn parse_raw_lectures(&mut self) -> Vec<RawLecture<'c>> {
    let mut anchor = "Unknown group";
    vec![RawLecture::default(); 1]
  }

  fn parse_raw_lecture<S: AsRef<str>, I: Iterator<Item = S>>(&self, mut row: I, anchor: &mut Box<str>) -> RawLecture {
    let ([group_name, subgroup], num) = {
      match row.next() {
        Some(cell) if self.is_group_name(cell.as_ref()) => {
          if &**anchor != cell.as_ref() {
            *anchor = Box::from(cell.as_ref());
          }
          (split_group_name(Some(cell)), parse_num(&mut row))
        }
        Some(cell) => (split_group_name(Some(&anchor)), Some(Box::from(cell.as_ref()))),
        _ => return RawLecture::default(),
      }
    };

    let [name, teacher] = split_teacher(row.next());
    let classroom = match row.next() {
      Some(x) if !x.as_ref().is_empty() => Some(x),
      _ => None,
    };

    RawLecture::default()
  }

  fn repair_and_assign<I: Iterator<Item = RawLecture<'c>>>(&mut self, lectures: I) {
    let mut prev: Option<RawLecture> = None;

    for lecture in lectures
      .map(|mut lecture| {
        if matches!(lecture.order, Some(PREVIOS_ORDER_PLACEHOLDER)) {
          lecture.order = prev.as_ref().and_then(|p| p.order)
        }
        prev = Some(lecture.clone());
        lecture
      })
      .filter(|l| l.group_name.is_some() && !matches!(l.name, None | Some("Нет") | Some("нет")))
    {
      let group_name = lecture.group_name.unwrap();
      let group = self.groups.iter_mut().find(|x| x.name() == group_name).unwrap();
      let lectures = lecture
        .order
        .unwrap_or_default()
        .split(',')
        .map(|x| Some(x.trim()))
        .map(|order| Lecture::new(order, lecture.name.unwrap_or_default(), lecture.classroom, lecture.subgroup, lecture.teacher))
        .collect::<Vec<Lecture>>();

      group.set_lectures(lectures);
    }
  }

  fn is_group_name(&self, name: &str) -> bool {
    self.groups.iter().any(|group| group.name() == name)
  }
}

fn parse_num<'o, S: AsRef<str>, I: Iterator<Item = S>>(row: &mut I) -> Option<Box<str>> {
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

// #[derive(Debug, Default)]
// struct RawLesson {
//   num: Num,
//   group_name: Option<String>,
//   subgroup: Option<String>,
//   name: Option<String>,
//   teacher: Option<String>,
//   classroom: Option<String>,
// }

// pub fn parse_snapshot(table: Table, fallback_date: DateTime<Utc>) -> anyhow::Result<Snapshot> {
//   let mut rows = table.rows.into_iter();
//   let date = date::parse_date(&mut rows).unwrap_or(fallback_date);
//   let mut groups = make_groups();
//   let mut group_cursor: GroupCursor = None;
//   let is_name_valid = |name: &str| {
//     let name = name.split(' ').next().unwrap_or_default();
//     groups.iter().any(|g| g.name == name)
//   };

//   let mut lessons = rows
//     .map(|vec| parse_row(&mut vec.iter().peekable(), &mut group_cursor, is_name_valid))
//     .collect::<Vec<RawLesson>>();
//   repair_nums(&mut lessons);
//   assign_lessons_to_groups(lessons, &mut groups);
//   replace_all_default(&mut groups, date);
//   groups.retain(|g| !g.lessons.is_empty());
//   groups.iter_mut().for_each(|g| {
//     g.lessons.sort_by_key(|g| g.subgroup);
//     g.lessons.sort_by(|a, b| a.num.cmp(&b.num));
//   });

//   Ok(Snapshot::new(groups, date))
// }

// fn assign_lessons_to_groups(lessons: Vec<RawLesson>, groups: &mut [Group]) {
// }

// fn parse_row<'a, F>(row: &mut Peekable<Iter<String>>, group_cursor: &mut GroupCursor, is_name_valid: F) -> RawLesson
// where
//   F: Fn(&str) -> bool + 'a,
// {
//   let ([group_name, subgroup], num) = {
//     match row.next() {
//       Some(x) if is_name_valid(x) => {
//         if !matches!(group_cursor, Some(ref c) if *c == *x) {
//           *group_cursor = Some(x.clone());
//         }
//         (split_group_name(Some(x)), parse_num(row))
//       }
//       Some(x) => (split_group_name(group_cursor.as_deref()), Num::Actual(x.clone())),
//       _ => return RawLesson::default(),
//     }
//   };

//   let [name, teacher] = split_teacher(row.next().map(|x| &**x));
//   let classroom = empty_to_none!(row.next());

//   RawLesson { num, group_name, subgroup, name, teacher, classroom: classroom.cloned() }
// }

// #[cfg(test)]
// #[test]
// fn __test_is_num() {
//   assert!(is_num("1,2,3"));
//   assert!(is_num("1,2,3(1ч)"));
//   assert!(!is_num("Информационные технологии, Иванов И.Л."));
//   assert!(is_num(""));
// }

// fn expand_num(num: Num) -> Vec<Num> {
//   match num {
//     Num::Actual(x) => x
//       .split(',')
//       .map(|x| Num::Actual(x.trim().to_string()))
//       .collect::<Vec<Num>>(),
//     _ => vec![Num::None],
//   }
// }

// fn make_groups() -> Vec<Group> {
//   let names: Vec<String> = env::groups().into();
//   let mut groups = Vec::with_capacity(names.len());
//   for name in names.into_iter() {
//     groups.push(Group::new(name));
//   }

//   groups
// }
