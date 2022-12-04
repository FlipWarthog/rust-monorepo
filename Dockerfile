# 1. This tells docker to use the Rust official image
FROM rust:latest

# 2. Copy the files in your machine to the Docker image
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./.env ./.env

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/rust-monorepo"]
