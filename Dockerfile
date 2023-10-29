FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /ticket_toy_server

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /ticket_toy_server/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin ticket_toy_server

FROM alpine AS runtime
RUN addgroup -S myunghun && adduser -S myunghun -G myunghun
COPY --from=builder /ticket_toy_server/target/x86_64-unknown-linux-musl/release/ticket_toy_server /usr/local/bin/
USER myunghun
CMD ["/usr/local/bin/ticket_toy_server"]