# Stage 1: Build
FROM rustlang/rust:nightly AS builder

# Notwendige Abhängigkeiten
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl

# Installiert cargo-binstall und die Dioxus CLI
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm dioxus-cli@0.6.0
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app
COPY . .

# SQLx Offline-Modus
ENV SQLX_OFFLINE=true

# Fullstack-Build (Dioxus 0.6 baut automatisch Client & Server)
RUN dx build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Kopiere die Binary und das öffentliche Asset-Verzeichnis aus dem dist-Ordner
# Ersetze "per-man" durch den Namen deiner Binary, falls er abweicht
COPY --from=builder /usr/src/app/dist/per-man /usr/local/bin/per-man-server
COPY --from=builder /usr/src/app/dist/public /usr/local/bin/public
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

CMD ["./per-man-server"]