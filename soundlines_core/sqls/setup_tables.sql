CREATE EXTENSION IF NOT EXISTS postgis WITH SCHEMA public;

CREATE TABLE cells (
    id serial PRIMARY KEY NOT NULL,
    geom geometry(Polygon,4326) NOT NULL,
    wifi real DEFAULT 0.0 NOT NULL,
    light real DEFAULT 0.0 NOT NULL,
    sound real DEFAULT 0.0 NOT NULL,
    sns integer DEFAULT 0 NOT NULL,
    visit integer DEFAULT 0 NOT NULL
);

-------------------------------------------------------------------------------
-- Entity Related -------------------------------------------------------------
-------------------------------------------------------------------------------

CREATE TABLE dnas (
    id serial PRIMARY KEY NOT NULL,
    setting_id integer NOT NULL,
    size real NOT NULL,
    fitness real NOT NULL,
    life_expectancy real NOT NULL,
    growth_rate real NOT NULL,
    aging_rate real NOT NULL,
    mutation_rate real NOT NULL,
    stress_rate real NOT NULL,
    healthy_rate real NOT NULL
);

CREATE TABLE entities (
    id serial PRIMARY KEY NOT NULL,
    point geometry(Point,4326) NOT NULL,
    prefab character varying(255) DEFAULT 'w-b-1'::character varying NOT NULL,
    cell_id integer DEFAULT 0 NOT NULL,
    setting_id integer DEFAULT 1 NOT NULL,
    dna_id integer DEFAULT 1 NOT NULL,
    fitness real DEFAULT 0.0 NOT NULL,
    life_expectancy real DEFAULT 0.0 NOT NULL,
    nickname character varying DEFAULT 'entity'::character varying NOT NULL,
    age real DEFAULT 0.0 NOT NULL,
    size real DEFAULT 0.0 NOT NULL,
    start_mating_at timestamp with time zone DEFAULT now() NOT NULL,
    last_seed_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TABLE seeds (
    id serial PRIMARY KEY NOT NULL,
    cell_id integer NOT NULL,
    dna_id integer NOT NULL,
    point geometry(Point,4326) NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    age real NOT NULL
);

CREATE TABLE settings (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    growth_limit real NOT NULL,
    life_expectancy real NOT NULL,
    wifi_sensitivity real NOT NULL,
    light_sensitivity real NOT NULL,
    sound_sensitivity real NOT NULL,
    neighbor_tolerance real NOT NULL,
    birth_proba real NOT NULL,
    bloom_proba real NOT NULL,
    mating_freq interval NOT NULL,
    mating_duration interval NOT NULL,
    fruit_duration interval NOT NULL,
    mating_distance real NOT NULL,
    crowd_distance real NOT NULL,
    prefab character varying(255) DEFAULT ''::character varying NOT NULL,
);


-------------------------------------------------------------------------------
-- Meta -----------------------------------------------------------------------
-------------------------------------------------------------------------------

CREATE TABLE parameters (
    id integer NOT NULL,
    cell_size integer NOT NULL
);

CREATE TABLE users (
    id integer NOT NULL,
    created_at timestamp with time zone NOT NULL
);

-------------------------------------------------------------------------------
-- Data Readings --------------------------------------------------------------
-------------------------------------------------------------------------------

CREATE TABLE wifi_readings (
    id integer NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp with time zone NOT NULL,
    ssid character varying NOT NULL,
    level real NOT NULL,
    frequency real NOT NULL
);

CREATE TABLE sound_readings (
    id integer NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp with time zone NOT NULL,
    level real NOT NULL
);

CREATE TABLE gps_readings (
    id integer NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp with time zone NOT NULL,
    point geometry(Point,4326)
);

CREATE TABLE light_readings (
    id integer NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp with time zone NOT NULL,
    level real NOT NULL
);

