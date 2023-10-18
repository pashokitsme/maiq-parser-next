use std::sync::OnceLock;
use std::time::Duration;

use build_info::chrono::FixedOffset;

build_info::build_info!(fn info);

pub fn build_info() -> &'static str {
  static INIT: OnceLock<String> = OnceLock::new();
  INIT.get_or_init(|| {
    let info = info();

    let git = info.version_control.as_ref().unwrap().git();
    let git = git.as_ref().unwrap();

    let offset = FixedOffset::east_opt(3 * 3600).unwrap();
    let took = info.timestamp.with_timezone(&offset).timestamp() - git.commit_timestamp.with_timezone(&offset).timestamp();
    let took = Duration::from_secs(took as u64);
    let took = pretty_duration::pretty_duration(&took, None);

    let ver = crate::reply!(
      "build_info.md",
      name = info.crate_info.name,
      version = info.crate_info.version,
      profile = info.profile,
      triple = info.target.triple,
      rustc_version = info.compiler.version,
      rustc_triple = info.compiler.host_triple,
      o = info.optimization_level,
      branch = git.branch.as_ref().unwrap(),
      commit = git.commit_short_id,
      deployed_time = info.timestamp.format("%d.%m.%Y %H:%m:%S"),
      deploy_time_took = took
    );
    ver
  })
}
