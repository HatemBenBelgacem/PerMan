# Stage 1: Build
FROM rustlang/rust:nightly AS builder

# Notwendige Abhängigkeiten für Dioxus und SQLx
RUN apt-get update && apt-get install -y pkg-config libssl-dev
# Installiert curl für den Download und libssl für die Ausführung
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl

# Installiert cargo-binstall, um fertige Binärdateien zu laden
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Installiert die Dioxus CLI blitzschnell als fertige Binary
RUN cargo binstall --no-confirm dioxus-cli@0.6.0
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