create table wifi_readings (
  id         serial primary key,
  user_id    serial not null,
  created_at timestamptz not null,
  ssid       varchar not null,
  level      real not null,
  frequency  real not null
);

create table sound_readings (
  id         serial primary key,
  user_id    serial not null,
  created_at timestamptz not null,
  level      real not null
);

create table light_readings (
  id serial primary key,
  user_id serial not null,
  created_at timestamptz not null,
  level real not null
);

create table gps_readings (
  id serial primary key,
  user_id serial not null,
  created_at timestamptz not null,
  point geometry(POINT, 4326)
);
