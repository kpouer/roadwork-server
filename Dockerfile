FROM rust:1.89-alpine as build
COPY Cargo.toml /Cargo.toml
COPY src /src
RUN apk update && apk add --no-cache musl-dev
RUN --mount=type=cache,target=/usr/local/cargo/registry \
        cargo build --release
FROM alpine:latest
COPY --from=build /target/release/roadwork_server /
RUN adduser -D roadwork-server
USER roadwork-server
EXPOSE 8080
CMD ["./roadwork_server"]