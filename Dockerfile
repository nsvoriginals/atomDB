# Builder stage
FROM rust:1.75-alpine as builder

RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

COPY ./src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:latest
RUN apk add --no-cache ca-certificates
RUN adduser -D -s /bin/sh atomuser

WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/atom ./atomdb
RUN chown atomuser:atomuser ./atomdb

USER atomuser
EXPOSE 4000
CMD ["./atomdb", "--server"]
