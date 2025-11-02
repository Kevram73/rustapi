# Étape 1: Build
FROM rust:1.88 as builder

WORKDIR /app

# Installer les dépendances système nécessaires pour PostgreSQL
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copier les fichiers de dépendances
COPY Cargo.toml Cargo.lock* ./

# Préparer un projet vide pour le cache des dépendances
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# Copier le code source
COPY . .

# Compiler l'application
RUN cargo build --release

# Étape 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Installer les dépendances runtime pour PostgreSQL
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copier le binaire et les migrations
COPY --from=builder /app/target/release/rustapi /app/rustapi
COPY --from=builder /app/migrations ./migrations

# Exposer le port
EXPOSE 3004

# Variables d'environnement
ENV RUST_LOG=info
ENV PORT=3004
ENV HOST=0.0.0.0

# Lancer le serveur
CMD ["/app/rustapi"]
