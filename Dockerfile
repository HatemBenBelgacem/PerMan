# Stage 1: Build
FROM rustlang/rust:nightly AS builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev curl \
    && rm -rf /var/lib/apt/lists/*

# CLI Version 0.6.3 installieren
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm dioxus-cli@0.6.3
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app
COPY . .
ENV SQLX_OFFLINE=true

# WICHTIG: --platform server zwingt Dioxus, Backend UND Frontend zu bauen
RUN dx build --release --platform server || (echo "BUILD FEHLGESCHLAGEN!" && exit 1)

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1. Server-Binary kopieren (Cargo legt diese standardmäßig hier ab)
COPY --from=builder /usr/src/app/target/release/per-man ./per-man

# 2. Web-Assets (WASM, JS, CSS) kopieren (Das ist der Ordner aus deinem Log!)
COPY --from=builder /usr/src/app/target/dx/per-man/release/web/public ./public

# 3. Datenbank-Migrationen kopieren
COPY --from=builder /usr/src/app/migrations ./migrations

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

CMD ["./per-man"]