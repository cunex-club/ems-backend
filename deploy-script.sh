export CUNEX_KEY_PATH=/Users/smartwatt/Documents/projects/cunex/cu-team_key.pem
ssh -i $CUNEX_KEY_PATH azureuser@cuelection.southeastasia.cloudapp.azure.com sudo docker stop $(sudo docker ps -q)
docker build -t ems-backend .
docker save ems-backend | bzip2 | ssh -i $CUNEX_KEY_PATH azureuser@cuelection.southeastasia.cloudapp.azure.com sudo docker load
ssh -i $CUNEX_KEY_PATH azureuser@cuelection.southeastasia.cloudapp.azure.com sudo docker run --env-file .env -p 443:4430 -p 80:8000 --restart=unless-stopped -d -v ~/cunex-ems-449308-89f684faa146.json:/cunex-ems-449308-89f684faa146.json:ro ems-backend 