FROM rust:slim-buster as build

WORKDIR /webapp
COPY ./webapp ./

RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /webapp/target/release/webapp .
CMD ["./webapp"]

