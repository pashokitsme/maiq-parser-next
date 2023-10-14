use std::slice::Iter;

use crate::utils::*;
use getset::*;

#[derive(Getters, CopyGetters, MutGetters, Clone, Debug)]
pub struct User {
  #[getset(get_copy = "pub")]
  pub(crate) chat_id: i64,

  #[getset(skip)]
  pub(crate) cached_fullname: Option<String>,

  #[getset(get = "pub", get_mut = "pub")]
  pub(crate) config: Config,

  #[getset(get = "pub")]
  pub(crate) created_at: DateTime,

  #[getset(get = "pub")]
  pub(crate) modified_at: DateTime,
}

impl Default for User {
  fn default() -> Self {
    Self {
      chat_id: Default::default(),
      cached_fullname: Default::default(),
      config: Default::default(),
      created_at: DateTime::now(),
      modified_at: DateTime::now(),
    }
  }
}

impl User {
  pub fn new(chat_id: i64) -> Self {
    Self { chat_id, ..Default::default() }
  }

  pub fn cached_fullname(&self) -> Option<&str> {
    self.cached_fullname.as_deref()
  }
}

#[derive(CopyGetters, Getters, Setters, Default, Clone, Debug)]
pub struct Config {
  pub(crate) chat_id: i64,

  #[getset(get_copy = "pub")]
  pub(crate) is_notifies_enabled: bool,

  #[getset(get_copy = "pub")]
  pub(crate) is_broadcast_enabled: bool,

  pub(crate) target_groups: Vec<String>,
}

impl Config {
  pub fn groups(&self) -> Iter<String> {
    self.target_groups.iter()
  }
}
