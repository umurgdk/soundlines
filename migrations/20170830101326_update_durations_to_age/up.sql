alter table settings
drop column mating_freq,
drop column mating_duration,
drop column fruit_duration,
add column mating_freq real not null default 50.0,
add column mating_duration real not null default 15.0,
add column fruit_duration real not null default 15.0;


alter table entities
drop column start_mating_at,
drop column last_seed_at,
add column start_mating_at real not null default 0.0,
add column last_seed_at real not null default 0.0;
