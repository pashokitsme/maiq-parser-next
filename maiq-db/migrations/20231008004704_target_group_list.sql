create table groups(
  id integer not null primary key autoincrement,
  group_name varchar(64) not null
);
create table target_groups(
  id integer not null primary key autoincrement,
  user_ref bigint not null,
  group_name_ref int not null,
  constraint fk_group_name_ref foreign key(group_name_ref) references groups(id)
);