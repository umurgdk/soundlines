alter table settings
drop column mating_freq,
drop column mating_duration,
drop column fruit_duration,
add column mating_freq interval not null default '7 minutes',
add column mating_duration interval not null default '2 minutes',
add column fruit_duration interval not null default '2 minutes';


alter table entities
drop column start_mating_at,
drop column last_seed_at,
add column start_mating_at timestamptz not null default now(),
add column last_seed_at timestamptz not null default now();
