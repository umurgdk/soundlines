SELECT
  entities.id,
  (
	entities.data ||
	jsonb_build_object('setting', to_jsonb("settings")) ||
	jsonb_build_object('point', st_asgeojson(entities.point)::jsonb->'coordinates')
  ) :: TEXT AS data,
  entities.cell_id
FROM entities_json AS entities
INNER JOIN settings ON settings.id = entities.setting_id;

create INDEX entities_json_point_index on entities_json using gist (point);

CREATE EXTENSION btree_gist;

drop INDEX entities_json_point_gg_index;
create INDEX entities_json_point_gg_index on entities_json using gist (id, geography(point));



insert into entity_neighbors (entity_id, neighbors)
SELECT
  e.id,
  case when count(n.*) > 0 then jsonb_agg(n.id) else jsonb_build_array() end
from entities_json e
LEFT JOIN entities_json as n
	on e.id <> n.id and st_dwithin(geography(e.point), geography(n.point), 100)
GROUP BY e.id;

SELECT
-- 	jsonb_agg(
  		'[' ||
  		string_agg
		(
		  entities.data ||
		  jsonb_build_object('setting', to_jsonb("settings")) ||
		  jsonb_build_object('point', st_asgeojson(entities.point)::jsonb->'coordinates')
		  ::text,
		  ','
		) ||
		']'
-- 	)
FROM entities_json AS entities
INNER JOIN settings ON settings.id = entities.setting_id;

insert into entities_json (data, setting_id, cell_id, point)
select
  e.data,
  e.setting_id,
  e.cell_id,
  e.point
from entities_json e;

update entities_json set data = jsonb_set(data, '{id}', id::text::jsonb);


DELETE FROM entities_json;

INSERT INTO entities_json (data, setting_id, cell_id, point)
  SELECT
	to_jsonb(entities) - 'dna_id' - 'setting_id' - 'point' || jsonb_build_object('dna', to_jsonb(dnas) - 'id' - 'setting_id') AS data,
	entities.setting_id,
	cell_id,
	point
  FROM entities
	INNER JOIN dnas ON dnas.id = entities.dna_id;

select * from entity_neighbors;
select json_build_array(entity_id, neighbors)::text from entity_neighbors;

select * from entities_json where st_dwithin(geography(point), st_makepoint(126.995941, 37.570373), 100);
select to_jsonb(cells.*) || jsonb_build_object('geom', st_asgeojson(geom)::jsonb->'coordinates'->0) from cells;


-- Select entities
SELECT
	(
		entities.data ||
		jsonb_build_object('setting', to_jsonb("settings")) ||
		jsonb_build_object('point', st_asgeojson(entities.point)::jsonb->'coordinates')
	)::text
FROM entities_json AS entities
INNER JOIN settings ON settings.id = entities.setting_id;

--------------------------------------------------
-- SEEDS


-- Select seeds
SELECT
	(
		seeds.data ||
		jsonb_build_object('setting', to_jsonb("settings")) ||
		jsonb_build_object('point', st_asgeojson(seeds.point)::jsonb->'coordinates')
	)::text
FROM seeds_json AS seeds
INNER JOIN settings ON settings.id = seeds.setting_id;


-- Populate seeds_json table

delete from seeds_json;

INSERT INTO seeds_json (data, setting_id, cell_id, point)
  SELECT
	to_jsonb(seeds) - 'dna_id' - 'setting_id' - 'point' || jsonb_build_object('dna', to_jsonb(dnas) - 'id' - 'setting_id') AS data,
	seeds.setting_id,
	cell_id,
	point
  FROM seeds
	INNER JOIN dnas ON dnas.id = seeds.dna_id;