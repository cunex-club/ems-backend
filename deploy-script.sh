export CUNEX_KEY_PATH=/Users/smartwatt/Documents/projects/cunex/cu-team_key.pem
docker build -t ems-backend .
docker save ems-backend | bzip2 | ssh -i $CUNEX_KEY_PATH azureuser@cuelection.southeastasia.cloudapp.azure.com sudo docker load
ssh -i "$CUNEX_KEY_PATH" azureuser@cuelection.southeastasia.cloudapp.azure.com \  'sudo docker stop $(sudo docker ps -q)'
ssh -i "$CUNEX_KEY_PATH" azureuser@cuelection.southeastasia.cloudapp.azure.com \  'sudo docker compose up -d'