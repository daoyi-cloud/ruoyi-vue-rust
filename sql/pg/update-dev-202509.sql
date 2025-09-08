alter table system_menu
    add perm_apis text default '[]' not null;

comment on column system_menu.perm_apis is '授权api';