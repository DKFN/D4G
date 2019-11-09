#!/bin/sh

# This script will be used to kick production
# For now it assumes a docker machine with access to the private repo
#

docker pull dkfn/d4g:dev
docker logs backend > old_run.txt || true
docker stop backend || true
docker rm -f backend || true
docker run --name backend -d -p 80:8080 dkfn/d4g:dev

