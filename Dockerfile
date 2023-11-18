FROM rust:1.74-bookworm as builder

# TODO some form of cache lmao
COPY ./src /build/src
COPY ./Cargo.* /build

WORKDIR /build

RUN cargo build


FROM debian:latest as runner

# COPY only the executable
COPY --from=builder /build/target/debug/season-link-profiles /opt

CMD [ "/opt/season-link-profiles" ]