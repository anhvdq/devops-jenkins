ARG RUST_VERSION=1.88
ARG APP_NAME=web-service
FROM rust:${RUST_VERSION}-slim-bookworm AS builder
ARG RUST_VERSION
ARG APP_NAME
WORKDIR /app
COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN apt update && \
    apt install -y pkg-config libssl-dev
RUN \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/${APP_NAME} /

FROM gcr.io/distroless/cc-debian12 AS final
ARG APP_NAME
COPY --from=builder /${APP_NAME} /usr/local/bin/${APP_NAME}
WORKDIR /opt/${APP_NAME}
USER nonroot:nonroot
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/web-service"]
