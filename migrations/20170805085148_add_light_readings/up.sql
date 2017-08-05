-- Your SQL goes here
create table light_readings (
  id serial primary key,
  user_id serial not null,
  created_at timestamptz not null,
  level real not null
);
