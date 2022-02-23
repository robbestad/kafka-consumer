FROM rust:latest AS builder
WORKDIR /app
RUN apt update && apt install -y cmake 
RUN update-ca-certificates

# CREATE UNPRIVILEGED USER 
ENV USER=app
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# BUILD APP
COPY . .
RUN cargo build --release

# BUILD FINAL IMAGE 
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /app/target/release/kafkaconsumer .
USER app:app

