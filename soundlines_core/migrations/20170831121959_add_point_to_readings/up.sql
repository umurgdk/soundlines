alter table wifi_readings
add column point geometry(POINT, 4326) not null;
