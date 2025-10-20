#!/usr/bin/env bash
builtin export container_name="scraper_postgres";
if [[ ! $(grep POSTGRES_PASSWORD .env) ]]; then
	builtin echo "POSTGRES_PASSWORD=$(uuidgen)" >> .env;
fi
. .env
if [[ ! $(docker ps --filter "name=${container_name}" --filter "status=running" | sed '1d' | wc -l) -lt 1 ]]; then
	docker cp scripts/schema/charts.sql ${container_name}:/tmp/
	docker exec -it ${container_name} psql -U postgres -f /tmp/charts.sql
else 
	builtin echo "${container_name} not running";
fi
