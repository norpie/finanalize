FROM rust:1.84

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["backend"]
