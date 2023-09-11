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
  pub inner: Lecture,
}
