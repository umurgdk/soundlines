---------------------------------------------------------
-- Entities
--
CREATE TABLE entities_json
(
  id         SERIAL                NOT NULL
    CONSTRAINT entities_json_pkey
    PRIMARY KEY,
  data       JSONB                 NOT NULL,
  setting_id INTEGER               NOT NULL,
  cell_id    INTEGER               NOT NULL,
  point      GEOMETRY(Point, 4326) NOT NULL
);
CREATE UNIQUE INDEX entities_json_id_uindex
  ON entities_json (id);

INSERT INTO entities_json (data, setting_id, cell_id, point)
  SELECT
    (to_jsonb(e) - 'cell_id' - 'setting_id' - 'dna_id') || jsonb_build_object(
        'dna', to_jsonb(dnas)
    ),
    e.setting_id,
    e.cell_id,
    e.point
  FROM entities e
    JOIN dnas ON dnas.id = e.dna_id;

---------------------------------------------------------
-- Entity Neighbors
--
CREATE TABLE entity_neighbors
(
  entity_id        INTEGER    NOT NULL
    CONSTRAINT entity_neighbors_pkey
    PRIMARY KEY,
  mating_neighbors INTEGER [] NOT NULL,
  crowd_neighbors  INTEGER [] NOT NULL
);
CREATE UNIQUE INDEX entity_neighbors_entity_id_uindex
  ON entity_neighbors (entity_id);


INSERT INTO entity_neighbors (entity_id, mating_neighbors, crowd_neighbors)
  SELECT
    e.id,
    array_agg(n.id),
    '{}'::int[] AS others
  FROM entities_json e
    INNER JOIN settings_json AS s ON e.setting_id = s.id
    LEFT JOIN entities_json AS n
      ON st_dwithin(e.point :: GEOGRAPHY, n.point :: GEOGRAPHY, (s.data -> 'mating_distance') :: TEXT :: NUMERIC)
  WHERE e.id != n.id
  GROUP BY e.id;

WITH t AS (
    SELECT
      e.id            AS entity_id,
      array_agg(c.id) AS crowd_neighbors
    FROM entities_json e
      INNER JOIN settings_json AS s ON e.setting_id = s.id
      LEFT JOIN entities_json AS c
        ON st_dwithin(e.point :: GEOGRAPHY, c.point :: GEOGRAPHY, (s.data -> 'crowd_distance') :: TEXT :: NUMERIC)
    WHERE e.id != c.id
    GROUP BY e.id
)
UPDATE entity_neighbors
SET crowd_neighbors = t.crowd_neighbors
FROM t
WHERE entity_neighbors.entity_id = t.entity_id;


-- select json_array_length(neighbors.json_agg) from  neighbors;

---------------------------------------------------------
-- Seeds
--
CREATE TABLE seeds_json
(
  id         SERIAL                NOT NULL
    CONSTRAINT seeds_json_pkey
    PRIMARY KEY,
  data       JSONB                 NOT NULL,
  setting_id INTEGER               NOT NULL,
  cell_id    INTEGER               NOT NULL,
  point      GEOMETRY(Point, 4326) NOT NULL
);
CREATE UNIQUE INDEX seeds_json_id_uindex
  ON seeds_json (id);

---------------------------------------------------------
-- Cells
--
CREATE TABLE cells_json
(
  id   SERIAL                  NOT NULL
    CONSTRAINT cells_json_pkey
    PRIMARY KEY,
  data JSONB                   NOT NULL,
  geom GEOMETRY(Polygon, 4326) NOT NULL
);
CREATE UNIQUE INDEX cells_json_id_uindex
  ON cells_json (id);


INSERT INTO cells_json (id, data, geom)
  SELECT
    id,
    to_jsonb(cells) - 'id' - 'geom',
    geom
  FROM cells;

---------------------------------------------------------
-- Plant Settings
--
CREATE TABLE settings_json
(
  id   SERIAL NOT NULL
    CONSTRAINT settings_json_pkey
    PRIMARY KEY,
  data JSONB  NOT NULL
);
CREATE UNIQUE INDEX settings_json_id_uindex
  ON settings_json (id);

INSERT INTO settings_json (id, data)
  SELECT
    id,
    to_jsonb(settings) - 'id'
  FROM settings;

