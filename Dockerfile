FROM node:latest
WORKDIR /frontend/
COPY ./frontend/** .
RUN npm install
RUN npm run build

FROM rust:latest
WORKDIR /backend/
COPY ./backend/** .
RUN cargo build --release
CMD ["./target/release/rust-monorepo"]
