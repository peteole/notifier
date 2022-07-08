
# FROM alpine:latest
# WORKDIR /app
# COPY target/release/notifier /app/notifier
# RUN pwd
# RUN ls -l
# CMD ["/app/notifier"]
# 1. This tells docker to use the Rust official image
# FROM rust:latest

# # 2. Copy the files in your machine to the Docker image
# COPY ./ ./

# # Build your program for release
# RUN SQLX_OFFLINE=true cargo build --release

# # Run the binary
# CMD ["./target/release/holodeck"]

# FROM rust:latest as builder
# WORKDIR /usr/src/myapp
# COPY . .
# RUN SQLX_OFFLINE=true cargo install --path .

# FROM debian:buster-slim
# #RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
# CMD ["myapp"]



FROM rust:latest as build-env
WORKDIR /app
COPY . /app
RUN SQLX_OFFLINE=true cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/notifier /
CMD ["./notifier"]



# FROM alpine:latest
# WORKDIR /app
# COPY target/x86_64-unknown-linux-musl/release/notifier /app/notifier
# CMD ["/app/notifier"]