use sqlx::*;

use crate::models::User;

impl User {
  pub async fn insert<'e, 'c, E: 'e + Executor<'c, Database = Postgres>>(self, pool: E) -> Result<Self> {
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
}
