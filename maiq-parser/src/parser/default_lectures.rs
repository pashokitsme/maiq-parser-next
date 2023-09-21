use std::ops::Deref;
use std::slice::Iter;

use serde::Deserialize;
use serde::Serialize;

use crate::snapshot::Lecture;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DefaultLectures(Vec<DefaultGroup>);

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Debug)]
pub enum LectureWeek {
  Even,
  Odd,
  #[default]
  Every,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DefaultGroup {
  pub name: String,
  pub lectures: Vec<DefaultLecture>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DefaultLecture {
  pub week: LectureWeek,
  #[serde(flatten)]
  inner: Lecture,
}

impl DefaultLectures {
  pub fn group<S: AsRef<str>>(&self, name: S) -> Option<Iter<DefaultLecture>> {
    let name = name.as_ref();
    self
      .0
      .iter()
      .find(|group| group.name == name)
      .map(|group| group.lectures.iter())
  }
}

impl Deref for DefaultLecture {
  type Target = Lecture;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}
