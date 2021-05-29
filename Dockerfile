# 1: Build
FROM rust:1.52.1 as builder
RUN groupadd --gid 1000 appuser && \
    useradd --uid 1000 --gid 1000 --shell /bin/sh appuser && \
    mkdir -p /home/appuser/app && \
    chown 1000:1000 /home/appuser/app
USER 1000:1000
WORKDIR /home/appuser/app

# 1a: Prepare for static linking

# 1b: Download and compile Rust dependencies (and store as a separate Docker layer)
COPY --chown=appuser Cargo.lock Cargo.lock
COPY --chown=appuser Cargo.toml Cargo.toml
COPY --chown=appuser foruma-web/Cargo.toml foruma-web/Cargo.toml
COPY --chown=appuser .docker/main.rs foruma-web/src/main.rs
COPY --chown=appuser actix-cors/Cargo.toml actix-cors/Cargo.toml
COPY --chown=appuser .docker/lib.rs actix-cors/src/lib.rs
COPY --chown=appuser tracing-actix-web/Cargo.toml tracing-actix-web/Cargo.toml
COPY --chown=appuser .docker/lib.rs tracing-actix-web/src/lib.rs
#RUN cargo build --release && \
#    rm -rf foruma-web/ && \
#    rm -rf actix-cors/ && \
#    rm -rf tracing-actix-web/

# 1c: Build the binary using the actual source code
COPY --chown=appuser foruma-web/ foruma-web/
COPY --chown=appuser actix-cors/ actix-cors/
COPY --chown=appuser tracing-actix-web/ tracing-actix-web/
COPY --chown=appuser sqlx-data.json sqlx-data.json
ENV SQLX_OFFLINE=true
RUN cargo build --release

# 2: Copy the exe and extra files to an empty Docker image
FROM debian:buster-slim
RUN groupadd --gid 1000 appuser && \
    useradd --uid 1000 --gid 1000 --shell /bin/sh appuser && \
    mkdir -p /home/appuser/app && \
    chown 1000:1000 /home/appuser/app
USER 1000:1000
COPY --chown=appuser entrypoint.sh entrypoint.sh
COPY --chown=appuser --from=builder /home/appuser/app/target/release/foruma-web /home/appuser/foruma-web
COPY --chown=appuser migrations/ migrations/
ENTRYPOINT ["./entrypoint.sh"]
