FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/server /server
USER nonroot
CMD ["/server"]
