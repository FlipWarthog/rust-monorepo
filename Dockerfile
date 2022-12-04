# 1. This tells docker to use the Rust official image
FROM rust:latest

# 2. Copy the files in your machine to the Docker image
COPY ./backend ./backend

# Build your program for release
RUN cargo build --release --manifest-file ./backend/Cargo.toml

# Run the binary
CMD ["./backend/target/release/rust-monorepo"]
