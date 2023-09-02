use crate::utils::time::*;

const MONTHS: [&str; 12] =
  ["января", "февраля", "марта", "апреля", "мая", "июня", "июля", "августа", "сентября", "октября", "ноября", "декабря"];

pub fn parse_date<S: AsRef<str>, T: Iterator<Item = Vec<S>>>(row: &mut T) -> Option<DateTime> {
  let x = row.next().unwrap();
  let mut split = x.first().unwrap().as_ref().split(' ');

  while let Some(word) = split.next() {
    let day = match word.trim().parse::<u32>() {
      Ok(day) => day,
      _ => continue,
    };

    let month = match split.next() {
      Some(month) => MONTHS.iter().position(|&m| m == month).map(|x| x as u32 + 1),
      _ => continue,
    };

    let month = match month {
      Some(m) => m,
      _ => continue,
    };

    return DateTime::now().with_day(day).unwrap().with_month(month);
  }

  None
}
