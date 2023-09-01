use tl::*;

use std::borrow::Cow;

use aho_corasick::AhoCorasick;

macro_rules! impl_parse_exact {
  ($name: ident, $select: ident) => {
    #[allow(unused_mut)]
    pub fn $name(html: &str) -> Option<Table> {
      let dom = tl::parse(html, ParserOptions::default()).ok()?;
      let parser = dom.parser();
      let table = dom.query_selector("table").and_then(|mut table| table.$select())?;
      let values = parse_table(table.get(parser).unwrap().inner_html(parser))?;
      if values.iter().all(|col| col.is_empty()) {
        return None;
      }
      Some(Table { rows: values })
    }
  };
}

#[derive(Debug, PartialEq)]
pub struct Table {
  pub rows: Vec<Vec<String>>,
}

pub fn parse_all(html: &str) -> Option<Vec<Table>> {
  let dom = tl::parse(html, ParserOptions::default()).ok()?;
  let parser = dom.parser();
  let tables = dom.query_selector("table")?;

  let values = tables
    .filter_map(|table| parse_table(table.get(parser).unwrap().inner_html(parser)))
    .map(|rows| Table { rows })
    .collect::<Vec<Table>>();
  if values.is_empty() || values.iter().all(|table| table.rows.iter().all(|col| col.is_empty())) {
    return None;
  }
  Some(values)
}

impl_parse_exact!(parse_first, next);
impl_parse_exact!(parse_last, last);

fn parse_table(html: Cow<str>) -> Option<Vec<Vec<String>>> {
  let dom = tl::parse(&html, ParserOptions::default()).ok()?;
  let parser = dom.parser();
  let trs = dom.query_selector("tr")?;

  let table = trs
    .map(|tr| tr.get(parser).unwrap())
    .map(|tr| {
      tr.children()
        .unwrap()
        .top()
        .iter()
        .filter_map(|handle| get_inner_text(parser, handle))
        .map(normalize)
        .collect::<Vec<String>>()
    })
    .filter(|x| !x.iter().all(|x| x.is_empty()))
    .collect::<Vec<Vec<String>>>();

  Some(table)
}

fn get_inner_text(parser: &Parser, node: &NodeHandle) -> Option<String> {
  let res = node.get(parser)?.inner_text(parser);
  let res = res.trim();
  match res.len() {
    0 => None,
    _ => Some(res.into()),
  }
}

const PATTERNS: [&str; 14] = [
  "&lt;", "&gt;", "&amp;", "&nbsp;", "&ensp;", "&emsp;", "&copy;", "&mdash;", "&ndash;", "&shy;", "&laquo;", "&raquo;",
  "&hellip;", "&sect;",
];

const REPLACE: [&str; 14] = ["<", ">", "&", " ", " ", " ", "©", "—", "–", " ", "«", "»", "...", "§"];

fn normalize(text: String) -> String {
  let corasik = AhoCorasick::new(PATTERNS).unwrap();
  let text = corasik.replace_all(&text, &REPLACE);
  let mut chars = text.chars().peekable();
  let mut whitespaces_only = true;
  let mut res = String::new();

  while let Some(c) = chars.next() {
    let next = chars.peek();
    if whitespaces_only && !c.is_whitespace() {
      whitespaces_only = false;
    }

    if c.is_whitespace() && ((next.is_some() && next.unwrap().is_whitespace()) || next.is_none()) {
      continue;
    }

    match c {
      '\n' => (),
      c if c.is_whitespace() => res.push(' '),
      c => res.push(c),
    }
  }

  res
}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! table {
    [$(($($v: literal),*)),*] => {
      Some(Table { rows: vec![$(vec![$($v.to_string(),)*],)*] })
    };
    [$([$(($($v: literal),*)),*]),*] => {
      Some(vec![$(Table { rows: vec![$(vec![$($v.to_string(),)*],)*] },)*])
    };
  }

  const ONE_TABLE_TWO_ROWS: &str = r#"
    <table>
        <tr><th>Header</th><th>Value</th></tr>
        <tr><td>A</td><td>B</td></tr>
    </table>"#;

  const SPECIAL_SYMBOL: &str = r#"<table>
        <tr><th>A</th><th>&lt;</th></tr>
    </table>"#;

  const TWO_TABLES: &str = r#"<table>
        <tr><th>Header</th><th>Value</th></tr>
        <tr><td>A</td><td>B</td></tr>
    </table>
    <table>
        <tr><th>Header</th><th>Value</th></tr>
        <tr><td>A</td><td>B</td></tr>
    </table>"#;

  #[rstest]
  #[case(ONE_TABLE_TWO_ROWS, table![("Header", "Value"), ("A", "B")])]
  #[case(SPECIAL_SYMBOL, table![("A", "<")])]
  #[case("<table>Hey</table>", None)]
  #[case("<div>Hi</div>", None)]
  fn simple(#[case] html: &str, #[case] expected: Option<Table>) {
    assert_eq!(expected, parse_first(html));
  }

  #[rstest]
  #[case(TWO_TABLES, table![[("Header", "Value"), ("A", "B")], [("Header", "Value"), ("A", "B")]])]
  #[case("<div>Hi</div>", None)]
  fn multiple(#[case] html: &str, #[case] expected: Option<Vec<Table>>) {
    assert_eq!(expected, parse_all(html));
  }

  #[rstest]
  #[case("&lt;", "<")]
  #[case("&nbsp;  af&lt;", " af<")]
  #[case("f&nbsp;f", "f f")]
  #[case("f  f", "f f")]
  fn special_symbols(#[case] input: String, #[case] expected: &str) {
    assert_eq!(normalize(input), expected);
  }
}
