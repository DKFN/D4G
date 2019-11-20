#!/bin/sh

# This script will be used to kick production
# For now it assumes a docker machine with access to the private repo
#

docker pull dkfn/d4g:dev
docker logs backend > old_run.txt || true
docker stop backend || true
docker rm -f backend || true
docker rm -f postgres || true
docker run --name backend -d -p 80:8080 dkfn/d4g:dev
docker run --name postgres -e POSTGRES_USER="d4g" -e POSTGRES_PASSWORD="Design4Green" -v /var/postgres/data:/var/lib/postgresql/data -p 5432:5432 -d postgres
sleep 1
docker exec -i postgres psql -U d4g -d d4g < ./init.sql

