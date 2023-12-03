use crate::schema;
use crate::utils::DateTime;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
  pub(crate) id: i64,
  pub(crate) cached_fullname: Option<String>,
  pub(crate) is_notifies_enabled: bool,
  pub(crate) is_broadcast_enabled: bool,
  pub(crate) created_at: NaiveDateTime,
  pub(crate) modified_at: NaiveDateTime,
}
#[derive(Queryable, Associations, Selectable, Debug)]
#[diesel(belongs_to(User, foreign_key = user_ref))]
#[diesel(belongs_to(Group, foreign_key = group_name_ref))]
#[diesel(table_name = schema::target_groups)]
pub struct TargetGroup {
  pub(crate) user_ref: i64,
  pub(crate) group_name_ref: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::groups)]
pub struct Group {
  pub(crate) id: i32,
  pub(crate) group_name: String,
}

// use crate::utils::*;
// use getset::*;

// #[derive(Getters, CopyGetters, MutGetters, Clone, Debug)]
// pub struct User {
//   #[getset(get_copy = "pub")]
//   pub(crate) chat_id: i64,

//   #[getset(skip)]
//   pub(crate) cached_fullname: Option<String>,

//   #[getset(get = "pub", get_mut = "pub")]
//   pub(crate) config: Config,

//   #[getset(get = "pub")]
//   pub(crate) created_at: DateTime,

//   #[getset(get = "pub")]
//   pub(crate) modified_at: DateTime,
// }

// impl Default for User {
//   fn default() -> Self {
//     Self {
//       chat_id: Default::default(),
//       cached_fullname: Default::default(),
//       config: Default::default(),
//       created_at: DateTime::now(),
//       modified_at: DateTime::now(),
//     }
//   }
// }

// impl User {
//   pub fn new(chat_id: i64) -> Self {
//     let mut user = Self { chat_id, ..Default::default() };
//     user.config.chat_id = chat_id;
//     user
//   }

//   pub fn cached_fullname(&self) -> Option<&str> {
//     self.cached_fullname.as_deref()
//   }
// }

// #[derive(CopyGetters, Getters, Setters, Default, Clone, Debug)]
// pub struct Config {
//   pub(crate) chat_id: i64,

//   #[getset(get_copy = "pub", set = "pub")]
//   pub(crate) is_notifies_enabled: bool,

//   #[getset(get_copy = "pub", set = "pub")]
//   pub(crate) is_broadcast_enabled: bool,

//   pub(crate) target_groups: Vec<String>,
// }

// impl Config {
//   pub fn groups(&self) -> &[String] {
//     &self.target_groups
//   }

//   pub fn has_group<S: AsRef<str>>(&self, name: S) -> bool {
//     self.groups().iter().any(|g| g == name.as_ref())
//   }

// }
