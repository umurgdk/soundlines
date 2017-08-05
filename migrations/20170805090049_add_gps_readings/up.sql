-- Your SQL goes here
create table gps_readings (
  id serial primary key,
  user_id serial not null,
  created_at timestamptz not null,
  latitude double precision not null,
  longitude double precision not null
);
