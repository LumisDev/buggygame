FROM rust:1.67
RUN cargo install wasm-pack
FROM debian:bullseye-slim
WORKDIR /usr/src/myapp
COPY . .
CMD ["sh", "./build.sh"]

