#!/bin/env sh
psql -d soundlines -c "select id, data->'age' from entities_json limit 10;"

