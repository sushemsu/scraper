#!/usr/bin/env bash
builtin export container_name="scraper_postgres";
if [[ ! $(grep POSTGRES_PASSWORD .env) ]]; then
	builtin echo "POSTGRES_PASSWORD=$(uuidgen)" >> .env;
fi
. .env
if [[ $(docker ps -a --filter "name=${container_name}" | sed '1d' | wc -l) -lt 1 ]]; then
	docker run --name ${container_name} -e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} -p 5432:5432 -d postgres:16.10-alpine3.22
elif [[ $(docker ps --filter "name=${container_name}" --filter "status=running" | sed '1d' | wc -l) -lt 1 ]]; then
	docker start ${container_name}
fi
docker ps --filter "name=${container_name}" --filter "status=running"
