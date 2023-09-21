use serde::Deserialize;
use serde::Serialize;

use crate::utils::time::*;

use std::slice::Iter;

pub use cmp::*;

// todo: избавиться от Box<str>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Snapshot {
  #[serde(default)]
  id: i32,
  date: DateTime,
  groups: Vec<Group>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Group {
  #[serde(default)]
  id: i32,
  name: Box<str>,
  lectures: Vec<Lecture>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lecture {
  #[serde(default)]
  id: i32,
  order: Option<Box<str>>,
  name: Box<str>,
  classroom: Option<Box<str>>,
  subgroup: Option<Box<str>>,
  teacher: Option<Box<str>>,
}

pub trait Id {
  fn compute_id(self) -> Self;
  fn id(&self) -> i32;
}

impl Id for Snapshot {
  fn compute_id(self) -> Self {
    let id = 1;
    Self { id, ..self }
  }

  fn id(&self) -> i32 {
    self.id
  }
}

impl Id for Group {
  fn compute_id(self) -> Self {
    let id = 2;
    Self { id, ..self }
  }

  fn id(&self) -> i32 {
    self.id
  }
}

impl Id for Lecture {
  fn compute_id(self) -> Self {
    let id = 3;
    Self { id, ..self }
  }

  fn id(&self) -> i32 {
    self.id
  }
}

impl Snapshot {
  pub fn new(date: DateTime, groups: Vec<Group>) -> Self {
    Self { id: 0, date, groups }.sort_groups().compute_id()
  }

  pub fn group(&self, name: &str) -> Option<&Group> {
    self.groups.iter().find(|group| *group.name == *name)
  }

  pub fn groups(&self) -> Iter<Group> {
    self.groups.iter()
  }

  pub fn date(&self) -> DateTime {
    self.date
  }

  fn sort_groups(mut self) -> Self {
    self.groups.iter_mut().for_each(|g| {
      g.lectures.sort_by_key(|g| g.subgroup.clone());
      g.lectures.sort_by(|a, b| a.order.cmp(&b.order));
    });
    self
  }
}

impl Group {
  pub fn new(name: &str, lectures: Vec<Lecture>) -> Self {
    Self { name: Box::from(name), lectures, ..Default::default() }.compute_id()
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn lectures(&self) -> Iter<Lecture> {
    self.lectures.iter()
  }

  pub fn has_lectures(&self) -> bool {
    !self.lectures.is_empty()
  }

  pub fn push_lectures<I: Iterator<Item = Lecture>>(&mut self, lectures: I) {
    self.lectures.extend(lectures)
  }
}

impl Lecture {
  pub fn new(
    order: Option<Box<str>>,
    name: Box<str>,
    classroom: Option<Box<str>>,
    subgroup: Option<Box<str>>,
    teacher: Option<Box<str>>,
  ) -> Self {
    Self { id: 0, order, name, classroom, subgroup, teacher }.compute_id()
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn order(&self) -> Option<&str> {
    self.order.as_deref()
  }

  pub fn subgroup(&self) -> Option<&str> {
    self.subgroup.as_deref()
  }

  pub fn teacher(&self) -> Option<&str> {
    self.teacher.as_deref()
  }

  pub fn classroom(&self) -> Option<&str> {
    self.classroom.as_deref()
  }
}

pub mod cmp {
  use crate::snapshot::*;

  pub trait Compare {
    fn distinct(&self, rhs: Self, placeholder: &[&str]) -> Vec<String>;
  }

  impl Compare for Option<&Snapshot> {
    fn distinct(&self, rhs: Self, defs: &[&str]) -> Vec<String> {
      let (lhs, rhs) = match (self, rhs) {
        (Some(lhs), Some(rhs)) if lhs.id() == rhs.id() => return vec![],
        (Some(lhs), Some(rhs)) => (lhs, rhs),
        (Some(_), None) => return vec![],
        (None, Some(_)) => return defs.iter().map(|x| x.to_string()).collect(),
        (None, None) => return vec![],
      };

      let is_updated = |name: &String| -> bool {
        let lhs = lhs.group(name);
        let rhs = rhs.group(name);

        match (lhs, rhs) {
          (None, Some(_)) => true,
          (Some(_), None) => true,
          (Some(lhs), Some(rhs)) if lhs.id() != rhs.id() => true,
          _ => false,
        }
      };

      let mut changes = defs.iter().map(|x| x.to_string()).collect::<Vec<_>>();
      changes.retain(is_updated);
      changes
    }
  }

  #[cfg(test)]
  mod tests {
    use crate::snapshot::cmp::Compare;
    use crate::snapshot::*;

    #[fixture]
    fn snapshot_1() -> Snapshot {
      let groups =
        vec![Group::new("Group1", vec![Lecture::new(Some("1".into()), "Lecture1".into(), Some("1E".into()), None, None)])];
      Snapshot::new(DateTime::now(), groups)
    }

    #[fixture]
    fn snapshot_2() -> Snapshot {
      let groups =
        vec![Group::new("Group1", vec![Lecture::new(Some("1".into()), "Lecture2".into(), Some("1E".into()), None, None)])];
      Snapshot::new(DateTime::now(), groups)
    }

    #[rstest]
    fn diff_lectures(#[from(snapshot_1)] s1: Snapshot, #[from(snapshot_2)] s2: Snapshot) {
      assert_eq!(vec!["Group1".to_string()], Some(&s1).distinct(Some(&s2), &["Group1"]))
    }
  }
}
