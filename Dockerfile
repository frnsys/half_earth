# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen git gcc g++ boost-dev boost-libs
    # protoc openssl-dev protobuf-dev libc-dev make binaryen

RUN npm install -g sass

RUN cargo install cargo-leptos

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .
RUN rm -rf /work/hector-rs && git clone https://github.com/frnsys/hector-rs.git /work/hector-rs

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
