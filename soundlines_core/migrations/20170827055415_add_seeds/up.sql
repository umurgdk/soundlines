create table seeds(
	id serial not null primary key,
	cell_id integer not null,
	dna_id integer not null,
	setting_id integer not null,
	point geometry(POINT, 4326) not null,
	created_at timestamptz not null,
	age real not null
);
