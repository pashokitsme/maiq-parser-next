use rstest::*;

use maiq_db::models::*;
use maiq_db::Result;
use maiq_db::*;

#[fixture]
async fn pool() -> Pool {
  std::env::set_var("SQLITE_PATH", "sqlite::memory:");
  maiq_db::pool().await.expect("unable to create db")
}

#[rstest]
#[tokio::test]
async fn create_user(#[future] pool: Pool) -> Result<()> {
  let pool = pool.await;
  User::get_by_id_or_create(66, &pool).await?;
  User::get_by_id_or_create(77, &pool).await?;

  let users = sqlx::query!("select id, config_ref from users group by id")
    .fetch_all(&pool)
    .await?;
  let configs = sqlx::query!("select id from configs group by id")
    .fetch_all(&pool)
    .await?;

  users.iter().zip(configs.iter()).zip([66i64, 77i64]).for_each(|i| {
    assert_eq!(i.0 .0.id, i.1);
    assert_eq!(i.0 .0.config_ref, i.0 .1.id);
  });
  Ok(())
}

#[rstest]
#[tokio::test]
async fn add_group(#[future] pool: Pool) -> Result<()> {
  let pool = pool.await;
  let mut user = User::get_by_id_or_create(0, &pool).await?;
  user.config_mut().add_group("Ир3-21", &pool).await?;
  user.config_mut().add_group("Ир1-21", &pool).await?;

  let user = User::get_by_id_or_create(0, &pool).await?;
  user
    .config()
    .groups()
    .iter()
    .zip(["Ир3-21", "Ир1-21"])
    .for_each(|(g, e)| assert_eq!(g, e));
  Ok(())
}

#[rstest]
#[tokio::test]
async fn remove_group(#[future] pool: Pool) -> Result<()> {
  let pool = pool.await;
  let mut user = User::get_by_id_or_create(0, &pool).await?;
  user.config_mut().add_group("Ир3-21", &pool).await?;
  user.config_mut().add_group("Ир1-21", &pool).await?;

  let mut user = User::get_by_id_or_create(0, &pool).await?;
  user.config_mut().remove_group("Ир1-21", &pool).await?;
  let user = User::get_by_id_or_create(0, &pool).await?;

  user
    .config()
    .groups()
    .iter()
    .zip(["Ир3-21"])
    .for_each(|(g, e)| assert_eq!(g, e));
  Ok(())
}

#[rstest]
#[tokio::test]
async fn get_all(#[future] pool: Pool) -> Result<()> {
  let pool = pool.await;
  let mut user1 = User::get_by_id_or_create(0, &pool).await?;
  user1.config_mut().add_group("Ир1-21", &pool).await?;
  user1.config_mut().add_group("Ир3-21", &pool).await?;
  let mut user2 = User::get_by_id_or_create(1, &pool).await?;
  user2.config_mut().add_group("Ир3-21", &pool).await?;

  let all = User::get_all_notified(&pool).await?;

  all
    .into_iter()
    .zip([(0i64, vec!["Ир1-21".into(), "Ир3-21".into()]), (1i64, vec!["Ир3-21".into()])])
    .for_each(|i| assert_eq!(i.0, i.1));

  Ok(())
}

#[rstest]
#[tokio::test]
async fn update(#[future] pool: Pool) -> Result<()> {
  let pool = pool.await;
  let mut user = User::get_by_id_or_create(0, &pool).await?;
  user.config_mut().set_is_notifies_enabled(false);
  user.update(&pool).await?;
  assert!(!user.config().is_notifies_enabled());
  assert!(!User::get_by_id_or_create(0, &pool)
    .await?
    .config()
    .is_notifies_enabled(),);

  Ok(())
}
