FROM --platform=$BUILDPLATFORM rust:1.89 AS builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM
RUN case "$TARGETPLATFORM" in \
    "linux/arm64") \
        rustup target add aarch64-unknown-linux-gnu; \
        apt update && apt install -y gcc gcc-aarch64-linux-gnu libc6-dev-arm64-cross && rm -rf /var/lib/apt/lists/*; \
        export CARGO_BUILD_TARGET=aarch64-unknown-linux-gnu; \
        export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc; \
        ;; \
    "linux/amd64") \
        # Nothing to do
        ;; \
    *) exit 1;; \
esac
ENV SQLX_OFFLINE=true
WORKDIR /app
COPY \
    Cargo.toml Cargo.lock \
    .
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo fetch --locked
COPY \
    src db web tests \
    .
RUN cargo build --locked --release
RUN cargo install --locked --path . --root /out

FROM debian:trixie-slim AS runtime
COPY --from=builder /out/bin/pickeat-server /usr/local/bin/
ENTRYPOINT [ "/usr/local/bin/pickeat-server" ]
