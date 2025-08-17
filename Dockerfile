FROM rust:1.89 AS builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:trixie-slim AS runtime
COPY --from=builder /app/target/release/pickeat-server /usr/local/bin/
ENTRYPOINT [ "/usr/local/bin/pickeat-server" ]
