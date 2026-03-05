# !/bin/bash

# if the script fails, exit immediately
set -e

docker build -t ems-backend .
docker save ems-backend | bzip2 | ssh cunex-ems sudo docker load
ssh cunex-ems \  'sudo docker stop $(sudo docker ps -q)'
ssh cunex-ems \  'sudo docker compose up -d'