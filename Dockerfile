FROM rust:slim-bullseye AS build

RUN apt-get update && \
    apt-get install -y build-essential clang && \
    rustup component add rustfmt

WORKDIR /app

COPY . /app

RUN cargo clean && cargo build --release && \
    strip ./target/release/zyst

FROM gcr.io/distroless/cc

WORKDIR /usr/src/zyst
COPY --from=build /app/target/release/zyst /usr/local/bin/zyst

CMD [ "zyst", "--bind", "0.0.0.0"]

EXPOSE 6379
