update entities set cell_id = cell.id
from ( select id, geom from cells ) cell
where st_contains(cell.geom, point);