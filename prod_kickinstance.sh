#!/bin/sh

# This script will be used to kick production
# For now it assumes a docker machine with access to the private repo
#

docker pull dkfn/d4g:dev
docker logs backend > old_run.txt || true
docker stop backend || true
docker stop postgres || true
docker rm -f backend || true
docker rm -f postgres || true
docker run --name backend -d -p 80:8080 dkfn/d4g:dev
docker run --name postgres -e POSTGRES_USER="d4g" -e POSTGRES_PASSWORD="Design4Green" -v /var/postgres/data:/var/lib/postgresql/data -p 5432:5432 -d postgres
sleep 3
docker cp ./test_data/logement.csv postgres:/var/lib/postgresql/data/logement.csv
docker cp ./test_data/proprietaire.csv postgres:/var/lib/postgresql/data/proprietaire.csv
docker cp ./test_data/locataire.csv postgres:/var/lib/postgresql/data/locataire.csv
docker cp ./test_data/utilisateur.csv postgres:/var/lib/postgresql/data/utilisateur.csv
docker exec -i postgres psql -U d4g -d d4g < ./init.sql
docker exec -i postgres psql -U d4g -d d4g -c "COPY logement(foyer, type, surface, nb_pieces, chauffage, date_construction, n_voie, voie1, code_postal, ville) FROM '/var/lib/postgresql/data/logement.csv' DELIMITER ';' CSV HEADER"
docker exec -i postgres psql -U d4g -d d4g -c "COPY proprietaire(foyer, nom, prenom, societe, adresse) FROM '/var/lib/postgresql/data/proprietaire.csv' DELIMITER ';' CSV HEADER"
docker exec -i postgres psql -U d4g -d d4g -c "COPY locataire(foyer, nom, prenom) FROM '/var/lib/postgresql/data/locataire.csv' DELIMITER ';' CSV HEADER"
docker exec -i postgres psql -U d4g -d d4g -c "COPY utilisateur(foyer, login, password) FROM '/var/lib/postgresql/data/utilisateur.csv' DELIMITER ';' CSV HEADER"
