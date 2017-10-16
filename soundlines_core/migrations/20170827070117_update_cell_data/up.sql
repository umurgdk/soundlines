alter table cells
add column wifi real not null default 0.0,
add column light real not null default 0.0,
add column sound real not null default 0.0,
add column sns integer not null default 0,
add column visit integer not null default 0;
