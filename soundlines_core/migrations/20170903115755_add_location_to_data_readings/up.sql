alter table sound_readings
add column point geometry(POINT, 4326) not null default ST_SetSRID(ST_MakePoint(-71.1043443253471, 42.3150676015829),4326);

alter table light_readings
add column point geometry(POINT, 4326) not null default ST_SetSRID(ST_MakePoint(-71.1043443253471, 42.3150676015829),4326);

update sound_readings
set point=ST_Centroid(last_location.geom)
from (
	select cells.geom
	from cells 
	join gps_readings on ST_Contains(cells.geom, gps_readings.point)
	join sound_readings on gps_readings.created_at < sound_readings.created_at
	order by gps_readings.created_at desc
	limit 1
) as last_location;

update light_readings
set point=ST_Centroid(last_location.geom)
from (
	select cells.geom
	from cells 
	join gps_readings on ST_Contains(cells.geom, gps_readings.point)
	join light_readings on gps_readings.created_at < light_readings.created_at
	order by gps_readings.created_at desc
	limit 1
) as last_location;

