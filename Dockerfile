FROM rust:1.78-alpine3.20 as build
COPY Cargo.toml /
COPY src /
RUN cargo build 
FROM alpine:3.20
COPY --from=build /target/debug/roadwork_server /
EXPOSE 8080
CMD ["/roadwork_server"]