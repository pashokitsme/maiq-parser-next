create table configs(
  id serial not null primary key,
  is_notifies_enabled boolean not null default(true),
  is_broadcast_enabled boolean not null default(true)
);

create table users(
  id bigint not null primary key,
  cached_fullname varchar(256),
  config_ref int not null,
  modified_at timestamp not null default(now()),
  created_at timestamp not null default(now()),
  constraint fk_config foreign key(config_ref) references configs(id)
);