use std::sync::OnceLock;

use maiq_parser_next::utils::time::*;

build_info::build_info!(fn info);

pub fn build_info() -> &'static str {
  static INIT: OnceLock<String> = OnceLock::new();
  INIT.get_or_init(|| {
    let info = info();

    let branch = std::env::var("RAILWAY_GIT_BRANCH");
    let commit = std::env::var("RAILWAY_GIT_COMMIT_SHA");

    let ver = crate::reply!(
      "build_info.md",
      name = info.crate_info.name,
      version = info.crate_info.version,
      profile = info.profile,
      rustc_version = info.compiler.version,
      rustc_triple = info.compiler.host_triple,
      o = info.optimization_level,
      branch = branch.as_deref().unwrap_or("unknown"),
      commit = commit.as_deref().unwrap_or("unknown").split_at(7).0,
      deployed_time = info
        .timestamp
        .with_timezone(&FixedOffset::east_opt(3 * 3600).unwrap())
        .format("%d.%m.%Y %H:%m:%S")
    );
    ver
  })
}
