insert into configs default values;

insert into users (id, cached_fullname, config_ref)
values ($1, $2, last_insert_rowid());