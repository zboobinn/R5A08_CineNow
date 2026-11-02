# Image commune aux 4 services Rust : le service lancé est choisi par `command` dans docker-compose.yml
FROM rust:1-trixie AS build
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked \
      -p weather-service -p movie-service -p recommendation-service -p web-server \
    && mkdir -p /out \
    && cp target/release/weather-service target/release/movie-service \
          target/release/recommendation-service target/release/web-server /out/

FROM debian:trixie-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3t64 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /out/ /usr/local/bin/
USER nobody
