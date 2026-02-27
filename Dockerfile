# Stage 1: Build
FROM rustlang/rust:nightly AS builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev curl \
    && rm -rf /var/lib/apt/lists/*

# Wir nutzen das schnelle binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm dioxus-cli@0.6.3
RUN rustup target add wasm32-unknown-unknown

# Ordern app wird unter usr/src/app erstellt
WORKDIR /usr/src/app
COPY . .
ENV SQLX_OFFLINE=true


# Dioxus baut das komplette Bundle (Binary + Assets)
RUN dx build --release --platform server || (echo "BUILD FEHLGESCHLAGEN!" && exit 1)

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Wir kopieren EXAKT den Ordner, in den Dioxus das fertige Server-Bundle abgelegt hat!
COPY --from=builder /usr/src/app/target/dx/per-man/release/web/ ./

# Datenbank-Migrationen kopieren
COPY --from=builder /usr/src/app/migrations ./migrations

# Railway und Dioxus Variablen
ENV PORT=8080
ENV IP=0.0.0.0
ENV HOST=0.0.0.0
ENV BIND_ADDR=0.0.0.0

EXPOSE 8080

# Dioxus hat die Binary im fertigen Web-Ordner "server" genannt
CMD ["./server"]