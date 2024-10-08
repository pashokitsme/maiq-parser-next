use crate::parser::SnapshotParserAgent;

use crate::parser::default_lectures::*;
use crate::parser::parse_date::*;
use crate::parser::table::*;
use crate::parser::GROUP_NAMES;

use crate::snapshot::*;
use crate::utils::time::*;

const PREVIOUS_ORDER_PLACEHOLDER: &str = "-1";

#[derive(Clone, Default, PartialEq, Debug)]
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
    let mut rows = table.rows.into_iter().skip(1);
    let date = parse_date(&mut rows).unwrap_or(self.fallback_date);
    let is_week_even = date.iso_week().week0() % 2 == 0;

    let raw_lectures = self.parse_raw_lectures(rows);
    let groups = self.assign_to_groups(raw_lectures.into_iter(), is_week_even);
    Snapshot::new(date, groups)
  }
}

impl SnapshotParser4 {
  fn parse_raw_lectures<I: Iterator<Item = Vec<String>>>(&self, rows: I) -> Vec<RawLecture> {
    let mut left = vec![];
    let mut right = vec![];

    for row in rows.into_iter().skip(1) {
      let mut chunks = row.chunks(3);
      if let Some(left_row) = chunks.next() {
        left.push(left_row.to_vec());
      }
      if let Some(right_row) = chunks.next() {
        right.push(right_row.to_vec());
      }
    }

    left
      .into_iter()
      .chain(right)
      .filter_map(RawLecture::parse_from_row)
      .collect()
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
        if lecture.group_name.is_none() {
          lecture.group_name = prev.as_ref().and_then(|p| p.group_name.clone());
        }

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
        if let Some(group) = group {
          let lectures = self.expand_raw_lecture(lecture, is_week_even);
          group.push_lectures(lectures.into_iter());
        }
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

impl RawLecture {
  pub fn parse_from_row(row: Vec<String>) -> Option<Self> {
    let mut row = row.into_iter();
    let group_name = row
      .next()
      .take_if(|name| !name.is_empty() && GROUP_NAMES.contains(&name.as_str()));
    let order = row.next().take_if(|order| !order.is_empty())?;

    let subgroup_name_teacher_classroom = row.next().take_if(|s| !s.is_empty())?;

    let (subgroup, name_teacher_classroom) = subgroup_name_teacher_classroom
      .split_once("п/г")
      .map(|(subgroup, rest)| (subgroup.trim(), rest.trim().trim_start_matches(',')))
      .unwrap_or(("", subgroup_name_teacher_classroom.as_str()));

    let subgroup = Some(subgroup).take_if(|s| !s.is_empty());

    let mut name_teacher_classroom = name_teacher_classroom.split(',');
    let lecture_name = name_teacher_classroom.next()?.trim();
    let teacher = name_teacher_classroom.next().map(|s| s.trim());
    let classroom = name_teacher_classroom.next().map(|s| s.trim());

    Some(Self {
      order: Some(order.into()),
      group_name: group_name.map(Box::from),
      name: Some(lecture_name.into()),
      teacher: teacher.map(Box::from),
      classroom: classroom.map(Box::from),
      subgroup: subgroup.map(Box::from),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[rstest]
  #[case(
  vec!["".to_string(), "2".to_string(), "По расписанию".to_string()], 
  Some(RawLecture {
    order: Some("2".into()),
    group_name: None,
    subgroup: None,
    name: Some("По расписанию".into()),
    teacher: None,
    classroom: None
  }))]
  #[case(
    vec!["Ир5-21".to_string(), "1".to_string(), "МДК 08.02, Маркова М.А., 211М".to_string()], 
    Some(RawLecture {
    order: Some("1".into()),
    group_name: Some("Ир5-21".into()),
    subgroup: None,
    name: Some("МДК 08.02".into()),
    teacher: Some("Маркова М.А.".into()),
    classroom: Some("211М".into())
  }))]
  #[case(
    vec!["Ир5-21".to_string(), "1".to_string(), "1п/г МДК 08.02, Маркова М.А., 211М".to_string()], 
    Some(RawLecture {
    order: Some("1".into()),
    group_name: Some("Ир5-21".into()),
    subgroup: Some("1".into()),
    name: Some("МДК 08.02".into()),
    teacher: Some("Маркова М.А.".into()),
    classroom: Some("211М".into())
  }))]
  #[case(
    vec!["Ир5-21".to_string(), "1".to_string(), "2 п/г МДК 08.02, Маркова М.А., 211М".to_string()], 
    Some(RawLecture {
    order: Some("1".into()),
    group_name: Some("Ир5-21".into()),
    subgroup: Some("2".into()),
    name: Some("МДК 08.02".into()),
    teacher: Some("Маркова М.А.".into()),
    classroom: Some("211М".into())
  }))]
  #[case(
    vec!["".to_string(), "1".to_string(), "2 п/г МДК 08.02, Маркова М.А., 211М".to_string()], 
    Some(RawLecture {
    order: Some("1".into()),
    group_name: None,
    subgroup: Some("2".into()),
    name: Some("МДК 08.02".into()),
    teacher: Some("Маркова М.А.".into()),
    classroom: Some("211М".into())
  }))]
  #[case(
    vec!["".to_string(), "1".to_string(), "2 п/г МДК 08.02, Маркова М.А.".to_string()], 
    Some(RawLecture {
    order: Some("1".into()),
    group_name: None,
    subgroup: Some("2".into()),
    name: Some("МДК 08.02".into()),
    teacher: Some("Маркова М.А.".into()),
    classroom: None
  }))]
  #[case(
    vec!["q234".to_string(), "as".to_string()],
    None
  )]
  fn parse_raw_lecture(#[case] input: Vec<String>, #[case] expected: Option<RawLecture>) {
    assert_eq!(expected, RawLecture::parse_from_row(input))
  }
}
