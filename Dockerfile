# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen git
    # protoc openssl-dev protobuf-dev gcc git g++ libc-dev make binaryen

RUN npm install -g sass

RUN cargo install cargo-leptos

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .
RUN git submodule init && git submodule update

WORKDIR /work/hes-game

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine AS runner

WORKDIR /app

COPY --from=builder /work/hes-game/target/release/hes-game /app/
COPY --from=builder /work/hes-game/target/site /app/site
COPY --from=builder /work/hes-game/Cargo.toml /app/

EXPOSE 8888
ENV LEPTOS_SITE_ADDR="0.0.0.0:8888"
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/hes-game"]
