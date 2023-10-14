create unique index ux_groups_group_name on groups(group_name);
create unique index ux_target_groups_pair on target_groups(user_ref, group_name_ref);