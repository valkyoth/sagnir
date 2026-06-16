ARG RUST_VERSION=1.96.0
FROM docker.io/library/rust:${RUST_VERSION}-bookworm@sha256:19817ead3289c8c631c73df281e18b59b172f6a31f4f563290f69cddd06c30e9 AS build

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
