ARG RUST_VERSION=1.97.0
FROM docker.io/library/rust:${RUST_VERSION}-bookworm@sha256:7d0723df719e7f213b69dc7c8c595985c3f4b060cfbee4f7bc0e347a86fe3b6a AS build

WORKDIR /src/sagnir
COPY . .
RUN cargo build --locked --release -p sagnir-cli --bin saga

FROM docker.io/library/debian:stable-slim@sha256:34363c20bd149e41365fc77b086da067ed13ab2dff4cd0612788e12e6d52c44c
LABEL org.opencontainers.image.title="Sagnir"
LABEL org.opencontainers.image.licenses="EUPL-1.2"

RUN useradd --system --create-home --uid 10001 sagnir
COPY --from=build /src/sagnir/target/release/saga /usr/local/bin/saga
USER 10001:10001
WORKDIR /home/sagnir
ENTRYPOINT ["saga"]
