ARG RUST_VERSION=1.96.0
FROM docker.io/library/rust:${RUST_VERSION}-bookworm AS build

WORKDIR /src/sagnir
COPY . .
RUN cargo build --locked --release -p sagnir-cli --bin saga

FROM docker.io/library/debian:stable-slim
LABEL org.opencontainers.image.title="Sagnir"
LABEL org.opencontainers.image.licenses="EUPL-1.2"

RUN useradd --system --create-home --uid 10001 sagnir
COPY --from=build /src/sagnir/target/release/saga /usr/local/bin/saga
USER 10001:10001
WORKDIR /home/sagnir
ENTRYPOINT ["saga"]
