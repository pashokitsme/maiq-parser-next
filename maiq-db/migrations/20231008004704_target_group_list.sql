create table groups(
  id serial primary key,
  group_name varchar(64) not null
);
create table target_groups(
  id serial primary key,
  user_ref bigint not null,
  group_name_ref int not null,
  constraint fk_group_name_ref foreign key(group_name_ref) references groups(id)
);