FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /usr/src/app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM debian:trixie-slim AS runtime
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/deardisplay-server /usr/local/bin
ENTRYPOINT ["/usr/local/bin/deardisplay-server"]
