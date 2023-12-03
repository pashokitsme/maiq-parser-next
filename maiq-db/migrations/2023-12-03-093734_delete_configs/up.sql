create table users_new (
  id bigint not null primary key,
  cached_fullname varchar(256),
  is_notifies_enabled boolean not null default(true),
  is_broadcast_enabled boolean not null default(true),
  modified_at timestamp not null default(datetime()),
  created_at timestamp not null default(datetime())
);

insert into users_new select
  users.id,
  users.cached_fullname,
  configs.is_notifies_enabled,
  configs.is_broadcast_enabled,
  users.modified_at,
  users.created_at
  from users join configs on users.config_ref = configs.id;

drop table users;
drop table configs;

alter table users_new
  rename to users;