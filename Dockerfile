# Stage 1: Build
FROM rustlang/rust:nightly AS builder

# Notwendige System-Abhängigkeiten
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Dioxus CLI passend zur Cargo.lock Version (0.6.3) installieren
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm dioxus-cli@0.6.3
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app
COPY . .

# WICHTIG: Kein "cargo update" hier, um Versionskonflikte zu vermeiden
ENV SQLX_OFFLINE=true

# Fullstack-Build für Dioxus 0.6
RUN dx build --release --fullstack

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Wir arbeiten im Ordner /app
WORKDIR /app

# Kopiere ALLES aus dem dist-Ordner in das aktuelle Verzeichnis
# Das beinhaltet die Binary "per-man" und alle Web-Assets
COPY --from=builder /usr/src/app/dist .

# Kopiere die Datenbank-Migrationen in einen Unterordner
COPY --from=builder /usr/src/app/migrations ./migrations

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

# Starte die Binary direkt aus dem Arbeitsverzeichnis
# In Dioxus 0.6 heißt die Datei wie das Projekt in der Cargo.toml
CMD ["./per-man"]