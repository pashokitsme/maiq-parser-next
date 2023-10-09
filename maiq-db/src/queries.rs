use log::*;
use sqlx::*;

use crate::models::*;
use crate::utils::*;
use crate::Db;
use crate::Result;

impl User {
  pub async fn insert(self, pool: &Pool<Db>) -> Result<Self> {
    info!(target: "db", "inserting new user id {}", self.chat_id);
    sqlx::query_file!("sql/insert_new_user.sql", self.chat_id, self.cached_fullname)
      .execute(pool)
      .await?;
    Ok(self)
  }

  pub async fn get_by_id_or_create<'c>(id: i64, pool: &Pool<Db>) -> Result<Self> {
    info!(target: "db", "get user {}", id);
    let row = sqlx::query_file!("sql/get_user_by_id.sql", id)
      .fetch_optional(pool)
      .await
      .unwrap();

    let row = if let Some(row) = row { row } else { return User::insert(User::new(id), pool).await };

    let created_at = DateTime::from_naive(row.created_at);
    let modified_at = DateTime::from_naive(row.modified_at);

    let user = User {
      chat_id: row.id,
      cached_fullname: row.cached_fullname,
      config: Config {
        is_notifies_enabled: row.is_notifies_enabled,
        is_broadcast_enabled: row.is_broadcast_enabled,
        target_groups: row.group_names.unwrap_or_default(),
      },
      created_at,
      modified_at,
    };

    Ok(user)
  }

  pub async fn update(&self, pool: &Pool<Db>) -> Result<()> {
    info!(target: "db", "update user {}", self.chat_id);
    let mut tx = pool.begin().await?;

    sqlx::query!(
      r#"update users 
      set 
        cached_fullname = $2,
        modified_at = now()
      where id = $1"#,
      self.chat_id,
      self.cached_fullname,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
      r#"update configs
       set 
         is_notifies_enabled = $2,
         is_broadcast_enabled = $3
       where id in (select config_ref from users where id = $1)
      "#,
      self.chat_id,
      self.config.is_notifies_enabled,
      self.config.is_broadcast_enabled
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
  }
}
