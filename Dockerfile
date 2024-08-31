FROM ubuntu:latest
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
RUN cargo install wasm-pack 
RUN sh ./build.sh