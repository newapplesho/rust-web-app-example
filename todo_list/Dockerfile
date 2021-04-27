FROM rust:1.51 AS builder

WORKDIR /todo_list

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY ./infrastructure ./infrastructure
COPY ./domain ./domain
COPY ./service ./service
COPY ./api ./api

RUN cargo build
# 本番環境へデプロイするときは以下の release buildを用いる方がよい
# RUN cargo build --release

# for release
FROM ubuntu:20.04
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /todo_list/target/debug/api /usr/local/bin/api
# release buildの場合は以下を用いる
# COPY --from=builder /todo_list/target/release/api /usr/local/bin/api

CMD ["/usr/local/bin/api"]
