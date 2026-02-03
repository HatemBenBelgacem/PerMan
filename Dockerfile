# Stage 1: Build
FROM rustlang/rust:nightly AS builder

# Notwendige System-Abhängigkeiten installieren
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Dioxus CLI via binstall (schneller als kompilieren)
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm dioxus-cli@0.6.3
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app
COPY . .

# SQLx Offline-Modus & Index Update
ENV SQLX_OFFLINE=true

# Fullstack-Build für Dioxus 0.6
RUN dx build --release --fullstack

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Kopiere die Binary und Assets aus dem dist-Ordner
COPY --from=builder /usr/src/app/dist/ /usr/local/bin/
COPY --from=builder /usr/src/app/dist/public /usr/local/bin/public
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

CMD ["./per-man-server"]