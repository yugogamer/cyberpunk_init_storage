FROM rust:alpine3.17 AS builder
RUN apk add --no-cache musl-dev pkgconfig openssl-dev
WORKDIR /usr/src/app
COPY . .
RUN SQLX_OFFLINE=true cargo build --release

FROM alpine:3.17
COPY --from=builder /usr/src/app/target/release/cyberpunk_init_storage /usr/local/bin/cyberpunk_init_storage
EXPOSE 8080
ENTRYPOINT ["cyberpunk_init_storage"]