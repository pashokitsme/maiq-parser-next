#![allow(unused)]

use std::slice::Iter;

#[derive(Default, Debug)]
pub struct Snapshot<'s> {
  id: i32,
  date: (),
  groups: &'s [Group],
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

impl Id for Snapshot<'_> {
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

impl<'s> Snapshot<'s> {
  pub fn new(date: (), groups: &'s [Group]) -> Self {
    Self { date, groups, ..Default::default() }.compute_id()
  }

  pub fn group(&self, name: &str) -> Option<&Group> {
    self.groups.iter().find(|group| *group.name == *name)
  }

  pub fn groups(&self) -> Iter<Group> {
    self.groups.iter()
  }

  pub fn date(&self) {
    self.date
  }
}

impl Group {
  pub fn new(name: &str, lectures: Vec<Lecture>) -> Self {
    Self { name: Box::from(name), lectures, ..Default::default() }.compute_id()
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn set_lectures(&mut self, lectures: Vec<Lecture>) {
    self.lectures = lectures
  }
}

impl Lecture {
  pub fn new<S: AsRef<str> + Into<Box<str>>>(
    order: Option<S>,
    name: S,
    classroom: Option<S>,
    subgroup: Option<S>,
    teacher: Option<S>,
  ) -> Self {
    Self {
      id: 0,
      order: order.map(|o| o.into()),
      name: name.into(),
      classroom: classroom.map(|c| c.into()),
      subgroup: subgroup.map(|s| s.into()),
      teacher: teacher.map(|t| t.into()),
    }
    .compute_id()
  }
}
