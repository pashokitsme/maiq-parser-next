use crate::utils::time::*;

use std::slice::Iter;

#[derive(Debug)]
pub struct Snapshot {
  id: i32,
  date: DateTime,
  groups: Vec<Group>,
}

#[derive(Default, Debug)]
pub struct Group {
  id: i32,
  name: Box<str>,
  lectures: Vec<Lecture>,
}

#[derive(Debug)]
pub struct Lecture {
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

  pub fn has_lectures(&self) -> bool {
    !self.lectures.is_empty()
  }

  pub fn set_lectures(&mut self, lectures: Vec<Lecture>) {
    self.lectures = lectures
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
}
