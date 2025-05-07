# ems-backend

## Environment Variables

```bash
DATABASE_URL=connect_timeout=
GOOGLE_OAUTH_CLIENT_ID=
GOOGLE_OAUTH_CLIENT_SECRET=
HOST=
PORT=
ROOT_URI=
TOKEN_MAXAGE=
TOKEN_SECRET=
```

## Build

### Docker

```bash
docker build -t ems-backend .
```

## Run

### Docker

```bash
sudo docker run --env-file .env -p 443:4430 -p 80:8000 -restart=unless-stopped -d -v ~/cunex-ems-449308-89f684faa146.json:/cunex-ems-449308-89f684faa146.json:ro ems-backend
```

## Deploy

### Docker

```bash
docker save ems-backend | bzip2 | ssh -i cu-team_key.pem azureuser@cuelection.southeastasia.cloudapp.azure.com sudo docker load
```
