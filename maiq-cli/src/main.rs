use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
  let (tx, mut rx) = mpsc::channel(32);

  let parser = maiq_parser_next::parser::PeriodicalParserBuilder::new()
    .add_url("https://rsp.chemk.org/4korp/today.htm")
    .unwrap()
    .on_update(tx)
    .build()
    .unwrap();

  _ = parser.start();

  while let Some(value) = rx.recv().await {
    println!("{value:?}");
  }
}
