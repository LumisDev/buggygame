FROM ubuntu:latest
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && cargo install wasm-pack && sh ./build.sh