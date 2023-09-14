pub mod default_lectures;
pub mod table;

mod context;
mod parse_date;
mod periodical;

pub use context::*;
pub use periodical::*;

const GROUP_NAMES: [&str; 4] = ["Ир1-22", "Ир3-21", "Ип1-21", "Ип5-21"];
