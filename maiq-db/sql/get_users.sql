select 
  users.id,
  users.cached_fullname,
  users.modified_at,
  users.created_at,
  configs.is_broadcast_enabled,
  configs.is_notifies_enabled,
  group_names
from users
  join configs on users.config_ref = configs.id
  left join (
    select user_ref, group_concat(group_name) as group_names
    from target_groups
      join groups on target_groups.group_name_ref = groups.id
    group by user_ref
  ) on user_ref = users.id