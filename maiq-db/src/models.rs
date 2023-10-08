use std::slice::Iter;

use crate::utils::*;
use getset::*;

#[derive(Getters, CopyGetters, MutGetters, Clone, Debug)]
pub struct User {
  #[getset(get_copy = "pub")]
  pub(crate) id: u64,

  #[getset(skip)]
  pub(crate) cached_fullname: Option<String>,

  #[getset(get = "pub", get_mut = "pub")]
  pub(crate) config: Config,

  pub(crate) created_at: DateTime,
  pub(crate) modified_at: DateTime,
}

impl Default for User {
  fn default() -> Self {
    Self {
      id: Default::default(),
      cached_fullname: Default::default(),
      config: Default::default(),
      created_at: DateTime::now(),
      modified_at: DateTime::now(),
    }
  }
}

impl User {
  pub fn cached_fullname(&self) -> Option<&str> {
    self.cached_fullname.as_deref()
  }
}

#[derive(CopyGetters, Getters, Setters, Default, Clone, Debug)]
pub struct Config {
  #[getset(get_copy = "pub", set = "pub")]
  pub(crate) is_notifies_enabled: bool,

  #[getset(get_copy = "pub", set = "pub")]
  pub(crate) is_broadcast_enabled: bool,

  pub(crate) target_groups: Vec<String>,
}

impl Config {
  pub fn add_group<S: AsRef<str>>(&mut self, name: S) {
    if !self.target_groups.iter().any(|n| n == name.as_ref()) {
      self.target_groups.push(name.as_ref().into());
    }
  }

  pub fn remove_group<S: AsRef<str>>(&mut self, name: S) {
    self.target_groups.retain(|n| n != name.as_ref());
  }

  pub fn target_groups(&self) -> Iter<String> {
    self.target_groups.iter()
  }
}
