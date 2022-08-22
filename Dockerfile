FROM ekidd/rust-musl-builder:latest AS builder
COPY --chown=rust:rust . ./
RUN cargo build --release

FROM scratch
WORKDIR /facemash-backend
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/facemash-backend ./
COPY .env ./.env
EXPOSE 8080
CMD ["./facemash-backend"]
