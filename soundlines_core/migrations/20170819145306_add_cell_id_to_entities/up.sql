ALTER TABLE entities ADD COLUMN cell_id integer NOT NULL DEFAULT 0;
CREATE INDEX cell_idx ON entities (cell_id);
