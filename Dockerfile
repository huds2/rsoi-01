FROM rust:1.81.0

COPY ./webapp ./

RUN cargo build --release

CMD ["./target/release/webapp"]
