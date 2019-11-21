#!/bin/sh

# This script will be used to kick production
# For now it assumes a docker machine with access to the private repo
#
# PROD
# docker pull dkfn/d4g:dev
# DEV
docker build -t dkfn/d4g:dev .
# END DEV
docker logs backend > old_run.txt || true
docker stop backend || true
docker stop postgres || true
docker rm -f backend || true
docker rm -f postgres || true
docker run --name backend -d -p 80:8080 -e DOMAIN="vps753500.ovh.net" -e SMTP_USERNAME="green.jiraration@gmail.com" -e SMTP_PASSWORD="Bijour!1" dkfn/d4g:dev
docker run --name postgres -e POSTGRES_USER="d4g" -e POSTGRES_PASSWORD="Design4Green" -p 5432:5432 -d postgres
sleep 3
docker cp ./test_data/logement.csv postgres:/var/lib/postgresql/data/logement.csv
docker cp ./test_data/proprietaire.csv postgres:/var/lib/postgresql/data/proprietaire.csv
docker cp ./test_data/locataire.csv postgres:/var/lib/postgresql/data/locataire.csv
docker cp ./test_data/releve.csv postgres:/var/lib/postgresql/data/releve.csv
docker cp ./test_data/utilisateur.csv postgres:/var/lib/postgresql/data/utilisateur.csv
docker exec -i postgres psql -U d4g -d d4g < ./init.sql
docker exec -i postgres psql -U d4g -d d4g -c "COPY logement(foyer, type, surface, nb_pieces, chauffage, date_construction, n_voie, voie1, code_postal, ville) FROM '/var/lib/postgresql/data/logement.csv' DELIMITER ';' CSV HEADER"
docker exec -i postgres psql -U d4g -d d4g -c "COPY proprietaire(foyer, nom, prenom, societe, adresse) FROM '/var/lib/postgresql/data/proprietaire.csv' DELIMITER ';' CSV HEADER"
docker exec -i postgres psql -U d4g -d d4g -c "COPY locataire(foyer, nom, prenom) FROM '/var/lib/postgresql/data/locataire.csv' DELIMITER ';' CSV HEADER"
docker exec -i postgres psql -U d4g -d d4g -c "COPY releve(foyer, date, valeur) FROM '/var/lib/postgresql/data/releve.csv' DELIMITER ';' CSV HEADER"
docker exec -i postgres psql -U d4g -d d4g -c "COPY utilisateur(foyer, login, password, active, admin) FROM '/var/lib/postgresql/data/utilisateur.csv' DELIMITER ';' CSV HEADER"