---------------------------------------------------------
-- Functions
--

CREATE OR REPLACE FUNCTION notify_simulation()
  RETURNS TRIGGER AS $$
DECLARE
  id  INTEGER;
  rec RECORD;
BEGIN
  IF tg_op = 'DELETE'
  THEN
    rec = OLD;
  ELSE
    rec = NEW;
  END IF;

  IF tg_table_name = 'entity_neighbors'
  THEN
    id = rec.entity_id;
  ELSE
    id = rec.id;
  END IF;

  PERFORM pg_notify('simulation', json_build_object(
      'table', tg_table_name,
      'operation', tg_op,
      'id', id
  )::text);

  RETURN rec;
END;
$$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION notify_neighbors()
  RETURNS TRIGGER AS $$
DECLARE
  setting jsonb;
BEGIN
  IF tg_op = 'DELETE'
  THEN
    setting := (select data from settings_json where id = OLD.setting_id);
    PERFORM pg_notify('neighbors', jsonb_build_object(
        'operation', 'DELETE',
        'entity_id', OLD.id,
        'point', st_asgeojson(OLD.point) :: JSONB -> 'coordinates',
        'mating_distance', (setting->'mating_distance'),
        'crowd_distance', (setting->'crowd_distance')
    ) :: TEXT);
    RETURN OLD;
  ELSE
    setting := (select data from settings_json where id = NEW.setting_id);
    PERFORM pg_notify('neighbors', jsonb_build_object(
        'operation', 'INSERT',
        'entity_id', NEW.id,
        'point', st_asgeojson(NEW.point) :: JSONB -> 'coordinates',
        'mating_distance', (setting->'mating_distance'),
        'crowd_distance', (setting->'crowd_distance')
    ) :: TEXT);
    RETURN NEW;
  END IF;
END;
$$ LANGUAGE plpgsql;

---------------------------------------------------------
-- Triggers
--

-- Entities trigger on insert
DROP TRIGGER IF EXISTS notify_simulation
ON entities_json;

CREATE TRIGGER notify_simulation
AFTER INSERT ON entities_json
FOR EACH ROW EXECUTE PROCEDURE notify_simulation();

DROP TRIGGER IF EXISTS notify_neighbors
ON entities_json;

CREATE TRIGGER notify_neighbors
AFTER DELETE OR INSERT ON entities_json
FOR EACH ROW EXECUTE PROCEDURE notify_neighbors();

-- Seeds trigger on delete/insert
DROP TRIGGER IF EXISTS notify_simulation
ON seeds_json;

CREATE TRIGGER notify_simulation
AFTER DELETE OR INSERT ON seeds_json
FOR EACH ROW EXECUTE PROCEDURE notify_simulation();

-- Neighbors trigger on insert, update and delete
DROP TRIGGER IF EXISTS notify_simulation
ON entity_neighbors;

CREATE TRIGGER notify_simulation
AFTER INSERT OR UPDATE OR DELETE ON entity_neighbors
FOR EACH ROW EXECUTE PROCEDURE notify_simulation();









update entity_neighbors set mating_neighbors = array_remove(mating_neighbors, 75581), crowd_neighbors = array_remove(crowd_neighbors, 75581);
select array_remove('{1, 2, 3}'::int[], 2);

select setting_id from entities_json where id = 74006;


select jsonb_build_object(
        'operation', 'DELETE',
        'entity_id', '123',
        'point', '123',
        'mating_distance', (select data->'mating_distance' from settings_json where id = 11)
    ) :: TEXT;

select st_asgeojson(point) from entities_json where id = 33323;
select *, st_distance(point::geography, st_setsrid(st_point(126.992057709281,37.5740173649277), 4326)) from entities_json where st_dwithin(point::geography, st_setsrid(st_point(126.992057709281,37.5740173649277), 4326)::geography, 50.0);

-- insert into entity_neighbors_new (entity_id, other_id, distance)
SELECT DISTINCT ON (e.id, n.id)
    count(*)
  FROM entities_json e
  INNER JOIN entities_json as n on (e.id != n.id)
GROUP BY e.id, n.id;

select count(*) from entities_json;




select unnest(a.i)::text || 'a' from (select '{1,2,3}'::integer[] as i) a;
select count(*) from entities_json e where (e.data->'last_seed_at')::text::numeric > 0;


