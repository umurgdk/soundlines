#!/bin/env sh
psql -d soundlines -c "select id, data->'age', data->'start_mating_at', data->'last_seed_at' from entities_json order by id asc limit 10;"
