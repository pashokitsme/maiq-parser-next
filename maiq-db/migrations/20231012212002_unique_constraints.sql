alter table groups
add constraint unique_name unique(group_name);
alter table target_groups
add constraint unique_pair unique(user_ref, group_name_ref);