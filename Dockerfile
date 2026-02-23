# Stage 1: Build
FROM rustlang/rust:nightly AS builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev curl \
    && rm -rf /var/lib/apt/lists/*

# CLI Version 0.6.3 passend zum Crate installieren
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm dioxus-cli@0.6.3
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app
COPY . .

# SQLx Offline-Modus (verhindert DB-Connect Fehler beim Build)
ENV SQLX_OFFLINE=true


RUN dx build --release --platform fullstack || (echo "BUILD FEHLGESCHLAGEN!" && exit 1)

# DIAGNOSE: Zeigt im Railway-Log an, was wirklich im dist-Ordner liegt
RUN ls -R dist || echo "Dist-Ordner wurde nicht erstellt!"

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Kopiere ALLES aus dem dist-Ordner
COPY --from=builder /usr/src/app/dist .
COPY --from=builder /usr/src/app/migrations ./migrations

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

# In Dioxus 0.6 Fullstack ist die Binary direkt im dist-Ordner
CMD ["./per-man"]