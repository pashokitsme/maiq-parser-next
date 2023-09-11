pub mod table;
pub mod default_lectures;

mod context;
mod parse_date;
mod periodical;

pub use context::*;
pub use periodical::*;

const GROUP_NAMES: [&str; 25] = [
  "Ит1-22", "Са1-21", "Са3-21", "С1-21", "С3-21", "Ир1-21", "Ир3-21", "Ир5-21", "С1-20", "С3-20", "Ип1-20", "Ип3-20", "Ир1-20",
  "Ир3-20", "Ир5-20", "Кс1-20", "Кс3-20", "Кс5-20", "С1-19", "С3-19", "С1-18", "С3-18", "ЗК1-22", "ЗК1-18", "ЗК1-19",
];
