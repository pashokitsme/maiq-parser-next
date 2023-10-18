use std::sync::OnceLock;
use std::time::Duration;

use maiq_parser_next::utils::time::*;

build_info::build_info!(fn info);

pub fn build_info() -> &'static str {
  static INIT: OnceLock<String> = OnceLock::new();
  INIT.get_or_init(|| {
    let info = info();

    let branch = std::env::var("RAILWAY_GIT_BRANCH").unwrap_or_default();
    let commit = std::env::var("RAILWAY_GIT_COMMIT_SHA").unwrap_or_default();

    let ver = crate::reply!(
      "build_info.md",
      name = info.crate_info.name,
      version = info.crate_info.version,
      profile = info.profile,
      triple = info.target.triple,
      rustc_version = info.compiler.version,
      rustc_triple = info.compiler.host_triple,
      o = info.optimization_level,
      branch = branch,
      commit = commit.split_at(7).0,
      deployed_time = info
        .timestamp
        .with_timezone(&FixedOffset::east_opt(3 * 3600).unwrap())
        .format("%d.%m.%Y %H:%m:%S")
    );
    ver
  })
}
