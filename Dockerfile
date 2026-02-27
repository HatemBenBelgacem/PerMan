# Stage 1: Build
FROM rustlang/rust:nightly AS builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev curl \
    && rm -rf /var/lib/apt/lists/*

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm dioxus-cli@0.6.3
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app
COPY . .
ENV SQLX_OFFLINE=true

# Build ausführen
RUN dx build --release || (echo "BUILD FEHLGESCHLAGEN!" && exit 1)

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Wir gehen in den app Ordner
WORKDIR /app

# BINGO: Wir kopieren jetzt den GANZEN Ordner "web" und nennen ihn im Container auch "web"
COPY --from=builder /usr/src/app/target/dx/per-man/release/web /app/web

# 2. NEU: Wir holen den originalen Assets-Ordner direkt aus dem Quellcode!
COPY --from=builder /usr/src/app/assets /app/web/assets

# Migrationen kopieren
COPY --from=builder /usr/src/app/migrations /app/migrations

ENV PORT=8080
ENV IP=0.0.0.0
ENV HOST=0.0.0.0
ENV BIND_ADDR=0.0.0.0

EXPOSE 8080

# WICHTIG: Wir wechseln IN den kopierten Web-Ordner, bevor wir starten!
# So findet der Server seine Assets garantiert, weil er direkt daneben steht.
WORKDIR /app/web

CMD ["./server"]