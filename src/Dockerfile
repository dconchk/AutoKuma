ARG FEATURES=

FROM rust:1.89 AS builder
ARG FEATURES
ARG TARGETARCH
WORKDIR /usr/src/autokuma
COPY . .
RUN --mount=type=cache,target=/cache/$TARGETARCH,id=cache-ghcr.io/bigboot/autokuma-${FEATURES} \
    cargo install --features "${FEATURES}" --locked --target-dir /cache/$TARGETARCH --path ./autokuma && \
    cargo install --features "${FEATURES}" --locked --target-dir /cache/$TARGETARCH --path ./kuma-cli
 
FROM gcr.io/distroless/cc-debian13:debug
COPY --from=builder /usr/local/cargo/bin/autokuma /usr/local/bin/autokuma
COPY --from=builder /usr/local/cargo/bin/kuma /usr/local/bin/kuma

ENV AUTOKUMA_DOCKER=1
HEALTHCHECK --interval=30s --timeout=5s --start-period=30s --retries=3 \
    CMD ["/busybox/wget", "-qO/dev/null", "http://localhost:8090/health"]
ENTRYPOINT ["/usr/local/bin/autokuma"]