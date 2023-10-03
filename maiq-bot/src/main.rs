mod logger;

#[tokio::main]
async fn main() {
  color_eyre::install().unwrap();
  dotenvy::dotenv().ok();
  logger::init_logger(false);

  let bot = maiq_bot::setup_bot().await.expect("unable to setup bot");
  let parser = maiq_bot::setup_parser().expect("unable to setup parser");
  maiq_bot::start(bot, parser).await;
}
