FROM rust:1.85 as builder

WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt update && apt install -y podman && apt clean all

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/pod-station .

EXPOSE 8080
CMD ["./pod-station"]