SELECT (
  seeds.data ||
  jsonb_build_object(
      'setting', settings_json.data || jsonb_build_object('id', settings_json.id),
      'point', st_asgeojson(seeds.point) :: JSONB -> 'coordinates',
      'id', seeds.id
  )
) :: TEXT
FROM seeds_json AS seeds
  INNER JOIN settings_json ON settings_json.id = seeds.setting_id;

DELETE FROM entities_json;
DELETE FROM seeds_json;
DELETE FROM entity_neighbors;

SELECT *
FROM settings_json;
SELECT COUNT(*)
FROM seeds_json
  JOIN settings_json ON settings_json.id = seeds_json.setting_id;
SELECT count(*)
FROM entities_json;
SELECT DISTINCT st_asgeojson(seeds_json.point)
FROM seeds_json;
SELECT DISTINCT st_asgeojson(entities_json.point)
FROM entities_json;

-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (20, 'herbaceous cactus', 350, 450, 0, 0, 0, 5, 1, 1, 200, 36, 'h-c-1', 12, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (21, 'herbaceous cactus', 350, 450, 0, 0, 0, 5, 1, 1, 200, 36, 'h-c-2', 12, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (11, 'woody conifers', 300, 350, 0, 0, 0, 5, 1, 1, 200, 42, 'w-c-1', 6, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (12, 'woody conifers', 300, 350, 0, 0, 0, 5, 1, 1, 200, 42, 'w-c-2', 6, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (13, 'woody conifers', 300, 350, 0, 0, 0, 5, 1, 1, 200, 42, 'w-c-3', 6, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (14, 'herbaceous grass', 300, 250, 0, 0, 0, 4, 1, 1, 200, 39, 'h-g-1', 7, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (15, 'herbaceous grass', 300, 250, 0, 0, 0, 4, 1, 1, 200, 39, 'h-g-2', 7, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (16, 'herbaceous grass', 300, 250, 0, 0, 0, 4, 1, 1, 200, 39, 'h-g-3', 7, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (17, 'herbaceous bamboo', 350, 350, 0, 0, 0, 5, 1, 1, 200, 40, 'h-b-1', 8, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (18, 'herbaceous bamboo', 350, 350, 0, 0, 0, 5, 1, 1, 200, 40, 'h-b-2', 8, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (19, 'herbaceous bamboo', 350, 350, 0, 0, 0, 5, 1, 1, 200, 40, 'h-b-3', 8, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (8, 'woody broadleaaved trees', 500, 400, 0, 0, 0, 4, 1, 1, 200, 50, 'w-b-1', 7, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (9, 'woody broadleaaved trees', 500, 400, 0, 0, 0, 4, 1, 1, 200, 50, 'w-b-2', 7, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (10, 'woody broadleaaved trees', 500, 400, 0, 0, 0, 4, 1, 1, 200, 50, 'w-b-3', 7, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (22, 'herbaceous cactus', 350, 300, 0, 0, 0, 5, 1, 1, 200, 36, 'h-c-3', 12, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (23, 'herbaceous fern', 300, 350, 0, 0, 0, 4, 1, 1, 200, 43, 'h-f-1', 7, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (24, 'herbaceous fern', 300, 500, 0, 0, 0, 4, 1, 1, 200, 43, 'h-f-2', 7, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (25, 'herbaceous fern', 300, 500, 0, 0, 0, 4, 1, 1, 200, 43, 'h-f-3', 7, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (26, 'herbaceous moss', 300, 500, 0, 0, 0, 4, 1, 1, 200, 36, 'h-m-1', 10, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (27, 'herbaceous moss', 300, 300, 0, 0, 0, 4, 1, 1, 200, 36, 'h-m-2', 10, 10, 10);
-- INSERT INTO settings (id, name, growth_limit, life_expectancy, wifi_sensitivity, light_sensitivity, sound_sensitivity, neighbor_tolerance, birth_proba, bloom_proba, mating_distance, crowd_distance, prefab
-- , mating_freq, mating_duration, fruit_duration) VALUES (28, 'herbaceous moss', 300, 300, 0, 0, 0, 4, 1, 1, 200, 36, 'h-m-3', 10, 10, 10);