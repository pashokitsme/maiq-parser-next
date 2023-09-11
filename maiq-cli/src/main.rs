use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
  let (tx, mut rx) = mpsc::channel(32);

  let parser = maiq_parser_next::parser::PeriodicalParserBuilder::new()
    .add_url("http://google.com")
    .unwrap()
    .with_interval(Duration::from_secs(5))
    .on_update(tx)
    .build()
    .unwrap();

  let token = parser.start();

  while let Some(value) = rx.recv().await {
    println!("{value:?}");
    token.cancel()
  }
}
