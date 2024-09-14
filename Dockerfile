FROM rust:1.81.0 as build

WORKDIR /webapp
COPY ./webapp ./

RUN cargo build --release

FROM rust:1.81.0
COPY --from=build /webapp/target/release/webapp .
CMD ["./webapp"]

