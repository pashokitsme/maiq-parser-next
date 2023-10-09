with configs(id) as (
  insert into configs default
  values
  returning id
)
insert into users (id, cached_fullname, config_ref)
values (
    $1,
    $2,
    (
      select id
      from configs
    )
  );