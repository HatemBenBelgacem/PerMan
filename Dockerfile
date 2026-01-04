# Stage 1: Build
FROM rustlang/rust:nightly AS builder

# Notwendige Abhängigkeiten für Dioxus und SQLx
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install dioxus-cli --version 0.6.0
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app
COPY . .

# SQLx Offline-Modus oder Datenbank-Check umgehen
ENV SQLX_OFFLINE=true

# Fullstack-Build (erzeugt Server-Binary und WASM-Assets)
RUN dx build --release --platform fullstack

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Kopiere die Binary und das öffentliche Asset-Verzeichnis
# "per-man" ist der Name aus deiner Cargo.toml
COPY --from=builder /usr/src/app/target/dx/per-man/release/server /usr/local/bin/per-man-server
COPY --from=builder /usr/src/app/target/dx/per-man/release/public /usr/local/bin/public
# Kopiere Migrations für sqlx::migrate!
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

CMD ["./per-man-server"]