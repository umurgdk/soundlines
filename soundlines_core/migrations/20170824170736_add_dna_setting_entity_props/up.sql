create table settings (
	id serial not null primary key,
	name varchar(255) not null,
	growth_limit real not null,
	life_expectancy real not null,
	wifi_sensitivity real not null,
	light_sensitivity real not null,
	sound_sensitivity real not null,
	neighbor_tolerance real not null,
	birth_proba real not null,
	bloom_proba real not null,
	mating_freq interval not null,
	mating_duration interval not null,
	fruit_duration interval not null,
	mating_distance real not null,
	crowd_distance real not null
);

create table dnas (
	id serial not null primary key,
	size real not null,
	fitness real not null,
	life_expectancy real not null,
	growth_rate real not null,
	aging_rate real not null,
	mutation_rate real not null,
	stress_rate real not null,
	healty_rate real not null
);

alter table entities
add column setting_id integer not null default 1,
add column dna_id integer not null default 1,
add column fitness real not null default 0.0,
add column life_expectancy real not null default 0.0,
add column nickname varchar not null default 'entity',
add column start_mating_at timestamptz not null default now(),
add column last_seed_at timestamptz not null default now();
