FROM rust:1.65.0

COPY ./frontend/dist ./frontend/dist
COPY ./backend ./backend

WORKDIR /backend

RUN cargo build --release
RUN cp ./target/release/rust-monorepo .
RUN rm -rf ./target

CMD ["./rust-monorepo"]
