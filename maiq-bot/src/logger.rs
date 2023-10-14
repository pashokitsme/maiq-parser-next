use std::io::Write;

use env_logger::fmt::Color;
use env_logger::Builder;
use env_logger::Target;

pub(crate) fn init_logger(is_test: bool) {
  Builder::new()
    .format_indent(Some(4))
    .is_test(is_test)
    .target(Target::Stdout)
    .format(|f, record| {
      let mut level_style = f.style();
      match record.level() {
        log::Level::Error => level_style.set_bold(true).set_color(Color::Red),
        log::Level::Warn => level_style.set_bold(true).set_color(Color::Yellow),
        log::Level::Info => level_style.set_color(Color::Green),
        log::Level::Debug => level_style.set_dimmed(true).set_color(Color::Blue),
        log::Level::Trace => level_style.set_dimmed(true).set_color(Color::Cyan),
      };

      let mut target_style = f.style();
      target_style.set_bold(true);

      let mut sep_style = f.style();
      sep_style.set_dimmed(true);

      write!(
        f,
        "{level:<6} {target} {sep} ",
        level = level_style.value(record.level()),
        target = target_style.value(record.target()),
        sep = sep_style.value("#")
      )?;

      writeln!(f, "{}", record.args())
    })
    .parse_filters(std::env::var("RUST_LOG").unwrap_or_default().as_str())
    .parse_write_style(std::env::var("RUST_LOG_STYLE").unwrap_or_default().as_str())
    .init();
}
