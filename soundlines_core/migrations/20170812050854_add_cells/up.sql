create table cells (
	id serial primary key,
	geom geometry(POLYGON, 4326) not null
);
