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
docker run --env-file .env -p 443:4430 -p 80:8000 ems-backend 
```
