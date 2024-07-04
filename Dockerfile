FROM rust:1.79-alpine3.20 as build
COPY Cargo.toml /Cargo.toml
COPY src /src
RUN apk update && apk add --no-cache musl-dev
RUN --mount=type=cache,target=/usr/local/cargo/registry \
        cargo build --release
FROM alpine:3.20.1
COPY --from=build /target/release/roadwork_server /
RUN adduser -D roadwork-server
USER roadwork-server
EXPOSE 8080
CMD ["./roadwork_server"]