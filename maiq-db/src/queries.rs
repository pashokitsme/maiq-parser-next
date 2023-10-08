use chrono::FixedOffset;
use sqlx::*;

use crate::models::Config;
use crate::models::User;
use crate::utils::*;
use crate::Db;

impl User {
  pub async fn insert(self, pool: &Pool<Db>) -> Result<Self> {
    sqlx::query!(
      "insert into users (id, cached_fullname, modified_at, created_at) values ($1, $2, $3, $4)",
      self.id as i64,
      self.cached_fullname(),
      self.modified_at.naive_utc(),
      self.created_at.naive_utc()
    )
    .execute(pool)
    .await?;
    Ok(self)
  }

  pub async fn get_by_id<'c>(id: u64, pool: &Pool<Db>) -> Result<Self> {
    let row = sqlx::query_file!("sql/get_user_by_id.sql", id as i64)
      .fetch_one(pool)
      .await?;

    let created_at = DateTime::from_naive(row.created_at);
    let modified_at = DateTime::from_naive(row.modified_at);

    let user = User {
      id: row.id as u64,
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
}
