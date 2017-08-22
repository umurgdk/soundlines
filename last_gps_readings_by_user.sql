select user_id, point, created_at
from gps_readings
where id in (select distinct max(id) from gps_readings group by user_id)
order by created_at desc;
