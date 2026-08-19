ARG RUST_VERSION=1.96
FROM rust:${RUST_VERSION}-slim-bullseye as build

COPY . .

RUN apt update \
    && apt install --yes binutils build-essential pkg-config libssl-dev clang lld git protobuf-compiler \
    && rm -rf /var/lib/{apt,dpkg,cache,log}

RUN cargo build --release --features "db_pgsql"

FROM debian:bullseye-slim

RUN apt update \
    && apt install --yes ca-certificates gettext-base libssl1.1 --no-install-recommends \
    && rm -rf /var/lib/{apt,dpkg,cache,log}

COPY --from=build "/target/release/platypus" "/bin/platypus"
COPY --from=build "/tls" "/etc/tls"

ENV RUST_LOG info
ENV DB_HOST "postgres://db_user:Platypus2025!@172.17.0.1/platypus"
ENV TLS_CERT "/etc/tls/cert.pem"
ENV TLS_KEY "/etc/tls/key.pem"

EXPOSE 8001

CMD ["/bin/platypus"]
