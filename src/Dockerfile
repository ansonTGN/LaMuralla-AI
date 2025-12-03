# ---------------------------------------------------
# 1. ETAPA DE CONSTRUCCIÓN (BUILDER)
# ---------------------------------------------------
FROM rust:1.75-slim-bookworm as builder

WORKDIR /app

# Instalar dependencias del sistema (OpenSSL y pkg-config son vitales para reqwest/neo4rs)
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# TRUCO DE CACHÉ:
# 1. Crear un proyecto vacío
RUN mkdir src && echo "fn main() {}" > src/main.rs
# 2. Copiar manifiestos
COPY Cargo.toml Cargo.lock ./
# 3. Compilar solo dependencias (esto se guarda en caché de Docker)
RUN cargo build --release

# 4. Ahora copiamos el código real.
# Borramos el dummy main para que no interfiera
RUN rm src/main.rs
COPY src ./src
COPY templates ./templates

# 5. Actualizamos la fecha de los archivos para forzar a cargo a recompilar
# (A veces cargo piensa que ya está compilado por el paso anterior)
RUN touch src/main.rs

# 6. Compilamos el binario final
RUN cargo build --release

# ---------------------------------------------------
# 2. ETAPA DE EJECUCIÓN (RUNNER)
# ---------------------------------------------------
FROM debian:bookworm-slim

WORKDIR /app

# Instalar OpenSSL y Certificados CA (Obligatorio para HTTPS con OpenAI/Neo4j+s)
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

# Crear usuario no-root por seguridad
RUN useradd -m -u 1000 appuser
USER appuser

# Copiar el binario desde el builder
COPY --from=builder /app/target/release/graph-rag-backend /app/server

# Copiar las plantillas HTML (¡Importante!)
COPY --from=builder /app/templates ./templates

# Render inyecta la variable PORT, pero definimos un default
ENV PORT=3000
ENV RUST_LOG=info
# Host 0.0.0.0 es necesario para Docker
ENV HOST=0.0.0.0 

EXPOSE 3000

# Ejecutar
CMD ["./server"]