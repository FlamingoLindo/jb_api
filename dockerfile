FROM rust:1.90.0

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["jb_api"]
