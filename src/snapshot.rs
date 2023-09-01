#![allow(unused)]

use std::slice::Iter;

#[derive(Default, Debug)]
pub struct Snapshot<'s> {
  id: i32,
  date: (),
  groups: &'s [Group<'s>],
}

#[derive(Default, Debug)]
pub struct Group<'g> {
  id: i32,
  name: &'g str,
  lectures: &'g [Lecture<'g>],
}

#[derive(Debug)]
pub struct Lecture<'l> {
  id: i32,
  order: Option<&'l str>,
  name: &'l str,
  classroom: Option<&'l str>,
  subgroup: Option<i32>,
  teacher: Option<&'l str>,
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

impl Id for Group<'_> {
  fn compute_id(self) -> Self {
    let id = 2;
    Self { id, ..self }
  }

  fn id(&self) -> i32 {
    self.id
  }
}

impl Id for Lecture<'_> {
  fn compute_id(self) -> Self {
    let id = 3;
    Self { id, ..self }
  }

  fn id(&self) -> i32 {
    self.id
  }
}

impl<'s> Snapshot<'s> {
  pub fn new(date: (), groups: &'s [Group<'s>]) -> Self {
    Self { date, groups, ..Default::default() }.compute_id()
  }

  pub fn group(&self, name: &str) -> Option<&Group> {
    self.groups.iter().find(|group| group.name == name)
  }

  pub fn groups(&self) -> Iter<Group> {
    self.groups.iter()
  }

  pub fn date(&self) {
    self.date
  }
}

impl<'g> Group<'g> {
  pub fn new(name: &'g str, lectures: &'g [Lecture<'g>]) -> Self {
    Self { name, lectures, ..Default::default() }.compute_id()
  }
}
