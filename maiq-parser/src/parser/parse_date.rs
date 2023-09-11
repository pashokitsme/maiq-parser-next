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

    if let Some(month) = split
      .next()
      .and_then(|month| MONTHS.iter().position(|&m| m == month).map(|x| x as u32 + 1))
    {
      return DateTime::now()
        .with_day(day)
        .and_then(|date| date.with_month(month).unwrap().with_nanosecond(0));
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use crate::parser::parse_date::*;

  #[rstest]
  #[case(vec![vec!["5 июня".into()]], (5, 6))]
  #[case(vec![vec!["Стваыф 5 июля авыфавыф".into()]], (5, 7))]
  #[case(vec![vec!["АВыфавыф 24 февраля fdjska sadf".into()]], (24, 2))]
  fn simple(#[case] raw: Vec<Vec<String>>, #[case] expect: (u32, u32)) {
    let expect = DateTime::now()
      .with_day(expect.0)
      .unwrap()
      .with_month(expect.1)
      .unwrap()
      .with_nanosecond(0)
      .unwrap();
    assert_eq!(parse_date(&mut raw.into_iter()), Some(expect));
  }

  #[rstest]
  #[case(vec![vec!["FDasfdsa fdsa sadf".into()]])]
  #[case(vec![vec!["41234 января".into()]])]
  fn invalid(#[case] raw: Vec<Vec<String>>) {
    assert_eq!(parse_date(&mut raw.into_iter()), None)
  }
}
