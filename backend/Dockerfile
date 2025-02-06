FROM rust:1.84 AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path . --debug

FROM debian:bookworm-slim
RUN apt-get update
RUN apt-get install -y chromium libssl3 libssl-dev
RUN apt-get clean
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend

EXPOSE 8080

CMD ["backend"]
