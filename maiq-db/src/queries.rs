use log::*;
use sqlx::*;

use crate::models::*;
use crate::utils::*;
use crate::Db;
use crate::Result;

pub type UserEntry = (i64, Vec<String>);

impl User {
  pub async fn insert(self, pool: &Pool<Db>) -> Result<Self> {
    info!(target: "db", "inserting new user id {}", self.chat_id);
    sqlx::query_file!("sql/insert_new_user.sql", self.chat_id, self.cached_fullname)
      .execute(pool)
      .await?;
    Ok(self)
  }

  pub async fn get_by_id_or_create<'c>(id: i64, pool: &Pool<Db>) -> Result<Self> {
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
        target_groups: row
          .group_names
          .unwrap_or_default()
          .split(',')
          .map(Into::into)
          .collect::<Vec<String>>(),
        chat_id: row.id,
      },
      created_at,
      modified_at,
    };

    Ok(user)
  }

  pub async fn all(pool: &Pool<Db>) -> Result<Vec<UserEntry>> {
    let entries = sqlx::query!(
      r#"
        select users.id as id, group_concat(group_name) as groups from users 
        join configs on configs.id = users.config_ref
        join target_groups on target_groups.user_ref = users.id
        join groups on groups.id = target_groups.group_name_ref
        where configs.is_notifies_enabled = 1
        group by users.id;
      "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| (row.id, row.groups.split(',').map(Into::into).collect()))
    .collect();

    Ok(entries)
  }

  pub async fn update(&self, pool: &Pool<Db>) -> Result<()> {
    info!(target: "db", "update user {}", self.chat_id);

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
    .execute(pool)
    .await?;

    Ok(())
  }

  pub async fn update_username(&mut self, name: String, pool: &Pool<Db>) -> Result<()> {
    if matches!(self.cached_fullname.as_ref(), Some(cached) if cached.as_str() == name) {
      return Ok(());
    }

    info!(target: "db", "update cached fullname {:?} -> {}", self.cached_fullname, name);
    self.cached_fullname = Some(name);

    sqlx::query!(
      r#"update users 
      set 
        cached_fullname = $2,
        modified_at = datetime()
      where id = $1"#,
      self.chat_id,
      self.cached_fullname,
    )
    .execute(pool)
    .await?;

    Ok(())
  }
}

impl Config {
  pub async fn add_group<S: AsRef<str>>(&mut self, name: S, pool: &Pool<Db>) -> Result<()> {
    if self.target_groups.iter().any(|n| n == name.as_ref()) {
      return Ok(());
    };

    let mut tx = pool.begin().await?;
    let name = name.as_ref();
    sqlx::query!(
      r#"
        insert into groups(group_name)
        select $1 where not exists (select 1 from groups where groups.group_name = $1 limit 1)
      "#,
      name
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
      r#"
        insert into target_groups(user_ref, group_name_ref)
        values ($1, (select id from groups where group_name = $2))
      "#,
      self.chat_id,
      name
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    self.target_groups.push(name.into());
    Ok(())
  }

  pub async fn remove_group<S: AsRef<str>>(&mut self, name: S, pool: &Pool<Db>) -> Result<()> {
    let name = name.as_ref();
    sqlx::query!(
      r#"
        delete from target_groups
        where user_ref = $1 and group_name_ref = (select id from groups where group_name = $2)
      "#,
      self.chat_id,
      name
    )
    .execute(pool)
    .await?;

    self.target_groups.retain(|n| n != name);
    Ok(())
  }
}
