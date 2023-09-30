mod logger;

#[tokio::main]
async fn main() {
  color_eyre::install().unwrap();
  dotenvy::dotenv().ok();
  logger::init_logger(false);

  let bot = maiq_bot::setup_bot().await.unwrap();
  maiq_bot::start(bot).await;
}
