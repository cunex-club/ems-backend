FROM rust:1.93.1-bookworm as build

WORKDIR /usr/src/app

# SQLx query macros validate SQL at compile time. Dokploy supplies this build
# argument from its protected app configuration; it is scoped to this builder
# stage and is not present in the final runtime image.
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

COPY . .

RUN apt-get update && apt-get install libpq5 -y

RUN cargo build --release




FROM debian:bookworm-slim


RUN apt-get update && apt-get install libssl-dev libpq5 ca-certificates openssl -y 


COPY --from=build /usr/src/app/target/release/ /usr/local/bin/

# COPY --from=build /usr/src/app/ssl/ /ssl/
# RUN chmod -R 755 /ssl/

# COPY --from=build /usr/src/app/.env /usr/local/bin/.env

EXPOSE 8000
EXPOSE 4430

CMD [ "ems-backend"]

