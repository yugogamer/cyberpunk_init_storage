FROM rust:buster AS builder
WORKDIR /usr/src/app
COPY . .
EXPOSE 8080
RUN SQLX_OFFLINE=true cargo build --release
#ENTRYPOINT [ "cargo", "run", "--release" ]

FROM debian:bullseye-slim 
COPY --from=builder /usr/src/app/target/release/cyberpunk_init_storage /app/cyberpunk_init_storage
COPY ./migrations /app/migrations
RUN apt-get update && apt-get install -y ca-certificates openssl
RUN update-ca-certificates
EXPOSE 8080
WORKDIR /app
RUN chmod +x cyberpunk_init_storage
RUN ls -la
RUN ls -la migrations
ENTRYPOINT ["./cyberpunk_init_storage"]